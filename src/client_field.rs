use super::primitives::Coords;
use super::primitives::CellState;

#[derive(Clone, PartialEq)]
pub enum VisibleCellState {
    Empty(u8),
    Mine,
    Closed,
    Marked,
    BlownMine,
}

pub struct VisibleField {
    pub width: usize,
    pub heigth: usize,
    pub rows: Vec<Vec<VisibleCellState>>,
}

impl VisibleField {
    pub fn new(width: usize, heigth: usize) -> Self {
        // check field params
        if heigth == 0 || width == 0 {
            panic!("invalid field params!")
        }
        VisibleField {
            width: width,
            heigth: heigth,
            rows: vec![vec![VisibleCellState::Closed; width]; heigth]
        }
    }

    pub fn display(&self) {
        for row in &self.rows {
            for cell_state in row {
                match cell_state {
                    VisibleCellState::Empty(n) => print!("{n} "),
                    VisibleCellState::Mine => print!("* "),
                    VisibleCellState::Closed => print!("■ "),
                    VisibleCellState::Marked => print!("🚩 "),
                    VisibleCellState::BlownMine => print!("💥 "),
                }
            }
            println!();
        }
    }

    pub fn update(&mut self, dat: &Vec<(Coords, CellState)>) {
        for el in dat {
            self.rows[el.0.row][el.0.col] = match el.1 {
                CellState::Mine => VisibleCellState::BlownMine,
                CellState::Empty(x) => VisibleCellState::Empty(x),
            }
        }
    }
}
