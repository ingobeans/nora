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
    pub head_covered: bool,
    pub jump_frames: u8,
    pub standing: bool,
    idle_animation: Animation,
    sprint_animation: Animation,
    slide_animation: Animation,
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
            head_covered: false,
            standing: true,
            idle_animation: Animation::from_file(include_bytes!(
                "../assets/entities/player/idle.ase"
            )),
            sprint_animation: Animation::from_file(include_bytes!(
                "../assets/entities/player/sprint.ase"
            )),
            slide_animation: Animation::from_file(include_bytes!(
                "../assets/entities/player/slide.ase"
            )),
        }
    }
    fn can_slide(&self) -> bool {
        true
    }
    pub fn update(&mut self, map: &Map) {
        self.anim_frame += 1000 / 60;

        let noclip = false;

        let mut forces = Vec2::ZERO;

        if !noclip {
            forces.y += GRAVITY
        }
        let mut speed = PLAYER_SPEED;
        let can_slide = self.can_slide();
        let shift_pressed = is_key_down(KeyCode::LeftShift);

        if self.standing {
            if shift_pressed && can_slide {
                self.standing = false;
                speed *= 1.5;
            }
        } else {
            if !shift_pressed && !self.head_covered {
                self.standing = true;
            }
            speed = 0.0;
            if self.velocity.x.abs() < 0.5 && self.head_covered {
                forces.x += 0.5 * if self.facing_right { 1.0 } else { -1.0 };
            }
        }

        if speed > 0.0 {
            if is_key_down(KeyCode::A) {
                forces.x -= speed;
                self.facing_right = false;
            }
            if is_key_down(KeyCode::D) {
                forces.x += speed;
                self.facing_right = true;
            }
        }

        if self.on_ground {
            self.jump_frames = 0;
        }
        if is_key_down(KeyCode::Space)
            && (self.on_ground || (self.jump_frames > 0 && self.jump_frames < 5))
        {
            forces.y -= if self.jump_frames == 0 {
                3.5
            } else {
                1.5 * (5 - self.jump_frames) as f32 / 2.5
            };
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
            * if !self.standing {
                0.02
            } else if self.on_ground {
                GROUND_FRICTION
            } else {
                AIR_DRAG
            };

        self.velocity += forces;

        let mut new = self.pos + self.velocity;

        let tile_x = self.pos.x / 8.0;
        let tile_y = self.pos.y / 8.0;

        let mut tiles_y = vec![
            (tile_x.trunc(), ceil_g(new.y / 8.0)),
            (ceil_g(tile_x), ceil_g(new.y / 8.0)),
            (tile_x.trunc(), (new.y / 8.0).trunc()),
            (ceil_g(tile_x), (new.y / 8.0).trunc()),
        ];
        if self.standing {
            tiles_y.push((tile_x.trunc(), (new.y / 8.0).trunc() - 1.0));
            tiles_y.push((ceil_g(tile_x), (new.y / 8.0).trunc() - 1.0));
        }

        let was_on_ground = self.on_ground;
        let old_velocity = self.velocity;
        self.on_ground = false;
        for (tx, ty) in tiles_y {
            let tile = map.get_collision_tile(tx as _, ty as _);
            if tile != 0 {
                let c = if self.velocity.y < 0.0 {
                    tile_y.floor() * 8.0
                } else {
                    self.on_ground = true;
                    tile_y.ceil() * 8.0
                };
                new.y = c;
                self.velocity.y = 0.0;
                break;
            }
        }
        let mut tiles_x = vec![
            ((new.x / 8.0).trunc(), ceil_g(new.y / 8.0)),
            (ceil_g(new.x / 8.0), ceil_g(new.y / 8.0)),
            (ceil_g(new.x / 8.0), (new.y / 8.0).trunc()),
            ((new.x / 8.0).trunc(), (new.y / 8.0).trunc()),
        ];
        if self.standing {
            tiles_x.push(((new.x / 8.0).trunc(), (new.y / 8.0).trunc() - 1.0));
            tiles_x.push((ceil_g(new.x / 8.0), (new.y / 8.0).trunc() - 1.0));
        }

        for (tx, ty) in tiles_x {
            let tile = map.get_collision_tile(tx as _, ty as _);
            if tile != 0 {
                let c = if self.velocity.x < 0.0 {
                    tile_x.floor() * 8.0
                } else {
                    tile_x.ceil() * 8.0
                };
                new.x = c;
                self.velocity.x = 0.0;
                break;
            }
        }

        // check if head covered
        let m = if self.standing { 2.0 } else { 1.0 };
        let tiles_head = [
            ((new.x / 8.0).trunc(), ceil_g(new.y / 8.0) - m),
            (ceil_g(new.x / 8.0), ceil_g(new.y / 8.0) - m),
        ];
        self.head_covered = false;
        for (tx, ty) in tiles_head {
            let tile = map.get_collision_tile(tx as _, ty as _);
            if tile != 0 {
                self.head_covered = true;
            }
        }

        if self.velocity.x.abs() <= 0.3 {
            self.velocity.x = 0.0;
        }
        if self.standing {
            self.velocity.x = self.velocity.x.clamp(-MAX_RUN_VELOCITY, MAX_RUN_VELOCITY);
        }
        if !was_on_ground
            && self.on_ground
            && !self.standing
            && self.velocity.x.abs() > 1.0
            && old_velocity.y > 0.0
        {
            let m = if self.facing_right { 1.0 } else { -1.0 };

            self.velocity.x += (1.0 / (old_velocity.y) + 2.0) * m;
        }

        self.pos = new;

        self.camera_pos.x = self.pos.x.floor();
        let delta = self.camera_pos.y - self.pos.y.floor();
        let max_delta = 3.0 * 8.0;
        if delta.abs() >= max_delta {
            self.camera_pos.y =
                max_delta * if delta < 0.0 { -1.0 } else { 1.0 } + self.pos.y.floor();
        }
    }
    pub fn draw(&self, ctx: &ScreenDrawContext) {
        let animation = if !self.standing {
            &self.slide_animation
        } else if self.velocity.length() != 0.0 {
            &self.sprint_animation
        } else {
            &self.idle_animation
        };
        set_camera(&ctx.render_layers.entities);
        draw_texture_ex(
            animation.get_at_time(self.anim_frame),
            self.pos.floor().x - 4.0,
            self.pos.floor().y - 8.0,
            WHITE,
            DrawTextureParams {
                flip_x: !self.facing_right,
                ..Default::default()
            },
        );
    }
}
