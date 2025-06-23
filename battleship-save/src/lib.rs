use async_trait::async_trait;
use battleship_core::Board;
use battleship_common::Result;

#[async_trait]
pub trait SaveLoad {
    async fn save(&self, board: &Board) -> Result<()>;
    async fn load(&self) -> Result<Board>;
}

