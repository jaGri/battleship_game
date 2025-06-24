use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ShipType { Carrier, Battleship, Cruiser, Submarine, Destroyer }

impl ShipType {
    pub fn len(&self) -> u8 {
        match self {
            ShipType::Carrier => 5,
            ShipType::Battleship => 4,
            ShipType::Cruiser => 3,
            ShipType::Submarine => 3,
            ShipType::Destroyer => 2,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ship {
    pub ship_type: ShipType,
    pub positions: Vec<(u8,u8)>,
    pub length: u8,
    pub hits: usize,
}

impl Ship {
    pub fn new(ship_type: ShipType, positions: Vec<(u8,u8)>) -> Self {
        let length = ship_type.len();
        Self { ship_type, positions, length, hits: 0 }
    }
}