use async_trait::async_trait;
use core::state::GameState;
use core::message::Message;
use crate::state::Orientation;
use std::io::{self, Write};

#[async_trait]
pub trait InputProvider: Clone + Send {
    async fn next_input(&mut self) -> io::Result<InputEvent>;
}

#[async_trait]
pub trait OutputRenderer: Clone + Send {
    async fn render(&mut self, msg: &Message) -> io::Result<()>;
    async fn render_state(&mut self, state: &GameState) -> io::Result<()>;
}

#[derive(Clone)]
pub struct CliInterface;
impl CliInterface { pub fn new() -> Self { CliInterface } }

#[async_trait]
impl InputProvider for CliInterface {
    async fn next_input(&mut self) -> io::Result<InputEvent> {
        print!("> "); io::stdout().flush()?;
        let mut buf = String::new(); io::stdin().read_line(&mut buf)?;
        let t = buf.trim();
        if t.starts_with("attack ") {
            let rest = &t[7..];
            let parts: Vec<&str> = rest.split(',').collect();
            if parts.len()==2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse()) {
                    return Ok(InputEvent::Attack { x, y });
                }
            }
        }
        if t.starts_with("place ") {
            let rest = &t[6..];
            let parts: Vec<&str> = rest.split(',').collect();
            if parts.len()==3 {
                if let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse()) {
                    let ori = match parts[2].to_uppercase().as_str() {
                        "H" => Orientation::Horizontal,
                        "V" => Orientation::Vertical,
                        _ => Orientation::Horizontal,
                    };
                    return Ok(InputEvent::Place { x, y, orientation: ori });
                }
            }
        }
        if t=="save" { return Ok(InputEvent::Save); }
        if t=="load" { return Ok(InputEvent::Load); }
        Ok(InputEvent::Unknown)
    }
}

#[async_trait]
impl OutputRenderer for CliInterface {
    async fn render(&mut self, msg: &Message) -> io::Result<()> {
        println!("<< {:?}", msg); Ok(())
    }
    async fn render_state(&mut self, state: &GameState) -> io::Result<()> {
        println!("Phase: {:?}, Turn: {:?}", state.phase, state.turn);
        println!("Board P1: {:?}", state.board_p1.cells);
        println!("Board P2: {:?}", state.board_p2.cells);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Attack { x: u8, y: u8 },
    Place { x: u8, y: u8, orientation: Orientation },
    Save, Load, Unknown,
}
