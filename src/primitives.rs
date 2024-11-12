#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Coords {
    pub row: usize,
    pub col: usize
}


#[derive(Clone, PartialEq)]
pub enum CellState {
    Empty(u8),
    Mine
}
