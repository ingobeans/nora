use macroquad::{miniquad::window::screen_size, prelude::*, time};

use crate::{assets::Assets, player::Player, screens::*, utils::*};

mod assets;
mod entity;
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

    let cameras = CameraBundle::new();

    let mut player = Player::new();

    let mut screens = create_screen_registry(&assets);
    let mut last = time::get_time();

    screens
        .get_mut(ScreenID::Test)
        .on_load(ScreenUpdateContext {
            player: &mut player,
            assets: &assets,
            render_layers: &cameras,
        });
    set_default_camera();
    loop {
        clear_background(BLACK);
        let (actual_screen_width, actual_screen_height) = screen_size();
        let scale_factor =
            (actual_screen_width / SCREEN_WIDTH).min(actual_screen_height / SCREEN_HEIGHT);
        let (mouse_x, mouse_y) = mouse_position();
        let _mouse_x = mouse_x / scale_factor;
        let _mouse_y = mouse_y / scale_factor;

        let screen = screens.get_mut(screens::ScreenID::Test);
        for camera in cameras.get_redrawn().iter() {
            set_camera(*camera);
            clear_background(BLACK.with_alpha(0.0));
        }
        let now = time::get_time();
        if now - last >= 1.0 / 60.0 {
            last = now;
            screen.update(ScreenUpdateContext {
                player: &mut player,
                assets: &assets,
                render_layers: &cameras,
            });
        }

        screen.draw(ScreenUpdateContext {
            player: &mut player,
            assets: &assets,
            render_layers: &cameras,
        });

        set_default_camera();
        // draw cameras
        for camera in cameras.get_all().iter() {
            draw_texture_ex(
                &camera.render_target.as_ref().unwrap().texture,
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
