use macroquad::prelude::*;

use crate::{
    assets::*,
    screens::{Map, ScreenDrawContext},
    utils::*,
};

fn ceil_g(a: f32) -> f32 {
    if a < 0.0 { a.floor() } else { a.ceil() }
}

pub struct Player {
    pub pos: Vec2,
    pub camera_pos: Vec2,
    pub velocity: Vec2,
    pub anim_frame: u32,
    pub facing_right: bool,
    pub on_ground: bool,
    pub jump_frames: u8,
    idle_animation: Animation,
    sprint_animation: Animation,
}
impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            camera_pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
            anim_frame: 0,
            jump_frames: 0,
            facing_right: true,
            on_ground: false,
            idle_animation: Animation::from_file(include_bytes!(
                "../assets/entities/player/idle.ase"
            )),
            sprint_animation: Animation::from_file(include_bytes!(
                "../assets/entities/player/sprint.ase"
            )),
        }
    }
    pub fn update(&mut self, map: &Map) {
        self.anim_frame += 1000 / 60;

        // only allow noclip on debug builds
        #[cfg(debug_assertions)]
        let noclip = is_key_down(KeyCode::LeftShift);
        #[cfg(not(debug_assertions))]
        let noclip = { false };

        let mut forces = Vec2::ZERO;

        if !noclip {
            forces.y += GRAVITY
        }

        if is_key_down(KeyCode::A) {
            forces.x -= 1.0;
            self.facing_right = false;
        }
        if is_key_down(KeyCode::D) {
            forces.x += 1.0;
            self.facing_right = true;
        }

        if self.on_ground {
            self.jump_frames = 0;
        }
        if is_key_down(KeyCode::Space)
            && (self.on_ground || (self.jump_frames > 0 && self.jump_frames < 5))
        {
            forces.y -= if self.jump_frames == 0 { 1.5 } else { 1.0 };
            self.jump_frames += 1;
        }

        if noclip {
            if is_key_down(KeyCode::W) {
                forces.y -= 1.0;
            }
            if is_key_down(KeyCode::S) {
                forces.y += 1.0;
            }
            self.velocity += forces * 2.0;
            self.velocity = self.velocity.lerp(Vec2::ZERO, GROUND_FRICTION);

            self.pos += self.velocity;
            self.camera_pos = self.pos.floor();
            return;
        }

        forces.x -= self.velocity.x
            * if self.on_ground {
                GROUND_FRICTION
            } else {
                AIR_DRAG
            };

        self.velocity += forces;

        let mut new = self.pos + self.velocity;

        let tile_x = self.pos.x / 16.0;
        let tile_y = self.pos.y / 16.0;

        let tiles_y = [
            (tile_x.trunc(), ceil_g(new.y / 16.0)),
            (ceil_g(tile_x), ceil_g(new.y / 16.0)),
            (tile_x.trunc(), (new.y / 16.0).trunc()),
            (ceil_g(tile_x), (new.y / 16.0).trunc()),
        ];

        self.on_ground = false;
        for (tx, ty) in tiles_y {
            let tile = map.get_collision_tile(tx as _, ty as _);
            if tile != 0 {
                let c = if self.velocity.y < 0.0 {
                    tile_y.floor() * 16.0
                } else {
                    self.on_ground = true;
                    tile_y.ceil() * 16.0
                };
                new.y = c;
                self.velocity.y = 0.0;
                break;
            }
        }
        let tiles_x = [
            ((new.x / 16.0).trunc(), ceil_g(new.y / 16.0)),
            (ceil_g(new.x / 16.0), ceil_g(new.y / 16.0)),
            (ceil_g(new.x / 16.0), (new.y / 16.0).trunc()),
            ((new.x / 16.0).trunc(), (new.y / 16.0).trunc()),
        ];

        for (tx, ty) in tiles_x {
            let tile = map.get_collision_tile(tx as _, ty as _);
            if tile != 0 {
                let c = if self.velocity.x < 0.0 {
                    tile_x.floor() * 16.0
                } else {
                    tile_x.ceil() * 16.0
                };
                new.x = c;
                self.velocity.x = 0.0;
                break;
            }
        }

        if self.velocity.x.abs() <= 0.3 {
            self.velocity.x = 0.0;
        }
        self.velocity.x = self.velocity.x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        self.pos = new;

        self.camera_pos.x = self.pos.x.floor();
        let delta = self.camera_pos.y - self.pos.y.floor();
        let max_delta = 3.0 * 16.0;
        if delta.abs() >= max_delta {
            self.camera_pos.y =
                max_delta * if delta < 0.0 { -1.0 } else { 1.0 } + self.pos.y.floor();
        }
    }
    pub fn draw(&self, ctx: &ScreenDrawContext) {
        let animation = if self.velocity.length() != 0.0 {
            &self.sprint_animation
        } else {
            &self.idle_animation
        };
        set_camera(&ctx.render_layers.detail2);
        draw_texture_ex(
            animation.get_at_time(self.anim_frame),
            self.pos.floor().x,
            self.pos.floor().y,
            WHITE,
            DrawTextureParams {
                flip_x: !self.facing_right,
                ..Default::default()
            },
        );
    }
}
