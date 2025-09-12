use macroquad::prelude::*;

use crate::{
    assets::*,
    entity::update_physics_entity,
    screens::{Map, ScreenUpdateContext},
    utils::*,
};

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
    pub health: f32,
    pub max_health: f32,
    idle_animation: Animation,
    sprint_animation: Animation,
    slide_animation: Animation,
}
impl Player {
    pub fn new() -> Self {
        Self {
            // public states
            pos: Vec2::ZERO,
            camera_pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
            max_health: 100.0,
            health: 100.0,

            // internal states
            anim_frame: 0,
            jump_frames: 0,
            facing_right: true,
            on_ground: false,
            head_covered: false,
            standing: true,

            // assets
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

        let mut forces = Vec2::ZERO;

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
            } * if !self.standing {
                self.velocity.x.abs().min(2.0) / 2.0
            } else {
                1.0
            };
            self.jump_frames += 1;
        }

        forces.x -= self.velocity.x
            * if !self.standing {
                0.02
            } else if self.on_ground {
                GROUND_FRICTION
            } else {
                AIR_DRAG
            };
        let old_velocity = self.velocity;
        let (on_ground, head_covered) = update_physics_entity(
            &mut self.pos,
            &mut forces,
            &mut self.velocity,
            self.standing,
            map,
        );
        if !self.on_ground
            && on_ground
            && !self.standing
            && self.velocity.x.abs() > 1.0
            && old_velocity.y > 0.0
        {
            let m = if self.facing_right { 1.0 } else { -1.0 };

            self.velocity.x += (1.0 / (old_velocity.y) + 2.0) * m;
        }
        if self.standing {
            self.velocity.x = self.velocity.x.clamp(-MAX_RUN_VELOCITY, MAX_RUN_VELOCITY);
        }
        self.on_ground = on_ground;
        self.head_covered = head_covered;

        self.camera_pos.x = self.pos.x.floor();
        let delta = self.camera_pos.y - self.pos.y.floor();
        let max_delta = 3.0 * 8.0;
        if delta.abs() >= max_delta {
            self.camera_pos.y =
                max_delta * if delta < 0.0 { -1.0 } else { 1.0 } + self.pos.y.floor();
        }
    }
    pub fn draw(&self, ctx: &ScreenUpdateContext) {
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
