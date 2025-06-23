//! Console app.

use clap::Parser;
use battleship_core::{run_game, Player};
mod ui;
use ui::{ConsoleInput, ConsoleRenderer};
use battleship_ai::{AiPlayer, Difficulty};
use std::process;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "hh", value_parser = ["hh", "ai"])]
    mode: String,
    #[arg(long, value_parser = ["easy", "medium", "hard"], required_if_eq("mode", "ai"))]
    difficulty: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut p1: Box<dyn Player> = Box::new(ConsoleInput::new());
    let mut p2: Box<dyn Player> = match args.mode.as_str() {
        "hh" => Box::new(ConsoleInput::new()),
        "ai" => {
            let lvl = match args.difficulty.unwrap().as_str() {
                "easy" => Difficulty::Easy,
                "medium" => Difficulty::Medium,
                "hard" => Difficulty::Hard,
                _ => unreachable!(),
            };
            Box::new(AiPlayer::new(lvl))
        }
        _ => unreachable!(),
    };
    let renderer = ConsoleRenderer::new();
    if let Err(e) = run_game(&mut *p1, &mut *p2, &renderer).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}