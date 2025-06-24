use serde::{Serialize, Deserialize};
use crate::state::Orientation;
use crate::ship::ShipType;
use crate::state::Cell;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    pub width: u8,
    pub height: u8,
    pub cells: Vec<Cell>,
    pub ships: Vec<crate::ship::Ship>,
}

impl Board {
    pub fn empty(width: u8, height: u8) -> Self {
        let cells = vec![Cell::Empty; (width as usize)*(height as usize)];
        Self { width, height, cells, ships: Vec::new() }
    }
    fn idx(&self, x: u8, y: u8) -> Option<usize> {
        if x < self.width && y < self.height {
            Some((y as usize)*self.width as usize + x as usize)
        } else {
            None
        }
    }
    pub fn place_ship(&mut self, ship_type: ShipType, x: u8, y: u8, ori: Orientation) -> Result<(), String> {
        let length = ship_type.len();
        let mut positions = Vec::new();
        for i in 0..length {
            let (sx, sy) = match ori {
                Orientation::Horizontal => (x+i, y),
                Orientation::Vertical => (x, y+i),
            };
            if let Some(idx) = self.idx(sx, sy) {
                if self.cells[idx] != Cell::Empty {
                    return Err("Overlap".into());
                }
                positions.push((sx, sy));
            } else {
                return Err("Out of bounds".into());
            }
        }
        let ship = crate::ship::Ship::new(ship_type, positions.clone());
        self.ships.push(ship);
        let ship_id = self.ships.len() - 1;
        for (sx, sy) in positions {
            let idx = self.idx(sx, sy).unwrap();
            self.cells[idx] = Cell::Ship(ship_id as u8);
        }
        Ok(())
    }
    pub fn grid_index(&self, x: u8, y: u8) -> Option<usize> { self.idx(x, y) }
    pub fn is_unattacked(&self, x: u8, y: u8) -> bool {
        matches!(self.idx(x,y).and_then(|i| Some(self.cells[i])), Some(Cell::Empty)|Some(Cell::Ship(_)))
    }
    pub fn all_sunk(&self) -> bool {
        self.ships.iter().all(|s| s.hits >= s.length as usize)
    }
}
