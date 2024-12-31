use macroquad::prelude::*;

pub mod client;
pub mod common;
pub mod server;
use crate::client::*;
use crate::common::*;
use crate::server::*;

// fn init_client_field(params: &InitParams) -> Box<dyn TClientField> {
//     Box::new(match params.grid_type {
//         GridType::RectGrid{width, heigth} => RectClientField::new(width, heigth, params.mines_cnt),
//         GridType::HexGrid => panic!("Not implemented")
//     })
// }

#[macroquad::main("Rs-Mines")]
async fn main() {
    let params = InitParams {
        grid_type: GridType::RectGrid {
            heigth: 20,
            width: 30,
        },
        mines_cnt: 99,
    };
    let mut server = Server::new();
    server.new_game(&params);

    let mut client = Client::new(&params);
    // let mut client_field = init_client_field(&params);

    client.run(|cells| server.process_client_data(cells)).await;
}
