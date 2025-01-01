use crate::common::*;

trait TRect {}

impl TRect for RectCoords {}
impl TCoords for RectCoords {}
struct RectCoords {
    col: usize,
    row: usize,
}

impl<CellState> TRect for RectGrid<CellState> {}
impl<CellState> TGrid for RectGrid<CellState> {}
struct RectGrid<CellState> {
    pub heigth: usize,
    pub width: usize,
    pub rows: Vec<Vec<CellState>>,
}

impl TRect for RectGridInitParams {}
pub struct RectGridInitParams {
    pub heigth: usize,
    pub width: usize,
}
