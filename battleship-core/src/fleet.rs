use std::collections::HashSet;

use crate::constants::GameplayError;
use crate::constants::GuessError;
use crate::constants::GuessResult;

use battleship_config::SHIPS;

use crate::constants::GameplayError::ShipNotFound;
use crate::constants::GuessResult::{Hit, Miss, Sunk};
use crate::ship::Ship;

use array_init::array_init;

/// Manages a collection of ships for a player.
///
/// The Fleet struct handles ship placement, tracking hits,
/// and maintaining the overall state of all ships in play.
pub struct Fleet {
    /// Array of all ships in the fleet
    ships: [Ship; SHIPS.len()],
}

impl Fleet {
    /// Creates a new fleet with all standard ships.
    ///
    /// Initializes ships based on the SHIPS constant which defines
    /// the standard set of Battleship game pieces.
    ///
    /// # Returns
    /// * `Fleet` - New fleet with unplaced ships
    pub fn new() -> Self {
        Self {
            ships: array_init(|i: usize| {
                let (name, length) = SHIPS[i];
                Ship::new(name, length)
            }),
        }
    }

    /// Gets a reference to a ship by name.
    ///
    /// # Arguments
    /// * `name` - Name of the ship to find
    ///
    /// # Returns
    /// * `Result<&Ship, GameplayError>` - Reference to ship if found
    ///
    /// # Errors
    /// Returns `ShipNotFound` if no ship matches the given name
    pub fn get_ship(&self, name: &str) -> Result<&Ship, GameplayError> {
        self.ships
            .iter()
            .find(|ship| ship.name() == name)
            .ok_or(ShipNotFound)
    }

    /// Gets a mutable reference to a ship by name.
    ///
    /// # Arguments
    /// * `name` - Name of the ship to find
    ///
    /// # Returns
    /// * `Result<&mut Ship, GameplayError>` - Mutable reference to ship if found
    ///
    /// # Errors
    /// Returns `ShipNotFound` if no ship matches the given name
    fn get_ship_mut(&mut self, name: &str) -> Result<&mut Ship, GameplayError> {
        self.ships
            .iter_mut()
            .find(|ship| ship.name() == name)
            .ok_or(ShipNotFound)
    }

    /// Gets an iterator over ships based on their sunk status.
    ///
    /// # Arguments
    /// * `unsunk` - Include unsunk ships
    /// * `sunk` - Include sunk ships
    ///
    /// # Returns
    /// * Iterator over ships matching the criteria
    pub fn get_ships(&self, unsunk: bool, sunk: bool) -> impl Iterator<Item = &Ship> {
        self.ships
            .iter()
            .filter(move |ship| (unsunk && !ship.is_sunk()) || (sunk && ship.is_sunk()))
    }

    /// Gets names and lengths of ships based on their sunk status.
    ///
    /// # Arguments
    /// * `unsunk` - Include unsunk ships
    /// * `sunk` - Include sunk ships
    ///
    /// # Returns
    /// * Iterator over tuples of (ship name, ship length) matching the criteria
    pub fn get_ship_names_and_length(
        &self,
        unsunk: bool,
        sunk: bool,
    ) -> impl Iterator<Item = (&str, usize)> {
        let ships = self.get_ships(unsunk, sunk);
        ships.map(|ship| (ship.name(), ship.length()))
    }

    /// Processes a guess against the fleet.
    ///
    /// Checks each unsunk ship to see if the guess hits it.
    ///
    /// # Arguments
    /// * `target` - Coordinate being targeted
    ///
    /// # Returns
    /// * `Result<GuessResult, GuessError>` - Result of the guess
    pub fn guess(&mut self, target: (usize, usize)) -> Result<GuessResult, GuessError> {
        for ship in &mut self.ships {
            if ship.is_sunk() {
                continue;
            }
            match ship.guess(target) {
                Ok(Hit) => return Ok(Hit),
                Ok(Sunk(name)) => return Ok(Sunk(name)),
                Ok(Miss) => continue,
                Err(err) => return Err(err),
            }
        }
        Ok(Miss)
    }

    /// Gets an iterator over unplaced ships.
    ///
    /// # Returns
    /// * Iterator over ships that haven't been placed on the board
    pub fn unplaced_ships(&self) -> impl Iterator<Item = &Ship> {
        self.ships.iter().filter(|ship: &&Ship| !ship.is_placed())
    }

    /// Places a ship at the specified coordinates.
    ///
    /// # Arguments
    /// * `name` - Name of the ship to place
    /// * `coords` - Set of coordinates where the ship will be placed
    ///
    /// # Returns
    /// * `Result<(), GameplayError>` - Ok if successful, Error if placement invalid
    pub fn place_ship(
        &mut self,
        name: &str,
        coords: HashSet<(usize, usize)>,
    ) -> Result<(), GameplayError> {
        let ship = self.get_ship_mut(name)?;
        ship.place(coords)
    }

    /// Gets the coordinates of ships based on their sunk status.
    ///
    /// # Arguments
    /// * `unsunk` - Include coordinates of unsunk ships
    /// * `sunk` - Include coordinates of sunk ships
    ///
    /// # Returns
    /// * `HashSet<(usize,usize)>` - Set of coordinates for matching ships
    pub fn ship_coords(&self, unsunk: bool, sunk: bool) -> HashSet<(usize, usize)> {
        self.get_ships(unsunk, sunk)
            .flat_map(|s: &Ship| s.coords().iter().cloned())
            .collect()
    }

    /// Gets the coordinates of hits on ships based on their sunk status.
    ///
    /// # Arguments
    /// * `unsunk` - Include hits on unsunk ships
    /// * `sunk` - Include hits on sunk ships
    ///
    /// # Returns
    /// * `HashSet<(usize,usize)>` - Set of hit coordinates for matching ships
    pub fn hit_coords(&self, unsunk: bool, sunk: bool) -> HashSet<(usize, usize)> {
        self.get_ships(unsunk, sunk)
            .flat_map(|s: &Ship| s.hits().iter().cloned())
            .collect()
    }
    /// Counts the number of ships based on their sunk status.
    ///
    /// # Arguments
    /// * `unsunk` - Include unsunk ships
    /// * `sunk` - Include sunk ships
    ///
    /// # Returns
    /// * `usize` - Number of ships matching the criteria
    pub fn n_ships(&self, unsunk: bool, sunk: bool) -> usize {
        self.get_ships(unsunk, sunk).count()
    }

    /// Counts the total number of coordinates occupied by ships based on their sunk status.
    ///
    /// # Arguments
    /// * `unsunk` - Include unsunk ships
    /// * `sunk` - Include sunk ships
    ///
    /// # Returns
    /// * `usize` - Total number of coordinates occupied by matching ships
    pub fn n_ship_coords(&self, unsunk: bool, sunk: bool) -> usize {
        self.get_ships(unsunk, sunk)
            .map(|s: &Ship| s.length())
            .sum()
    }

    /// Counts the total number of hits on ships based on their sunk status.
    ///
    /// # Arguments
    /// * `unsunk` - Include hits on unsunk ships
    /// * `sunk` - Include hits on sunk ships
    ///
    /// # Returns
    /// * `usize` - Total number of hits on matching ships
    pub fn n_ship_hits(&self, unsunk: bool, sunk: bool) -> usize {
        self.get_ships(unsunk, sunk)
            .map(|s: &Ship| s.hits().len())
            .sum()
    }

    /// Gets the total number of coordinates occupied by all ships.
    ///
    /// # Returns
    /// * `usize` - Sum of all ship lengths
    pub fn total_hits(&self) -> usize {
        self.ships.iter().map(|s: &Ship| s.length()).sum()
    }

    /// Gets the number of unhit ship coordinates remaining.
    ///
    /// # Returns
    /// * `usize` - Number of hits needed to sink all remaining ships
    pub fn hits_remaining(&self) -> usize {
        let total_hits = self.total_hits();
        total_hits - self.n_ship_hits(true, true)
    }

    /// Returns the status for each ship as a tuple of name, length and whether
    /// it has been sunk.
    pub fn ship_statuses(&self) -> Vec<(&'static str, usize, bool)> {
        self.ships
            .iter()
            .map(|s| (s.name(), s.length(), s.is_sunk()))
            .collect()
    }
}
