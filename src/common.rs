pub enum GridType {
    RectGrid{heigth: usize, width: usize},
    HexGrid
}
pub struct InitParams {
    pub grid_type: GridType,
    pub mines_cnt: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Coords {
    RectCoords {
        row: usize,
        col: usize,
    },
    HexCoords {}
}

#[derive(Copy, Clone, PartialEq)]
pub enum CellState {
    Empty(u8),
    Mine
}
pub struct Cell {
    pub crds: Coords,
    pub state: CellState
}
