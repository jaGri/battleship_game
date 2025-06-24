// Stub for embedded interface
use game_core::state::{GameState, Orientation};
use game_core::message::Message;
use interface_cli::InputEvent;

pub struct EmbeddedInput;
impl EmbeddedInput { pub fn new() -> Self { EmbeddedInput } }
impl EmbeddedInput { pub fn read_buttons(&self) -> InputEvent { unimplemented!() } }

pub struct EmbeddedDisplay;
impl EmbeddedDisplay { pub fn new() -> Self { EmbeddedDisplay } }
impl EmbeddedDisplay {
    pub fn draw(&self, state: &GameState) { unimplemented!() }
    pub fn draw_message(&self, msg: &Message) { unimplemented!() }
}
