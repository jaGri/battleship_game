pub trait BoardView: std::fmt::Display {
    fn grid_size(&self) -> usize;
}
