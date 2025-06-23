//! Simulation and tests.

#[cfg(test)]
mod tests {
    use battleship_core::{Board, Cell, constants::SHIPS};
    use battleship_core::{setup::random_setup};
    use battleship_core::gameplay::{run_game, Player, Renderer, CoreError, Move as GameMove};
    use async_trait::async_trait;
    #[tokio::test] async fn test_random_setup() { /* ... */ }
    #[test] fn test_place() { /* ... */ }
    #[quickcheck] fn prop_place(x: u8, y: u8, len: u8, horiz: bool) -> bool { true }
}