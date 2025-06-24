use serde_json;
use std::fs;
use std::io;
use std::path::Path;
use battleship_core::state::GameState;

pub trait Persistence {
    fn save(&self, state: &GameState, path: &Path) -> io::Result<()>;
    fn load(&self, path: &Path) -> io::Result<GameState>;
}

pub struct JsonPersistence;
impl Persistence for JsonPersistence {
    fn save(&self, state: &GameState, path: &Path) -> io::Result<()> {
        let tmp = path.with_extension("json.tmp");
        let data = serde_json::to_vec_pretty(state).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(&tmp, &data)?;
        fs::rename(&tmp, path)?;
        Ok(())
    }
    fn load(&self, path: &Path) -> io::Result<GameState> {
        let data = fs::read(path)?;
        let state = serde_json::from_slice(&data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(state)
    }
}

pub struct EmbeddedPersistence;
impl Persistence for EmbeddedPersistence {
    fn save(&self, _state: &GameState, _path: &Path) -> io::Result<()> { unimplemented!() }
    fn load(&self, _path: &Path) -> io::Result<GameState> { unimplemented!() }
}
