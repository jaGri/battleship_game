// Stub for embedded interface
use battleship_core::state::GameState;
use battleship_core::message::Message;

pub struct EmbeddedInput;
impl EmbeddedInput { pub fn new() -> Self { EmbeddedInput } }
impl EmbeddedInput { pub fn read_buttons(&self) -> interface_cli::InputEvent { unimplemented!() } }

pub struct EmbeddedDisplay;
impl EmbeddedDisplay { pub fn new() -> Self { EmbeddedDisplay } }
impl EmbeddedDisplay {
    pub fn draw(&self, state: &GameState) { unimplemented!() }
    pub fn draw_message(&self, msg: &Message) { unimplemented!() }
}
