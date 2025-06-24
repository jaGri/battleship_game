use serde::{Serialize, Deserialize};
use crate::board::Board;
use crate::ship::ShipType;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Phase { Handshake, Placement, Playing, Finished }

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum PlayerId { One, Two }

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub board_p1: Board,
    pub board_p2: Board,
    pub turn: PlayerId,
    pub phase: Phase,
}

impl GameState {
    pub fn new(width: u8, height: u8) -> Self {
        Self { board_p1: Board::empty(width,height), board_p2: Board::empty(width,height),
            turn: PlayerId::One, phase: Phase::Handshake }
    }
    pub fn receive_attack(&mut self, attacker: PlayerId, x: u8, y: u8) -> (bool, Option<ShipType>) {
        let board = match attacker {
            PlayerId::One => &mut self.board_p2,
            PlayerId::Two => &mut self.board_p1,
        };
        if let Some(idx) = board.grid_index(x,y) {
            let cell = &mut board.cells[idx];
            if let crate::state::Cell::Ship(id) = *cell {
                *cell = crate::state::Cell::Hit;
                let ship = &mut board.ships[id as usize];
                ship.hits += 1;
                if ship.hits >= usize::from(ship.length) {
                    let positions = ship.positions.clone();
                    let ship_type = ship.ship_type;
                    drop(ship);
                    for &(sx,sy) in &positions {
                        let i = board.grid_index(sx,sy).unwrap();
                        board.cells[i] = crate::state::Cell::Sunk;
                    }
                    return (true, Some(ship_type));
                }
                return (true, None);
            } else {
                *cell = crate::state::Cell::Miss;
            }
        }
        (false, None)
    }
    pub fn apply_attack(&mut self, attacker: PlayerId, x:u8, y:u8) {
        let _ = self.receive_attack(attacker, x,y);
        self.turn = match attacker { PlayerId::One=>PlayerId::Two, PlayerId::Two=>PlayerId::One };
    }
    pub fn is_valid_attack(&self, attacker: PlayerId, x:u8, y:u8) -> bool {
        let b = match attacker { PlayerId::One=>&self.board_p2, PlayerId::Two=>&self.board_p1 };
        b.is_unattacked(x,y)
    }
    pub fn is_game_over(&self) -> bool {
        self.board_p1.all_sunk() || self.board_p2.all_sunk()
    }
}

// Define Cell enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cell { Empty, Ship(u8), Hit, Miss, Sunk }

// Orientation
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Orientation { Horizontal, Vertical }
