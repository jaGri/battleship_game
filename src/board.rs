use std::fmt;

use crate::constants::PlayerState;
use crate::constants::GuessError;
use crate::constants::GameplayError;
use crate::constants::GameState;
use crate::probability::calc_pdf_and_guess;
use crate::ship::Ship;
use rand::{seq::IteratorRandom, Rng, thread_rng};

use crate::constants::GuessResult;

use crate::constants::Cell;

use crate::fleet::Fleet;

use std::collections::HashSet;
use crate::constants::GRID_SIZE;


// Represents the game board for Battleship, managing ship placement, guessing,
// and game state tracking.
pub struct Board {
    /// Size of the grid (typically 10x10)
    gridsize: usize,
    /// Collection of ships on the board
    fleet: Fleet,
    /// Set of all valid coordinates on the board
    coordinates: HashSet<(usize, usize)>,
    /// Set of coordinates that have been guessed
    guessed: HashSet<(usize, usize)>,
}

// /// Represents the current state of a board, including ship positions,
// /// guesses, hits, misses, and game state information.
// pub struct BoardState {
//     /// Set of coordinates where ships are located
//     ships: HashSet<(usize, usize)>,
//     /// Set of all guessed coordinates
//     guesses: HashSet<(usize, usize)>,
//     /// Set of successful hits
//     hits: HashSet<(usize, usize)>,
//     /// Set of missed shots
//     misses: HashSet<(usize, usize)>,
//     /// Vector of sunk ships with their names and lengths
//     sunk: Vec<(&'static str, usize)>,
//     /// Vector of ships still afloat with their names and lengths
//     unsunk: Vec<(&'static str, usize)>,
//     /// Current state of the player (Setup, Alive, or Dead)
//     state: PlayerState
// }


// impl BoardState {
//     pub fn new(board: &Board, incl_ships:bool) -> Self {
//         Self {
//             ships: board.ship_coords(incl_ships, incl_ships),
//             guesses: board.guessed().clone(),
//             hits: board.hit_coords(true, true),
//             misses: board.miss_coords(),
//             sunk: board.fleet.get_ship_names_and_length(false, true).collect(),
//             unsunk: board.fleet.get_ship_names_and_length(true, false).collect(),
//             state: board.player_state()
//         }
//     }
// }

impl Board {
    /// Creates a new empty game board with initialized coordinate space
    /// and empty fleet.
    ///
    /// # Returns
    /// * `Board` - A new board instance ready for ship placement
    ///
    /// # Example
    /// ```
    /// let mut board = Board::new();
    /// assert_eq!(board.player_state(), PlayerState::Setup);
    /// ```
    pub fn new() -> Self {
        let coordinates: HashSet<(usize, usize)> = (0..GRID_SIZE)
            .flat_map(|row| (0..GRID_SIZE).map(move |col| (row, col)))
            .collect();
        Self {
            gridsize: GRID_SIZE,
            fleet: Fleet::new(),
            coordinates: coordinates,
            guessed: HashSet::with_capacity(GRID_SIZE*GRID_SIZE),
        }
    }

    /// Returns a reference to the set of guessed coordinates.
    ///
    /// # Returns
    /// * `&HashSet<(usize,usize)>` - Reference to the set of guessed coordinates
    pub fn guessed(&self) -> &HashSet<(usize,usize)> {
        &self.guessed
    }

    /// Returns a set of coordinates that haven't been guessed yet.
    ///
    /// # Returns
    /// * `HashSet<(usize,usize)>` - Set of unguessed coordinates
    pub fn unguessed(&self) -> HashSet<(usize,usize)> {
        self.coordinates
            .difference(&self.guessed)
            .cloned()
            .collect()
    }

    /// Returns coordinates where ships have been hit.
    ///
    /// # Arguments
    /// * `unsunk` - Include hits on ships that haven't been sunk
    /// * `sunk` - Include hits on ships that have been sunk
    ///
    /// # Returns
    /// * `HashSet<(usize,usize)>` - Set of hit coordinates matching the criteria
    pub fn hit_coords(&self, unsunk:bool, sunk: bool) -> HashSet<(usize,usize)> {
        self.fleet.hit_coords(unsunk, sunk)
    }

    /// Returns coordinates of missed shots.
    ///
    /// # Returns
    /// * `HashSet<(usize,usize)>` - Set of coordinates where shots missed
    pub fn miss_coords(&self) -> HashSet<(usize,usize)> {
        self.guessed.difference(&self.hit_coords(true, true)).cloned().collect()
    }

    fn ship_coords(&self, unsunk:bool, sunk:bool) -> HashSet<(usize,usize)> {
        self.fleet.ship_coords(unsunk, sunk)
    }

    /// Calculates the coordinates a ship would occupy given a starting position,
    /// length, and orientation.
    ///
    /// # Arguments
    /// * `start` - Starting coordinate (row, col)
    /// * `length` - Length of the ship
    /// * `horizontal` - If true, ship extends horizontally; if false, vertically
    ///
    /// # Returns
    /// * `HashSet<(usize,usize)>` - Set of coordinates the ship would occupy
    pub fn calc_placement(&self, start: (usize,usize), length: usize, horizontal: bool) -> HashSet<(usize,usize)> {
        (0..length)
            .map(|i| {
                if horizontal {
                    (start.0, start.1 + i)
                } else {
                    (start.0 + i, start.1)
                }
            })
        .collect()
    }

    /// Validates whether a ship placement is legal.
    ///
    /// # Arguments
    /// * `coords` - Set of coordinates where ship would be placed
    /// * `invalid_coords` - Set of coordinates that can't be used (e.g., other ships)
    ///
    /// # Returns
    /// * `bool` - True if placement is valid, false otherwise
    pub fn valid_placement(&self, coords: &HashSet<(usize, usize)>, invalid_coords: &HashSet<(usize, usize)>) -> bool {
        coords.is_subset(&self.coordinates) && coords.is_disjoint(&invalid_coords)
    }

    /// Attempts to place a ship on the board.
    ///
    /// # Arguments
    /// * `name` - Name of the ship to place
    /// * `start` - Starting coordinate (row, col)
    /// * `horizontal` - If true, ship extends horizontally; if false, vertically
    ///
    /// # Returns
    /// * `Result<(), GameplayError>` - Ok(()) if successful, Error if placement invalid
    ///
    /// # Example
    /// ```
    /// let mut board = Board::new();
    /// let result = board.place_ship("Carrier", (0, 0), true);
    /// assert!(result.is_ok());
    /// ```
    pub fn place_ship(&mut self, name: &str, start: (usize, usize), horizontal: bool) -> Result<(), GameplayError> {
        let existing_ships: HashSet<(usize, usize)> = self.ship_coords(true, true);
        let length = match self.fleet.get_ship(name) {
            Ok(ship) => ship.length(),
            Err(e) => return Err(e),
        };
        let proposed = self.calc_placement(start, length, horizontal);
        if !self.valid_placement(&proposed, &existing_ships) {
            return Err(GameplayError::InvalidPlacement);
        }
        self.fleet.place_ship(name, proposed)
    }


    pub fn randomly_place_ship(&mut self, name: &str) -> Result<(), GameplayError> {
        let mut rng = thread_rng();
        for _ in 0..GRID_SIZE*GRID_SIZE*1000 { // Prevent infinite loop
            if let Some(start) = self.coordinates.iter().choose(&mut rng) {
                let horizontal = rng.gen_bool(0.5);
                if self.place_ship(name, *start, horizontal).is_ok() {
                    return Ok(());
                }
            }
        }
        Err(GameplayError::CantFindValidPlacement)
    }

    pub fn randomly_place_fleet(&mut self) -> Result<(), GameplayError> {
        // Collect the names of the unplaced ships into a vector
        let unplaced_ships: Vec<String> = self.fleet.unplaced_ships()
            .map(|ship| ship.name().to_string())
            .collect();
        // Now place each ship
        for ship_name in unplaced_ships {
            match self.randomly_place_ship(&ship_name) {
                Ok(_) => continue,
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }

    /// Makes a guess at the given coordinates.
    ///
    /// # Arguments
    /// * `target` - Coordinate to attack (row, col)
    ///
    /// # Returns
    /// * `Result<GuessResult, GuessError>` - Result of the guess or error if invalid
    ///
    /// # Example
    /// ```
    /// let mut board = Board::new();
    /// board.place_ship("Carrier", (0, 0), true);
    /// let result = board.guess((0, 0));
    /// assert_eq!(result.unwrap(), GuessResult::Hit);
    /// ```
    pub fn guess(&mut self, target: (usize, usize)) -> Result<GuessResult, GuessError> {
        if !self.is_valid_target(target) {
            return Err(GuessError::InvalidTarget);
        }
        if !self.guessed.insert(target) {
            return Err(GuessError::AlreadyGuessed);
        }
        self.fleet.guess(target)
    }
    
    fn is_valid_target(&self, target: (usize, usize)) -> bool {
        self.coordinates.contains(&target)
    }

    /// Makes a random guess on the board
    pub fn random_guess(&mut self) -> Result<GuessResult, GuessError> {
        let mut rng = thread_rng();
        let unguessed = self.unguessed();
        if unguessed.len()==0 {
            return Err(GuessError::NoValidCoordinates);
        }
        match unguessed.iter().choose(&mut rng) {
            Some(guess) => self.guess(*guess),
            None => Err(GuessError::RandomGuessFailed)
        }
    }

    /// Makes an educated guess using probability density calculation.
    ///
    /// This method uses the probability.rs module to calculate the most likely
    /// locations of ships based on current board state and makes an intelligent guess.
    ///
    /// # Returns
    /// * `Result<GuessResult, GuessError>` - Result of the guess or error if invalid
    pub fn educated_guess(&mut self) -> Result<GuessResult, GuessError> {
        let guess = calc_pdf_and_guess(&self);
        self.guess(guess)
    }

    /// Gets the current state of the player (Setup, Alive, or Dead).
    ///
    /// # Returns
    /// * `PlayerState` - Current state of the player
    pub fn player_state(&self) -> PlayerState {
        if self.fleet.unplaced_ships().count() > 0 {
            PlayerState::Setup
        } else if self.fleet.n_ships(true, false) > 0 {
            PlayerState::Alive
        } else {
            PlayerState::Dead
        }
    }

    pub fn share(&self) {
        let hits = self.hit_coords(true, true);
        let misses = self.miss_coords();

    }

    pub fn get_ships(&self, unsunk:bool, sunk: bool) -> impl Iterator<Item = &Ship> {
        self.fleet.get_ships(unsunk, sunk)
    }

    pub fn hits_remaining(&self) -> usize {
        self.fleet.hits_remaining() 
    }

    pub fn ship_lengths_remaining(&self) -> Vec<usize> {
        self.get_ships(true, false).map(|s| s.length()).collect()
    }

    pub fn unguessed_iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.coordinates.difference(&self.guessed)
    }

    // pub fn print_grid(&self) {
    //     let ships = self.fleet.ship_coords(true, true);
    //     let hits = self.fleet.hit_coords(true, true);
    //     // Print header
    //     print!("   "); // Initial space for row labels
    //     for col in 1..=self.gridsize {
    //         print!(" {} ", col);
    //     }
    //     println!();
    //     // Print rows
    //     for row in 0..self.gridsize {
    //         // Print row letter
    //         print!("{} ", (b'A' + row as u8) as char);
    //         for col in 0..self.gridsize {
    //             let coord = (row, col);
    //             let icon = if self.guessed.contains(&coord) {
    //                 if hits.contains(&coord) {
    //                     Cell::Hit.icon()
    //                 } else {
    //                     Cell::Miss.icon()
    //                 }
    //             } else if ships.contains(&coord) {
    //                 Cell::Ship.icon()
    //             } else {
    //                 Cell::Empty.icon()
    //             };
    //             print!(" {} ", icon);
    //         }
    //         println!();
    //     }
    // }
    
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ships = self.fleet.ship_coords(true, true);
        let hits = self.fleet.hit_coords(true, true);

        // Print header
        write!(f, "   ")?; // Initial space for row labels
        for col in 1..=self.gridsize {
            write!(f, " {} ", col)?;
        }
        writeln!(f)?;

        // Print rows
        for row in 0..self.gridsize {
            // Print row letter
            write!(f, "{} ", (b'A' + row as u8) as char)?;
            for col in 0..self.gridsize {
                let coord = (row, col);
                let icon = if self.guessed.contains(&coord) {
                    if hits.contains(&coord) {
                        Cell::Hit.icon()
                    } else {
                        Cell::Miss.icon()
                    }
                } else if ships.contains(&coord) {
                    Cell::Ship.icon()
                } else {
                    Cell::Empty.icon()
                };
                write!(f, " {} ", icon)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

