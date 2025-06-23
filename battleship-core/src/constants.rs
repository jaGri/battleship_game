// Constants related to the game configuration

use std::fmt;

/// Represents the result of a guess attempt
#[derive(Debug, PartialEq)]
pub enum GuessResult {
    /// Shot missed any ships
    Miss,
    /// Shot hit a ship but didn't sink it
    Hit,
    /// Shot hit and sunk a ship (includes ship name)
    Sunk(&'static str),
}

impl fmt::Display for GuessResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuessResult::Miss => write!(f, "Miss"),
            GuessResult::Hit => write!(f, "Hit"),
            GuessResult::Sunk(s) => write!(f, "The {} was sunk!", s),
        }
    }
}

/// Represents possible errors during guess attempts
#[derive(Debug)]
pub enum GuessError {
    /// Coordinate has already been guessed
    AlreadyGuessed,
    /// Coordinate is outside the valid grid
    InvalidTarget,
    /// No valid coordinates remain
    NoValidCoordinates,
    /// Random guess generation failed
    RandomGuessFailed,
}

/// Represents possible errors during gameplay
#[derive(Debug)]
pub enum GameplayError {
    /// Ship placement is invalid (overlaps or out of bounds)
    InvalidPlacement,
    /// Cannot find a valid placement for the ship
    CantFindValidPlacement,
    /// Referenced ship doesn't exist
    ShipNotFound,
    /// No valid coordinates available
    NoValidCoordinates,
}

/// Represents the state of a player
#[derive(Debug, PartialEq)]
pub enum PlayerState {
    /// Player is still placing ships
    Setup,
    /// Player has ships remaining
    Alive,
    /// All player's ships have been sunk
    Dead,
}

/// Represents different cell states on the game board
#[derive(Debug)]
pub enum Cell {
    /// Empty cell
    Empty,
    /// Contains a ship
    Ship,
    // Preview of ship placement
    //ShipPrev,
    /// Hit on a ship
    Hit,
    /// Missed shot
    Miss,
    // Cursor position
    //Cursor,
}

impl Cell {
    /// Returns the display character for each cell state
    ///
    /// # Returns
    /// * `char` - Character representation of the cell state
    pub fn icon(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Ship => '■',
            //Cell::ShipPrev => '☐',
            Cell::Hit => 'X',
            Cell::Miss => 'O',
            //Cell::Cursor => '⌖',
        }
    }
}
