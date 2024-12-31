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

pub struct Scoreboard {
    // timer: u32,
    found_mines: u32,
    total_mines: u32,
    message: String,
    position: (f32, f32),
}

impl Scoreboard {
    pub fn new(total_mines: u32) -> Scoreboard {
        Scoreboard {
            found_mines: 0,
            total_mines,
            message: String::from(""),
            position: (0., 0.),
        }
    }

    pub fn draw(&self, board_size: (f32, f32)) {
        const TEXT_WIDTH_OFFSET: f32 = 30.;
        const TEXT_HEIGHT_OFFSET: f32 = 40.;
        const PADDING: f32 = 5.;

        draw_rectangle_lines(
            self.position.0 + PADDING,
            self.position.1 + PADDING,
            board_size.0 - PADDING * 2.,
            board_size.1 - PADDING * 2.,
            2.,
            BLACK,
        );
        draw_text(
            format!("Mines: {}/{}", self.found_mines, self.total_mines).as_str(),
            self.position.0 + TEXT_WIDTH_OFFSET,
            self.position.1 + TEXT_HEIGHT_OFFSET,
            30.,
            BLACK,
        );
        draw_text(
            // self.message.as_str(),
            "Hello, Minesweeper!",
            self.position.0 + TEXT_WIDTH_OFFSET + 400.,
            self.position.1 + TEXT_HEIGHT_OFFSET,
            30.,
            BLACK,
        );
    }

    pub fn update_marked_mines(&mut self, is_marked: bool) {
        if is_marked {
            self.found_mines += 1;
        } else {
            self.found_mines -= 1;
        }
    }
}

struct BoardOffset {
    x: f32,
    y: f32,
}

impl BoardOffset {
    fn new(heigth: f32) -> BoardOffset {
        BoardOffset { x: 0., y: heigth }
    }
}

pub struct Client {
    scoreboard: Scoreboard,
    field: Box<dyn TClientField>, // game_state: GameState,
                                  // client_id
}

impl Client {
    pub fn new(init_params: &InitParams) -> Client {
        match init_params.grid_type {
            GridType::RectGrid { heigth, width } => Client {
                scoreboard: Scoreboard::new(init_params.mines_cnt as u32),
                field: Box::new(RectClientField::new(heigth, width, init_params.mines_cnt)),
            },
            GridType::HexGrid => std::unimplemented!(),
        }
    }

    pub fn render(&self) {
        let window_width = SQ_SIZE as u32 * self.field.width() as u32;
        let window_height = SQ_SIZE as u32 * (self.field.heigth() as u32 + BOARD_HEIGHT as u32);

        set_window_size(window_width, window_height);
        clear_background(LIGHTGRAY);
        self.scoreboard
            .draw((window_width as f32, BOARD_HEIGHT as f32 * SQ_SIZE));
        self.field.draw();
    }

    pub async fn run<F>(&mut self, mut process_client_data: F) -> !
    where
        F: FnMut(Vec<Coords>) -> Vec<Cell>,
    {
        self.render();
        loop {
            if let Some(coords) = self.field.process_input() {
                if is_mouse_button_released(MouseButton::Right) {
                    self.scoreboard
                        .update_marked_mines(self.field.is_cell_marked(&coords[0]));
                } else {
                    let opened_cells = process_client_data(coords);
                    self.field.update(opened_cells);
                }
            }
            self.render();
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
    fn width(&self) -> usize;
    fn heigth(&self) -> usize;
    fn translate_to_board_position(&self, mouse_x: f32, mouse_y: f32) -> (f32, f32);
    fn is_within_bounds(&self, pos: (f32, f32)) -> bool;
    fn get_grid_coords(&self, pos: (f32, f32)) -> (usize, usize);
    fn process_input(&mut self) -> Option<Vec<Coords>>;
    fn draw(&self);
    fn update(&mut self, update_pack: Vec<Cell>);
    fn is_cell_marked(&self, coords: &Coords) -> bool;
}
struct RectClientField {
    heigth: usize,
    width: usize,
    mines_cnt: usize,
    board_offset: BoardOffset,
    rows: Vec<Vec<VisibleCellState>>,
    highlighted_cells: HashSet<Coords>,
}

const BOARD_HEIGHT: usize = 3;
const SQ_SIZE: f32 = 30.;
const GRID_LINE_THICKNESS: f32 = 1.0;

impl RectClientField {
    fn new(heigth: usize, width: usize, mines_cnt: usize) -> RectClientField {
        // check field params
        if heigth == 0 || width == 0 || heigth * width - 1 < mines_cnt {
            panic!("invalid field params!")
        }
        RectClientField {
            heigth,
            width,
            board_offset: BoardOffset::new(BOARD_HEIGHT as f32 * SQ_SIZE),
            mines_cnt,
            rows: vec![vec![VisibleCellState::Closed; width]; heigth],
            highlighted_cells: HashSet::new(),
        }
    }
}
impl TClientField for RectClientField {
    fn width(&self) -> usize {
        self.width
    }

    fn heigth(&self) -> usize {
        self.heigth
    }

    fn translate_to_board_position(&self, mouse_x: f32, mouse_y: f32) -> (f32, f32) {
        (mouse_x - self.board_offset.x, mouse_y - self.board_offset.y)
    }

    fn is_within_bounds(&self, pos: (f32, f32)) -> bool {
        let (x, y) = pos;

        x >= 0.0
            && x < (self.width as f32 * SQ_SIZE)
            && y >= 0.0
            && y < (self.heigth as f32 * SQ_SIZE)
    }

    fn get_grid_coords(&self, pos: (f32, f32)) -> (usize, usize) {
        let row = (pos.1 / SQ_SIZE) as usize;
        let col = (pos.0 / SQ_SIZE) as usize;

        (row, col)
    }

    fn process_input(&mut self) -> Option<Vec<Coords>> {
        let (mouse_x, mouse_y) = mouse_position();
        let adjusted_pos = self.translate_to_board_position(mouse_x, mouse_y);

        if !self.is_within_bounds(adjusted_pos) {
            return None;
        }
        let (row, col) = self.get_grid_coords(adjusted_pos);

        self.highlighted_cells.clear();
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
                if self.rows[row][col] == VisibleCellState::Closed {
                    Some(vec![Coords::RectCoords { row, col }]) // todo: add neighbours
                } else {
                    None
                }
            } else {
                if self.rows[row][col] == VisibleCellState::Closed {
                    Some(vec![Coords::RectCoords { row, col }])
                } else {
                    None
                }
            }
        } else if is_mouse_button_released(MouseButton::Right) {
            match self.rows[row][col] {
                VisibleCellState::Closed => {
                    self.rows[row][col] = VisibleCellState::Marked;
                    Some(vec![Coords::RectCoords { row, col }])
                }
                VisibleCellState::Marked => {
                    self.rows[row][col] = VisibleCellState::Closed;
                    Some(vec![Coords::RectCoords { row, col }])
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn draw(&self) {
        // Draw background
        draw_rectangle(
            self.board_offset.x,
            self.board_offset.y,
            SQ_SIZE * self.width as f32,
            SQ_SIZE * self.heigth as f32,
            WHITE,
        );

        for (row, cells) in self.rows.iter().enumerate() {
            for (col, cell) in cells.iter().enumerate() {
                match cell {
                    VisibleCellState::BlownMine => {
                        draw_rectangle(
                            col as f32 * SQ_SIZE + self.board_offset.x,
                            row as f32 * SQ_SIZE + self.board_offset.y,
                            SQ_SIZE,
                            SQ_SIZE,
                            RED,
                        );
                    }
                    VisibleCellState::Mine => {
                        draw_rectangle(
                            col as f32 * SQ_SIZE + self.board_offset.x,
                            row as f32 * SQ_SIZE + self.board_offset.y,
                            SQ_SIZE,
                            SQ_SIZE,
                            BLACK,
                        );
                    }
                    VisibleCellState::Closed => {
                        let crds = Coords::RectCoords { row, col };
                        if self.highlighted_cells.contains(&crds) {
                            draw_rectangle(
                                col as f32 * SQ_SIZE + self.board_offset.x,
                                row as f32 * SQ_SIZE + self.board_offset.y,
                                SQ_SIZE,
                                SQ_SIZE,
                                WHITE,
                            );
                        } else {
                            draw_rectangle(
                                col as f32 * SQ_SIZE + self.board_offset.x,
                                row as f32 * SQ_SIZE + self.board_offset.y,
                                SQ_SIZE,
                                SQ_SIZE,
                                DARKGRAY,
                            );
                        }
                    }
                    VisibleCellState::Marked => {
                        draw_rectangle(
                            col as f32 * SQ_SIZE + self.board_offset.x,
                            row as f32 * SQ_SIZE + self.board_offset.y,
                            SQ_SIZE,
                            SQ_SIZE,
                            YELLOW,
                        );
                    }
                    VisibleCellState::Empty(0) => {
                        draw_rectangle(
                            col as f32 * SQ_SIZE + self.board_offset.x,
                            row as f32 * SQ_SIZE + self.board_offset.y,
                            SQ_SIZE,
                            SQ_SIZE,
                            WHITE,
                        );
                    }
                    VisibleCellState::Empty(x) => {
                        draw_text(
                            format!("{}", x).as_str(),
                            (col as f32 + 0.3) as f32 * SQ_SIZE + self.board_offset.x,
                            (row as f32 + 0.75) * SQ_SIZE + self.board_offset.y,
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
                col as f32 * SQ_SIZE + self.board_offset.x,
                0.0 + self.board_offset.y,
                col as f32 * SQ_SIZE + self.board_offset.x,
                self.heigth as f32 * SQ_SIZE + self.board_offset.y,
                GRID_LINE_THICKNESS,
                GRAY,
            );
        }

        // Draw horizontal grid lines
        for row in 0..=self.heigth {
            draw_line(
                0.0 + self.board_offset.x,
                row as f32 * SQ_SIZE + self.board_offset.y,
                self.width as f32 * SQ_SIZE + self.board_offset.x,
                row as f32 * SQ_SIZE + self.board_offset.y,
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

    fn is_cell_marked(&self, coords: &Coords) -> bool {
        if let Coords::RectCoords { row, col } = coords {
            self.rows[*row][*col] == VisibleCellState::Marked
        } else {
            panic!("wrong coords type")
        }
    }
}
