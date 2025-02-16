use crate::board::Board;
use crate::constants::GRID_SIZE;
use itertools::Itertools;

use rand::Rng;

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

/// Calculates and displays a 2D array in a formatted way.
/// Useful for debugging and visualizing probability distributions.
///
/// # Arguments
/// * `matrix` - 2D array to display
/// * `N` - Number of rows
/// * `M` - Number of columns
///
/// # Type Parameters
/// * `T` - Type that implements Display and Copy
fn pretty_print_2d_array<T: Display + Copy, const N: usize, const M: usize>(matrix: &[[T; M]; N]) {
    for row in matrix.iter() {
        println!(
            "{}",
            row.iter()
                .map(|&elem| format_element(&elem))
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

/// Formats a single element for display in the probability matrix.
///
/// # Arguments
/// * `elem` - Element to format
///
/// # Returns
/// * `String` - Formatted string representation
fn format_element<T: Display>(elem: &T) -> String {
    if let Some(f) = elem_as_f64(elem) {
        format!("{:>8.3}", f)
    } else {
        format!("{:>8}", elem)
    }
}

/// Attempts to convert a displayable element to f64.
///
/// # Arguments
/// * `elem` - Element to convert
///
/// # Returns
/// * `Option<f64>` - Converted value if possible, None if conversion fails
fn elem_as_f64<T: Display>(elem: &T) -> Option<f64> {
    elem.to_string().parse::<f64>().ok()
}

/// Calculates the probability density function for ship locations using a Bayesian approach.
///
/// For each remaining ship length, we enumerate all candidate placements. Each placement's
/// posterior weight is computed as:
///    P(placement | observations) ∝ P(observations | placement) * P(placement)
///
/// Here we assume:
/// - A uniform prior over placements.
/// - A likelihood function defined as:
///     - If the placement conflicts with any miss/sunk cell, its likelihood is 0.
///     - If there are unsunk hit cells on board:
///         - If the placement explains at least one unsunk hit then
///             likelihood = (L_HIT)^(number of unsunk hits covered),
///         - Otherwise the placement is penalized with a low likelihood: L_NO_HIT.
///     - If there are no unsunk hits, the likelihood is 1 (all placements are equally likely).
///
/// For each candidate placement, its likelihood is added to each unguessed coordinate that is
/// part of that placement, and in the end the matrix is normalized.
///
/// # Arguments
/// * `board` - Reference to the game board
///
/// # Returns
/// * `[[f64; GRID_SIZE]; GRID_SIZE]` - 2D array of probabilities
fn calc_pdf(board: &Board) -> [[f64; GRID_SIZE]; GRID_SIZE] {
    let unguessed_coords = board.unguessed();
    let unsunk_hit_coords: HashSet<(usize, usize)> = board.hit_coords(true, false);
    let misses_and_sunk_coords = board
        .guessed()
        .difference(&unsunk_hit_coords)
        .cloned()
        .collect();
    let unsunk_ship_lengths: Vec<usize> = board.ship_lengths_remaining();

    let mut prob_matrix = [[0.0; GRID_SIZE]; GRID_SIZE];

    // Bayesian likelihood parameters
    const L_HIT: f64 = 5.0;   // For each unsunk hit cell that the placement covers, the likelihood is multiplied by this factor.
    // Range: Typically between 1.0 and 10.0
    // Lower Values (1.0 - 3.0): The AI will be more conservative, favoring placements that cover hits but still considering other placements.
    // Higher Values (4.0 - 10.0): The AI will aggressively favor placements that cover hits, potentially leading to a more focused but less exploratory strategy.
    const L_NO_HIT: f64 = 0.2; // If there is at least one unsunk hit on the board, and the candidate placement covers none, assign this small likelihood.
    // Range: Typically between 0.01 and 1.0.
    // Lower Values (0.01 - 0.1): The AI will be more forgiving of placements that do not cover hits, allowing for a broader search space.
    // Higher Values (0.2 - 1.0): The AI will heavily penalize placements that do not cover hits, which can lead to a more focused but potentially limited strategy.

    // For each remaining ship, consider all candidate placements.
    for ship_length in unsunk_ship_lengths {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let start_coord = (i, j);
                for &horizontal in &[true, false] {
                    let placement = board.calc_placement(start_coord, ship_length, horizontal);
                    // Skip placements that conflict with observed misses / sunk coordinates.
                    if !board.valid_placement(&placement, &misses_and_sunk_coords) {
                        continue;
                    }
                    
                    // Count how many unsunk hit coordinates are explained by this placement.
                    let n_hits = unsunk_hit_coords.intersection(&placement).count();
                    let likelihood = if !unsunk_hit_coords.is_empty() {
                        if n_hits > 0 {
                            // Multiply evidence: each hit multiplies the likelihood.
                            L_HIT.powi(n_hits as i32)
                        } else {
                            // Placements that do not capture any unsunk hit get penalized.
                            L_NO_HIT
                        }
                    } else {
                        // No unsunk hits on board implies no additional evidence.
                        1.0
                    };
                    
                    // Spread the candidate placement's likelihood to all cells that remain unguessed.
                    for coord in &placement {
                        if unguessed_coords.contains(&coord) {
                            prob_matrix[coord.0][coord.1] += likelihood;
                        }
                    }
                }
            }
        }
    }
    normalize_pdf(&prob_matrix)
}

/// Normalizes a probability density function matrix so all values sum to 1.
///
/// # Arguments
/// * `matrix` - Matrix to normalize
///
/// # Returns
/// * `[[f64; GRID_SIZE]; GRID_SIZE]` - Normalized probability matrix
fn normalize_pdf(matrix: &[[f64; GRID_SIZE]; GRID_SIZE]) -> [[f64; GRID_SIZE]; GRID_SIZE] {
    let sum: f64 = matrix.iter().flatten().sum();
    if sum == 0.0 {
        // Instead of returning a zeroed matrix, distribute probability uniformly.
        let uniform = 1.0 / (GRID_SIZE * GRID_SIZE) as f64;
        return [[uniform; GRID_SIZE]; GRID_SIZE];
    }
    let mut normalized_matrix = [[0.0; GRID_SIZE]; GRID_SIZE];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            normalized_matrix[i][j] = value / sum;
        }
    }
    normalized_matrix
}

/// Samples a coordinate from the probability distribution, adjusting
/// the decision-making using a temperature parameter.
///
/// A lower `temperature` (<1.0) sharpens the distribution (AI becomes greedy),
/// while a higher temperature (>1.0) flattens it (resulting in more random moves).
///
/// # Arguments
/// * `pdf` - Probability density function matrix
/// * `temperature` - Temperature factor to adjust the confidence in the moves
///
/// # Returns
/// * `(usize, usize)` - Selected coordinate
fn sample_pdf(
    pdf: &[[f64; GRID_SIZE]; GRID_SIZE],
    temperature: f64,
) -> (usize, usize) {
    // Create an adjusted matrix by applying a Boltzmann factor: p'(x) = p(x)^(1/temperature)
    let mut adjusted_matrix = [[0.0; GRID_SIZE]; GRID_SIZE];
    let mut total = 0.0;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            // When temperature == 1.0, probabilities remain unchanged.
            adjusted_matrix[i][j] = pdf[i][j].powf(1.0 / temperature);
            total += adjusted_matrix[i][j];
        }
    }

    // If total is zero, fall back to a uniform random selection from all board coordinates.
    if total == 0.0 {
        let mut rng = rand::thread_rng();
        return (rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));
    }

    // Perform cumulative sampling from the adjusted probability distribution.
    let mut rng = rand::thread_rng();
    let random_value: f64 = rng.gen_range(0.0..total);
    let mut cumulative_sum = 0.0;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            cumulative_sum += adjusted_matrix[i][j];
            if random_value < cumulative_sum {
                return (i, j);
            }
        }
    }
    // Fallback due to floating point imprecision.
    (GRID_SIZE - 1, GRID_SIZE - 1)
}

/// Calculates probabilities and makes an intelligent guess.
///
/// This is the main AI function that combines probability calculation
/// with coordinate selection for making educated guesses.
///
/// # Arguments
/// * `board` - Reference to the game board
///
/// # Returns
/// * `(usize, usize)` - Chosen coordinate for the next guess
pub fn calc_pdf_and_guess(board: &Board) -> (usize, usize) {
    let pdf = calc_pdf(board);
    sample_pdf(&pdf, 1.0)
}