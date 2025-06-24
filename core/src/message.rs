use serde::{Serialize, Deserialize};
use crate::state::{GameState, PlayerId};
use crate::ship::ShipType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Envelope {
    pub seq: u64,
    pub ack: Option<u64>,
    pub payload: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Handshake { player: PlayerId },
    StateSync(GameState),
    Attack { x: u8, y: u8 },
    AttackResult { hit: bool, sunk: Option<ShipType> },
    SaveRequest,
    LoadRequest,
}
