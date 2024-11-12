use rand::Rng;

use super::primitives::Coords;
use super::primitives::CellState;

#[derive(Clone)]
pub struct Field {
    width: usize,
    heigth: usize,
    rows: Vec<Vec<CellState>>,
}

impl Field {
    pub fn new(width: usize, heigth: usize, mut mines: usize, center: Option<Coords>) -> Self {
        // check field params
        if heigth == 0 || width == 0 || heigth * width - 1 < mines {
            panic!("invalid field params!")
        }
        let mut f = Field {
            width: width,
            heigth: heigth,
            rows: vec![vec![CellState::Empty(0); width]; heigth]
        };
        // generate mines
        while mines > 0 {
            let mine_coord = Coords{
                row: rand::thread_rng().gen_range(0..heigth),
                col: rand::thread_rng().gen_range(0..width)
            };
            if center.is_some() && mine_coord == center.unwrap() {
                continue;
            }
            if f.rows[mine_coord.row][mine_coord.col] != CellState::Mine {
                f.rows[mine_coord.row][mine_coord.col] = CellState::Mine;
                mines -= 1;
            }
        }
        // count empties
        for r in 0..f.heigth {
            for c in 0..width {
                if f.rows[r][c] == CellState::Mine {continue}
                let mut cnt = 0;
                let min_r = if r == 0 {0} else {r - 1};
                let max_r = if r == f.heigth - 1 {f.heigth} else {r + 2};
                let min_c = if c == 0 {0} else {c - 1};
                let max_c = if c == f.width - 1 {f.width} else {c + 2};
                for rr in min_r..max_r {
                    for cc in min_c..max_c {
                        cnt += if f.rows[rr][cc] == CellState::Mine {1} else {0};
                    }
                }
                f.rows[r][c] = CellState::Empty(cnt);
            }
        }
        f
    }

    pub fn display(&self) {
        for row in &self.rows {
            for cell_state in row {
                match cell_state {
                    CellState::Empty(n) => print!("{n} "),
                    CellState::Mine => print!("* "),
                }
            }
            println!();
        }
    }

    fn fill_recursive(&self, crds: Coords, filled: &mut Vec<Vec<bool>>, res: &mut Vec<(Coords, CellState)>) {
        if filled[crds.row][crds.col] {
            return;
        }
        filled[crds.row][crds.col] = true;
        match self.rows[crds.row][crds.col] {
            CellState::Empty(x) => {
                res.push((crds.clone(), CellState::Empty(x)));
                if x == 0 {
                    let min_r = if crds.row == 0 {crds.row} else {crds.row - 1};
                    let max_r = if crds.row == self.heigth - 1 {crds.row} else {crds.row + 1};
                    let min_c = if crds.col == 0 {crds.col} else {crds.col - 1};
                    let max_c = if crds.col == self.width - 1 {crds.col} else {crds.col + 1};
                    for r in min_r..=max_r {
                        for c in min_c..=max_c {
                            self.fill_recursive(Coords{row: r, col: c}, filled, res);
                        }
                    }
                }

            },
            _ => {}
        }
    }

    pub fn check(&self, c: Coords) -> Vec<(Coords, CellState)> {
        match self.rows[c.row][c.col] {
            CellState::Mine => vec![(c, CellState::Mine)],
            CellState::Empty(x) => if x > 0 {
                vec![(c, CellState::Empty(x))]
            } else {
                let mut filled = vec![vec![false; self.width]; self.heigth];
                let mut res = Vec::<(Coords, CellState)>::new();
                self.fill_recursive(c, &mut filled, &mut res);
                res
            }
        }
    }
}
