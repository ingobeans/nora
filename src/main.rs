use macroquad::{miniquad::window::screen_size, prelude::*, time};

use crate::{assets::Assets, graphics::RenderLayers, player::Player, screens::*, utils::*};

mod assets;
mod entity;
mod graphics;
mod player;
mod screens;
mod utils;

fn window_conf() -> Conf {
    Conf {
        window_title: "nora".to_string(),
        window_width: SCREEN_WIDTH as i32 * 3,
        window_height: SCREEN_HEIGHT as i32 * 3,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    println!("nora v{}", env!("CARGO_PKG_VERSION"));
    let assets = Assets::load();

    let mut render_layers = RenderLayers::new();

    let mut player = Player::new();

    let mut screens = create_screen_registry();
    let mut last = time::get_time();

    set_default_camera();

    let mut current_screen = screens::ScreenID::Street;
    let mut spawn: Option<usize> = Some(0);

    loop {
        clear_background(BLACK);
        let (actual_screen_width, actual_screen_height) = screen_size();
        let scale_factor =
            (actual_screen_width / SCREEN_WIDTH).min(actual_screen_height / SCREEN_HEIGHT);
        let (mouse_x, mouse_y) = mouse_position();
        let _mouse_x = mouse_x / scale_factor;
        let _mouse_y = mouse_y / scale_factor;

        let screen = screens.get_mut(current_screen);

        if let Some(i) = spawn {
            spawn = None;
            screen.on_load(
                ScreenUpdateContext {
                    player: &mut player,
                    render_layers: &mut render_layers,
                },
                i,
            );
        }

        let now = time::get_time();
        if now - last >= 1.0 / 60.0 {
            last = now;
            match screen.update(ScreenUpdateContext {
                player: &mut player,
                render_layers: &mut render_layers,
            }) {
                ScreenUpdateResult::Pass => {}
                ScreenUpdateResult::ChangeScreen(screen, i) => {
                    current_screen = screen;
                    spawn = Some(i);
                }
            }
        }

        screen.draw(ScreenUpdateContext {
            player: &mut player,
            render_layers: &mut render_layers,
        });

        // draw cameras
        for layer in render_layers.get_all().into_iter() {
            if !layer.calls.is_empty() {
                set_camera(&layer.camera);
                layer.draw(&assets);
            }
            set_default_camera();
            draw_texture_ex(
                &layer.camera.render_target.as_ref().unwrap().texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(
                        SCREEN_WIDTH * scale_factor,
                        SCREEN_HEIGHT * scale_factor,
                    )),
                    ..Default::default()
                },
            );
        }
        next_frame().await;
    }
}
