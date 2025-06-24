use serde::{Serialize, Deserialize};
use crate::state::GameState;
use crate::ship::ShipType;
use crate::state::Orientation;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipPlacement {
    pub ship_type: ShipType,
    pub x: u8,
    pub y: u8,
    pub orientation: Orientation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipLayout {
    pub ships: Vec<ShipPlacement>,
}

impl ShipLayout {
    pub fn apply(&self, gs: &mut GameState) {
        for sp in &self.ships {
            gs.board_p1.place_ship(sp.ship_type, sp.x, sp.y, sp.orientation)
                .expect("Invalid ship placement in layout");
        }
    }
}
