use crate::common::*;
use macroquad::prelude::*;
use miniquad::window::set_window_size;
use std::collections::HashSet;

// enum GameState {
//     InGame {
//     },
//     NotInGame,
//     GameOver,
// }

pub struct Client {
    field: Box<dyn TClientField>, // game_state: GameState,
                                  // client_id
}

impl Client {
    pub fn new(init_params: &InitParams) -> Client {
        match init_params.grid_type {
            GridType::RectGrid { heigth, width } => Client {
                field: Box::new(RectClientField::new(heigth, width, init_params.mines_cnt)),
            },
            GridType::HexGrid => std::unimplemented!(),
        }
    }

    pub async fn run<F>(&mut self, mut process_client_data: F) -> !
    where
        F: FnMut(Vec<Coords>) -> Vec<Cell>,
    {
        self.field.draw();
        loop {
            if let Some(coords) = self.field.process_input() {
                let opened_cells = process_client_data(coords);
                self.field.update(opened_cells);
            }
            self.field.draw();
            next_frame().await;
        }
    }
}

enum InputType {
    OpenCell,
    MarkCell,
    OpenCellNeighbours,
    HighlightCell,
    HighlightNeighbours,
}
struct Input {
    inp_type: InputType,
    coords: Coords,
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
const GRID_LINE_THICKNESS: f32 = 1.0;

impl RectClientField {
    fn new(heigth: usize, width: usize, mines_cnt: usize) -> Self {
        // check field params
        if heigth == 0 || width == 0 || heigth * width - 1 < mines_cnt {
            panic!("invalid field params!")
        }
        set_window_size(
            SQ_SIZE as u32 * width as u32,
            SQ_SIZE as u32 * heigth as u32,
        );
        RectClientField {
            heigth,
            width,
            mines_cnt,
            rows: vec![vec![VisibleCellState::Closed; width]; heigth],
            highlighted_cells: HashSet::new(),
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
                self.highlighted_cells
                    .insert(Coords::RectCoords { row, col }); // todo: add neighbours
                None
            } else {
                self.highlighted_cells
                    .insert(Coords::RectCoords { row, col });
                None
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            if !is_mouse_button_down(MouseButton::Right) {
                Some(vec![Coords::RectCoords { row, col }]) // todo: add neighbours
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

        draw_rectangle(
            offset_x,
            offset_y,
            SQ_SIZE * self.width as f32,
            SQ_SIZE * self.heigth as f32,
            WHITE,
        );

        for (row, cells) in self.rows.iter().enumerate() {
            for (col, cell) in cells.iter().enumerate() {
                match cell {
                    VisibleCellState::BlownMine => {
                        draw_rectangle(
                            col as f32 * SQ_SIZE,
                            row as f32 * SQ_SIZE,
                            SQ_SIZE,
                            SQ_SIZE,
                            RED,
                        );
                    }
                    VisibleCellState::Mine => {
                        draw_rectangle(
                            col as f32 * SQ_SIZE,
                            row as f32 * SQ_SIZE,
                            SQ_SIZE,
                            SQ_SIZE,
                            BLACK,
                        );
                    }
                    VisibleCellState::Closed => {
                        let crds = Coords::RectCoords { row, col };
                        if self.highlighted_cells.contains(&crds) {
                            draw_rectangle(
                                col as f32 * SQ_SIZE,
                                row as f32 * SQ_SIZE,
                                SQ_SIZE,
                                SQ_SIZE,
                                WHITE,
                            );
                        } else {
                            draw_rectangle(
                                col as f32 * SQ_SIZE,
                                row as f32 * SQ_SIZE,
                                SQ_SIZE,
                                SQ_SIZE,
                                DARKGRAY,
                            );
                        }
                    }
                    VisibleCellState::Empty(0) => {
                        draw_rectangle(
                            col as f32 * SQ_SIZE,
                            row as f32 * SQ_SIZE,
                            SQ_SIZE,
                            SQ_SIZE,
                            WHITE,
                        );
                    }
                    VisibleCellState::Empty(x) => {
                        draw_text(
                            format!("{}", x).as_str(),
                            (col as f32 + 0.3) as f32 * SQ_SIZE,
                            (row as f32 + 0.75) * SQ_SIZE,
                            SQ_SIZE,
                            GOLD,
                        );
                    }
                    _ => {}
                }
            }
        }

        // Draw vertical lines
        for col in 0..=self.width {
            draw_line(
                col as f32 * SQ_SIZE,
                0.0,
                col as f32 * SQ_SIZE,
                self.heigth as f32 * SQ_SIZE,
                GRID_LINE_THICKNESS,
                GRAY,
            );
        }

        // Draw horizontal grid lines
        for row in 0..=self.heigth {
            draw_line(
                0.0,
                row as f32 * SQ_SIZE,
                self.width as f32 * SQ_SIZE,
                row as f32 * SQ_SIZE,
                GRID_LINE_THICKNESS,
                GRAY,
            );
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
