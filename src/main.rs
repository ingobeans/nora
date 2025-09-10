use macroquad::prelude::*;

use crate::{assets::Assets, screens::*};

mod assets;
mod screens;
mod utils;

#[macroquad::main("nora")]
async fn main() {
    println!("nora v{}", env!("CARGO_PKG_VERSION"));
    let assets = Assets::load();
    let mut screens = screens::ScreensRegistry::new();
    loop {
        let screen = screens.get_mut(screens::ScreenID::Test);
        screen.update(ScreenUpdateContext {});
        screen.draw(ScreenDrawContext { assets: &assets });
        next_frame().await;
    }
}
