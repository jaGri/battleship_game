use crate::constants::GameplayError;
use crate::constants::GameplayError::InvalidPlacement;
use crate::constants::GuessError;
use crate::constants::GuessResult;
use std::collections::HashSet;

/// Represents a single ship in the Battleship game.
///
/// Each ship has a name, length, and tracks its position and damage state.
pub struct Ship {
    /// Name of the ship (e.g., "Carrier", "Battleship")
    name: &'static str,
    /// Length of the ship in grid units
    length: usize,
    /// Set of coordinates the ship occupies
    coords: HashSet<(usize, usize)>,
    /// Set of coordinates where the ship has been hit
    hits: HashSet<(usize, usize)>,
    /// Whether the ship has been placed on the board
    placed: bool,
    /// Whether the ship has been sunk
    sunk: bool,
}

impl Ship {
    /// Creates a new ship instance.
    ///
    /// # Arguments
    /// * `name` - Name of the ship
    /// * `length` - Length of the ship in grid units
    ///
    /// # Returns
    /// * `Ship` - New ship instance ready for placement
    ///
    /// # Example
    /// ```
    /// use battleship::Ship;
    /// let carrier = Ship::new("Carrier", 5);
    /// assert_eq!(carrier.length(), 5);
    /// assert!(!carrier.is_placed());
    /// ```
    pub fn new(name: &'static str, length: usize) -> Self {
        Self {
            name,
            length,
            coords: HashSet::with_capacity(length),
            hits: HashSet::with_capacity(length),
            placed: false,
            sunk: false,
        }
    }

    /// Places the ship at the specified coordinates.
    ///
    /// # Arguments
    /// * `coords` - Set of coordinates where the ship will be placed
    ///
    /// # Returns
    /// * `Result<(), GameplayError>` - Ok(()) if successful, Error if invalid placement
    ///
    /// # Errors
    /// Returns `InvalidPlacement` if the number of coordinates doesn't match ship length
    pub fn place(&mut self, coords: HashSet<(usize, usize)>) -> Result<(), GameplayError> {
        if coords.len() != self.length {
            return Err(InvalidPlacement);
        }
        self.coords.extend(coords.into_iter());
        self.placed = true;
        Ok(())
    }

    /// Processes a guess against this ship.
    ///
    /// # Arguments
    /// * `target` - Coordinate being targeted
    ///
    /// # Returns
    /// * `Result<GuessResult, GuessError>` - Result indicating hit, miss, or sunk
    ///
    /// # Examples
    /// ```
    /// use battleship::Ship;
    /// use battleship::constants::GuessResult;
    /// use std::collections::HashSet;
    /// let mut ship = Ship::new("Destroyer", 2);
    /// ship.place(HashSet::from([(0,0), (0,1)])).unwrap();
    /// let result = ship.guess((0,0));
    /// assert!(matches!(result, Ok(GuessResult::Hit)));
    /// ```
    pub fn guess(&mut self, target: (usize, usize)) -> Result<GuessResult, GuessError> {
        if self.hits.contains(&target) {
            return Err(GuessError::AlreadyGuessed);
        }
        if self.coords.contains(&target) {
            self.hits.insert(target);
            if self.hits_remaining() == 0 {
                self.sunk = true;
                Ok(GuessResult::Sunk(self.name))
            } else {
                Ok(GuessResult::Hit)
            }
        } else {
            Ok(GuessResult::Miss)
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn coords(&self) -> &HashSet<(usize, usize)> {
        &self.coords
    }

    pub fn hits(&self) -> &HashSet<(usize, usize)> {
        &self.hits
    }

    pub fn is_placed(&self) -> bool {
        self.placed
    }

    pub fn is_sunk(&self) -> bool {
        self.sunk
    }

    /// Returns the number of hits needed to sink this ship.
    ///
    /// # Returns
    /// * `usize` - Number of unhit coordinates remaining
    pub fn hits_remaining(&self) -> usize {
        self.coords.len() - self.hits.len()
    }
}
