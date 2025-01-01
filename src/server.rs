use crate::common::*;
use rand::Rng;

enum GameState {
    InGame { field: Box<dyn TServerField> },
    NotInGame,
    GameOver,
}

pub struct Server {
    game_state: GameState,
    // field: Box<dyn TServerField>,
    // clients: Vec<u32>
}

impl Server {
    pub fn new() -> Server {
        Server {
            game_state: GameState::NotInGame,
        }
    }

    pub fn new_game(&mut self, init_params: &InitParams) {
        match init_params.grid_type {
            GridType::RectGrid { heigth, width } => {
                self.game_state = GameState::InGame {
                    field: Box::new(RectServerField::new(heigth, width, init_params.mines_cnt)),
                }
            }
            GridType::HexGrid => std::unimplemented!(),
        }
    }

    pub fn process_client_data(&mut self, client_package: Vec<Coords>) -> Vec<Cell> {
        match &self.game_state {
            GameState::InGame { field } => field.open_cells(client_package),
            _ => {
                vec![]
            }
        }
    }
}

trait TServerField {
    fn open_cells(&self, coords: Vec<Coords>) -> Vec<Cell>;
}

struct RectServerField {
    heigth: usize,
    width: usize,
    mines_cnt: usize,
    rows: Vec<Vec<CellState>>,
}

impl RectServerField {
    fn new(heigth: usize, width: usize, mut mines_cnt: usize) -> Self {
        if heigth == 0 || width == 0 || heigth * width - 1 < mines_cnt {
            panic!("invalid field params!")
        }
        let mut f = RectServerField {
            width,
            heigth,
            mines_cnt,
            rows: vec![vec![CellState::Empty(0); width]; heigth],
        };
        // todo quicker algo
        while mines_cnt > 0 {
            let (r, c) = (
                ::rand::thread_rng().gen_range(0..heigth),
                ::rand::thread_rng().gen_range(0..width),
            );
            // todo: 1 cell empty for multipleer or generate on first click for singleplayer
            // if center.is_some() && mine_coord == center.unwrap() {
            //     continue;
            // }
            if f.rows[r][c] != CellState::Mine {
                f.rows[r][c] = CellState::Mine;
                mines_cnt -= 1;
            }
        }
        // count empties
        for r in 0..f.heigth {
            for c in 0..width {
                if f.rows[r][c] == CellState::Mine {
                    continue;
                }
                let mut cnt = 0;
                let min_r = if r == 0 { 0 } else { r - 1 };
                let max_r = if r == f.heigth - 1 { f.heigth } else { r + 2 };
                let min_c = if c == 0 { 0 } else { c - 1 };
                let max_c = if c == f.width - 1 { f.width } else { c + 2 };
                for rr in min_r..max_r {
                    for cc in min_c..max_c {
                        cnt += if f.rows[rr][cc] == CellState::Mine {
                            1
                        } else {
                            0
                        };
                    }
                }
                f.rows[r][c] = CellState::Empty(cnt);
            }
        }
        f
    }

    fn fill_recursive(&self, crds: Coords, filled: &mut Vec<Vec<bool>>, res: &mut Vec<Cell>) {
        if let Coords::RectCoords { row, col } = crds {
            if filled[row][col] {
                return;
            }
            filled[row][col] = true;
            match self.rows[row][col] {
                CellState::Empty(x) => {
                    res.push(Cell {
                        crds,
                        state: CellState::Empty(x),
                    });
                    if x == 0 {
                        let min_r = if row == 0 { row } else { row - 1 };
                        let max_r = if row == self.heigth - 1 { row } else { row + 1 };
                        let min_c = if col == 0 { col } else { col - 1 };
                        let max_c = if col == self.width - 1 { col } else { col + 1 };
                        for r in min_r..=max_r {
                            for c in min_c..=max_c {
                                self.fill_recursive(
                                    Coords::RectCoords { row: r, col: c },
                                    filled,
                                    res,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        } else {
            panic!("wrong coords type");
        }
    }
}

impl TServerField for RectServerField {
    fn open_cells(&self, all_coords: Vec<Coords>) -> Vec<Cell> {
        let mut res = Vec::<Cell>::new();
        let mut filled = vec![vec![false; self.width]; self.heigth];
        for crds in all_coords {
            if let Coords::RectCoords { row, col } = crds {
                match self.rows[row][col] {
                    CellState::Mine => res.push(Cell {
                        crds,
                        state: CellState::Mine,
                    }),
                    CellState::Empty(x) => {
                        if x > 0 {
                            res.push(Cell {
                                crds,
                                state: CellState::Empty(x),
                            });
                        } else {
                            self.fill_recursive(crds, &mut filled, &mut res);
                        }
                    }
                }
            } else {
                panic!("wrong coords type");
            }
        }
        res
    }
}
