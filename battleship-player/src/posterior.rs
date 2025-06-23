//! Exact Bayesian posterior for Battleship—bitmask-based, fully joint over all remaining ships.
//!
//! This module computes the true posterior probability that each cell contains part of any
//! unsunk ship, given a board of observed hits and misses.  It uses:
//! 1. **Bitmask representations** (`u128`) for fast overlap/coverage checks on a 10×10 grid.
//! 2. **Pruning** via a precomputed "future union" mask to eliminate branches that can't explain all hits.
//! 3. **Memoization** of subtrees to avoid re-exploring identical partial fleets.
//! 4. **Parallelism** (Rayon) over the first ship's placements for multicore speed.
//!
//! # Usage
//! ```rust
//!
//! // Example: one ship length 2, a single hit at (0,0), no misses.
//! let post = Posterior::new(&[], &[(0,0)], &[2]);
//! let heatmap = post.compute();  // [[f64;10];10] sums to 1.0
//! ```

use rayon::prelude::*;
use std::collections::HashMap;
use battleship_config::GRID_SIZE;
type Mask = u128;  // 100 bits → 10×10 grid

/// Packs a list of (row, col) coordinates into a single `Mask`,
/// with bit (r*10 + c) set to 1 for each occupied cell.
fn coords_to_mask(coords: &[(usize, usize)]) -> Mask {
    coords.iter()
          .fold(0, |mask, &(r, c)| mask | (1 << (r * GRID_SIZE + c)))
}

/// Generate every possible placement of a ship of length `length`, excluding any which
/// overlap the `exclude_mask` (misses or sunk cells).
fn gen_placements(exclude_mask: Mask, length: usize) -> Vec<Mask> {
    let mut out = Vec::new();

    // Horizontal placements
    for r in 0..GRID_SIZE {
        for c in 0..=(GRID_SIZE - length) {
            let mut m = 0;
            for k in 0..length {
                m |= 1 << (r * GRID_SIZE + c + k);
            }
            if m & exclude_mask == 0 {
                out.push(m);
            }
        }
    }

    // Vertical placements
    for c in 0..GRID_SIZE {
        for r in 0..=(GRID_SIZE - length) {
            let mut m = 0;
            for k in 0..length {
                m |= 1 << ((r + k) * GRID_SIZE + c);
            }
            if m & exclude_mask == 0 {
                out.push(m);
            }
        }
    }

    out
}

/// Main struct for computing the exact posterior.
pub struct Posterior {
    miss_mask: Mask,
    hit_mask: Mask,
    placements: Vec<Vec<Mask>>,  // All valid placements for each remaining ship
    future_union: Vec<Mask>,     // Pruning masks: union of placements[depth..]
}

impl Posterior {
    /// Construct a new Posterior calculator.
    ///
    /// # Arguments
    /// - `misses`: list of observed miss or sunk-cell coordinates
    /// - `hits`:   list of observed unsunk-hit coordinates
    /// - `unsunk_ship_lengths`: lengths of all ships not yet sunk (e.g., `[5,4,3,3,2]`)
    pub fn new(
        misses: &[(usize, usize)],
        hits: &[(usize, usize)],
        unsunk_ship_lengths: &[usize],
    ) -> Self {
        let miss_mask = coords_to_mask(misses);
        let hit_mask  = coords_to_mask(hits);

        let mut placements: Vec<Vec<Mask>> = unsunk_ship_lengths
            .iter()
            .map(|&len| gen_placements(miss_mask, len))
            .collect();

        let mut zipped: Vec<_> = unsunk_ship_lengths
            .iter()
            .cloned()
            .zip(placements.into_iter())
            .collect();
        zipped.sort_by_key(|(_, p)| p.len());
        placements = zipped.into_iter().map(|(_, p)| p).collect();

        let n = placements.len();
        let mut future_union = vec![0; n + 1];
        for d in (0..n).rev() {
            let u = placements[d].iter().fold(future_union[d + 1], |acc, &m| acc | m);
            future_union[d] = u;
        }

        Posterior { miss_mask, hit_mask, placements, future_union }
    }

    /// Compute the 10×10 posterior heatmap `[[f64;10];10]` summing to 1.0.
    pub fn compute(&self) -> [[f64; GRID_SIZE]; GRID_SIZE] {
        let ship_count = self.placements.len();
        let mut total_weight = 0f64;
        let mut cell_counts = vec![0f64; GRID_SIZE * GRID_SIZE];

        fn backtrack(
            depth: usize,
            used: Mask,
            cfg: &Posterior,
            counts: &mut [f64],
            weight: &mut f64,
            memo: &mut HashMap<(usize, Mask), (f64, Vec<f64>)>,
        ) {
            if (used | cfg.future_union[depth]) & cfg.hit_mask != cfg.hit_mask {
                return;
            }

            if depth == cfg.placements.len() {
                if used & cfg.hit_mask == cfg.hit_mask {
                    *weight += 1.0;
                    for bit in 0..(GRID_SIZE * GRID_SIZE) {
                        if (used >> bit) & 1 == 1 {
                            counts[bit] += 1.0;
                        }
                    }
                }
                return;
            }

            let key = (depth, used);
            if let Some(&(w, ref subtotal)) = memo.get(&key) {
                *weight += w;
                for i in 0..subtotal.len() {
                    counts[i] += subtotal[i];
                }
                return;
            }

            let mut local_weight = 0f64;
            let mut local_counts = vec![0f64; GRID_SIZE * GRID_SIZE];

            for &placement_mask in &cfg.placements[depth] {
                if used & placement_mask != 0 { continue; }
                backtrack(
                    depth + 1,
                    used | placement_mask,
                    cfg,
                    &mut local_counts,
                    &mut local_weight,
                    memo,
                );
            }

            memo.insert(key, (local_weight, local_counts.clone()));
            *weight += local_weight;
            for i in 0..local_counts.len() {
                counts[i] += local_counts[i];
            }
        }

        if ship_count == 0 {
            return [[0.0; GRID_SIZE]; GRID_SIZE];
        }

        let first_ship_placements = &self.placements[0];
        let partials: Vec<(Vec<f64>, f64)> = first_ship_placements
            .par_iter()
            .map(|&first_mask| {
                let mut counts = vec![0f64; GRID_SIZE * GRID_SIZE];
                let mut weight = 0f64;
                let mut memo = HashMap::new();

                if self.hit_mask == 0 || (first_mask & self.hit_mask != 0) {
                    backtrack(1, first_mask, self, &mut counts, &mut weight, &mut memo);
                }

                (counts, weight)
            })
            .collect();

        for (counts, w) in partials {
            total_weight += w;
            for i in 0..counts.len() {
                cell_counts[i] += counts[i];
            }
        }

        let mut heatmap = [[0.0; GRID_SIZE]; GRID_SIZE];
        if total_weight > 0.0 {
            for bit in 0..(GRID_SIZE * GRID_SIZE) {
                let p = cell_counts[bit] / total_weight;
                let r = bit / GRID_SIZE;
                let c = bit % GRID_SIZE;
                heatmap[r][c] = p;
            }
        }
        heatmap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool { (a - b).abs() < 1e-8 }

    #[test]
    fn test_single_length1_uniform() {
        let post = Posterior::new(&[], &[], &[1]);
        let pm = post.compute();
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                assert!(approx_eq(pm[r][c], 1.0 / 100.0));
            }
        }
    }

    #[test]
    fn test_length2_with_one_hit() {
        let post = Posterior::new(&[], &[(0, 0)], &[2]);
        let pm = post.compute();
        assert!(approx_eq(pm[0][0], 1.0));
        assert!(approx_eq(pm[0][1], 0.5));
        assert!(approx_eq(pm[1][0], 0.5));
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                let is_known = (r == 0 && c == 0) || (r == 0 && c == 1) || (r == 1 && c == 0);
                if !is_known {
                    assert!(approx_eq(pm[r][c], 0.0));
                }
            }
        }
    }

    #[test]
    fn test_two_length1_ships_uniform() {
        let post = Posterior::new(&[], &[], &[1, 1]);
        let pm = post.compute();
        let expected = 99.0 / 4950.0;
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                assert!(approx_eq(pm[r][c], expected));
            }
        }
    }
}

