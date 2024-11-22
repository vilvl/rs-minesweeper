use macroquad::prelude::*;
use miniquad::window::set_window_size;
use ::rand::Rng;
use std::collections::HashSet;

enum GridType {
    RectGrid{heigth: usize, width: usize},
    HexGrid
}
struct InitParams {
    grid_type: GridType,
    mines_cnt: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Coords {
    RectCoords {
        row: usize,
        col: usize,
    },
    HexCoords {}
}

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Empty(u8),
    Mine
}
struct Cell {
    crds: Coords,
    state: CellState
}


trait TServerField {
    fn open_cells(&self, coords: Vec<Coords>) -> Vec<Cell>;
}
struct RectServerField {
    heigth: usize,
    width: usize,
    mines_cnt: usize,
    rows: Vec<Vec<CellState>>
}
impl RectServerField {
    fn new(width: usize, heigth: usize, mut mines_cnt: usize) -> Self {
        if heigth == 0 || width == 0 || heigth * width - 1 < mines_cnt {
            panic!("invalid field params!")
        }
        let mut f = RectServerField {
            width,
            heigth,
            mines_cnt,
            rows: vec![vec![CellState::Empty(0); width]; heigth]
        };
        // todo quicker algo
        while mines_cnt > 0 {
            let (r, c) = (::rand::thread_rng().gen_range(0..heigth), ::rand::thread_rng().gen_range(0..width));
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

    fn fill_recursive(&self, crds: Coords, filled: &mut Vec<Vec<bool>>, res: &mut Vec<Cell>) {
        if let Coords::RectCoords { row, col } = crds {
            if filled[row][col] {
                return;
            }
            filled[row][col] = true;
            match self.rows[row][col] {
                CellState::Empty(x) => {
                    res.push(Cell{crds, state: CellState::Empty(x)});
                    if x == 0 {
                        let min_r = if row == 0 {row} else {row - 1};
                        let max_r = if row == self.heigth - 1 {row} else {row + 1};
                        let min_c = if col == 0 {col} else {col - 1};
                        let max_c = if col == self.width - 1 {col} else {col + 1};
                        for r in min_r..=max_r {
                            for c in min_c..=max_c {
                                self.fill_recursive(Coords::RectCoords{row: r, col: c}, filled, res);
                            }
                        }
                    }

                },
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
                    CellState::Mine => res.push(Cell{crds, state: CellState::Mine}),
                    CellState::Empty(x) => if x > 0 {
                        res.push(Cell{crds, state: CellState::Empty(x)});
                    } else {
                        self.fill_recursive(crds, &mut filled, &mut res);
                    }
                }
            } else {
                panic!("wrong coords type");
            }
        }
        res
    }
}

enum InputType {
    OpenCell,
    MarkCell,
    OpenCellNeighbours,
    HighlightCell,
    HighlightNeighbours
}
struct Input {
    inp_type: InputType,
    coords: Coords
}

#[derive(Copy, Clone, PartialEq)]
enum VisibleCellState {
    Empty(u8),
    Mine,
    Closed,
    Marked,
    BlownMine,
}
trait TClientField {
    fn process_input(&mut self) -> Option<Vec<Coords>>;
    fn draw(&self);
    fn update(&mut self, update_pack: Vec<Cell>);
}
struct RectClientField {
    heigth: usize,
    width: usize,
    mines_cnt: usize,
    rows: Vec<Vec<VisibleCellState>>,
    highlighted_cells: HashSet<Coords>,
}
const SQ_SIZE: f32 = 30.;
impl RectClientField {
    fn new(width: usize, heigth: usize, mines_cnt: usize) -> Self {
        // check field params
        if heigth == 0 || width == 0 || heigth * width - 1 < mines_cnt {
            panic!("invalid field params!")
        }
        set_window_size(SQ_SIZE as u32 * width as u32, SQ_SIZE as u32 * heigth as u32);
        RectClientField {
            heigth,
            width,
            mines_cnt,
            rows: vec![vec![VisibleCellState::Closed; width]; heigth],
            highlighted_cells: HashSet::new()
        }
    }
}
impl TClientField for RectClientField {
    fn process_input(&mut self) -> Option<Vec<Coords>> {
        let pos = mouse_position();
        self.highlighted_cells.clear();
        // check pos, if not ret None
        let (row, col) = ((pos.1 / SQ_SIZE) as usize, (pos.0 / SQ_SIZE) as usize);
        if is_mouse_button_down(MouseButton::Left) {
            if is_mouse_button_down(MouseButton::Right) {
                self.highlighted_cells.insert(Coords::RectCoords { row, col });  // todo: add neighbours
                None
            } else {
                self.highlighted_cells.insert(Coords::RectCoords { row, col });
                None
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            if !is_mouse_button_down(MouseButton::Right) {
                Some(vec![Coords::RectCoords { row, col }])  // todo: add neighbours
            } else {
                if (self.rows[row][col] == VisibleCellState::Closed) {
                    Some(vec![Coords::RectCoords { row, col }])
                } else {
                    None
                }
            }
        } else if is_mouse_button_released(MouseButton::Right) {
            match self.rows[row][col] {
                VisibleCellState::Closed => self.rows[row][col] = VisibleCellState::Marked,
                VisibleCellState::Marked => self.rows[row][col] = VisibleCellState::Closed,
                _ => {}
            }
            None
        } else {
            None
        }
    }

    fn draw(&self) {
        clear_background(LIGHTGRAY);
        let offset_x = 0.;
        let offset_y = 0.;

        draw_rectangle(offset_x, offset_y, SQ_SIZE * self.width as f32, SQ_SIZE * self.heigth as f32, WHITE);

        for (row, cells) in self.rows.iter().enumerate() {
            for (col, cell) in cells.iter().enumerate() {
                match cell {
                    VisibleCellState::BlownMine => {
                        draw_rectangle(col as f32 * SQ_SIZE, row as f32 * SQ_SIZE, SQ_SIZE, SQ_SIZE, RED);
                    },
                    VisibleCellState::Mine => {
                        draw_rectangle(col as f32 * SQ_SIZE, row as f32 * SQ_SIZE, SQ_SIZE, SQ_SIZE, BLACK);
                    },
                    VisibleCellState::Closed => {
                        let crds = Coords::RectCoords { row, col };
                        if self.highlighted_cells.contains(&crds) {
                            draw_rectangle(col as f32 * SQ_SIZE, row as f32 * SQ_SIZE, SQ_SIZE, SQ_SIZE, WHITE);
                        } else {
                            draw_rectangle(col as f32 * SQ_SIZE, row as f32 * SQ_SIZE, SQ_SIZE, SQ_SIZE, DARKGRAY);
                        }
                    },
                    VisibleCellState::Empty(0) => {
                        draw_rectangle(col as f32 * SQ_SIZE, row as f32 * SQ_SIZE, SQ_SIZE, SQ_SIZE, WHITE);
                    },
                    VisibleCellState::Empty(x) => {
                        draw_text(format!("{}", x).as_str(), (col as f32 + 0.3) as f32 * SQ_SIZE, (row as f32 + 0.75) * SQ_SIZE, SQ_SIZE, GOLD);
                    },
                    _ => {}
                }
            }
        }
    }

    fn update(&mut self, update_pack: Vec<Cell>) {
        for cell in update_pack {
            if let Coords::RectCoords { row, col } = cell.crds {
                self.rows[row][col] = match cell.state {
                    CellState::Mine => VisibleCellState::BlownMine,
                    CellState::Empty(x) => VisibleCellState::Empty(x),
                }
            } else {
                panic!("wrong coords type")
            }
        }
    }
}


fn init_server_field(params: &InitParams) -> Box<dyn TServerField> {
    Box::new(match params.grid_type {
        GridType::RectGrid{width, heigth} => RectServerField::new(width, heigth, params.mines_cnt),
        GridType::HexGrid => panic!("Not implemented")
    })
}

fn init_client_field(params: &InitParams) -> Box<dyn TClientField> {
    Box::new(match params.grid_type {
        GridType::RectGrid{width, heigth} => RectClientField::new(width, heigth, params.mines_cnt),
        GridType::HexGrid => panic!("Not implemented")
    })
}


#[macroquad::main("Rs-Mines")]
async fn main() {
    let params = InitParams {
        grid_type: GridType::RectGrid{heigth: 20, width: 30},
        mines_cnt: 99,
    };

    let server_field = init_server_field(&params);
    let mut client_field = init_client_field(&params);

    client_field.draw();
    loop {
        if let Some(coords) = client_field.process_input() {
            let opened_cells = server_field.open_cells(coords);
            client_field.update(opened_cells);
        }
        client_field.draw();
        next_frame().await;
    }
}
