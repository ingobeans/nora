use macroquad::prelude::*;

use crate::assets::Assets;

mod assets;

#[macroquad::main("nora")]
async fn main() {
    println!("nora v{}", env!("CARGO_PKG_VERSION"));
    let assets = Assets::load();
    loop {
        next_frame().await;
    }
}
