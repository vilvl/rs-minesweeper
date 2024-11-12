pub mod field;
pub mod client_field;
pub mod primitives;

use crate::primitives::Coords;
use crate::client_field::VisibleField;
use crate::field::Field;

use client_field::VisibleCellState;
use macroquad::{prelude::*, window};
use miniquad::window::set_window_size;

struct InitParams {
    width: usize,
    heigth: usize,
    mines: usize
}

struct State {
    server_field: Field,
    client_field: VisibleField,
    init_params: InitParams
}

const SQ_SIZE: f32 = 30.;

fn init() -> State {
    let params = InitParams {
        width: 20,
        heigth: 10,
        mines: 30
    };
    set_window_size(SQ_SIZE as u32 * params.width as u32, SQ_SIZE as u32 * params.heigth as u32);
    State {
        server_field: Field::new(params.width, params.heigth, params.mines, None),
        client_field: VisibleField::new(params.width, params.heigth),
        init_params: params
    }
}

fn get_input() -> Option<Coords> {
    if is_mouse_button_released(MouseButton::Left) {
        let pos = mouse_position();
        Some(Coords {
            row: (pos.1 / SQ_SIZE) as usize,
            col: (pos.0 / SQ_SIZE) as usize
        })
    } else {
        None
    }
}

trait Draw {
    fn draw(&self);
}


impl Draw for VisibleField {
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
                        draw_rectangle(col as f32 * SQ_SIZE, row as f32 * SQ_SIZE, SQ_SIZE, SQ_SIZE, DARKGRAY);
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
}

#[macroquad::main("Rs-Mines")]
async fn main() {
    let mut state = init();

    state.client_field.draw();
    loop {
        if let Some(coords) = get_input() {
            let update = state.server_field.check(coords);
            state.client_field.update(&update);
        }
        state.client_field.draw();
        next_frame().await;
    }
}
