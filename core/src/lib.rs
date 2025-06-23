//! # Battleship Core
//! Provides the core game logic, state types, and traits.
//! Supports both `std` and `no_std` environments via feature flags.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod constants;
pub mod board;
pub mod setup;
pub mod gameplay;

pub use constants::*;
pub use board::Board;
pub use gameplay::{Move as GameMove, Player, Renderer, InputSource, run_game, CoreError};
