use macroquad::prelude::*;

use crate::{
    assets::AnimationID,
    graphics::DrawCall,
    screens::{Map, ScreenUpdateContext},
    utils::*,
};

fn ceil_g(a: f32) -> f32 {
    if a < 0.0 { a.floor() } else { a.ceil() }
}

#[expect(unused_variables)]
pub trait NonPlayerEntity {
    fn update(&mut self, map: &Map, ctx: &mut ScreenUpdateContext) {}
    fn draw(&self, ctx: &mut ScreenUpdateContext) {}
}
pub struct HumanoidEnemy {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub anim_frame: u32,
    pub animation: AnimationID,
    pub on_ground: bool,
    pub speed: f32,
}
impl HumanoidEnemy {
    pub fn new(pos: Vec2, animation: AnimationID, speed: f32) -> Self {
        Self {
            pos,
            velocity: Vec2::ZERO,
            anim_frame: 0,
            animation,
            on_ground: false,
            speed,
        }
    }
}
impl NonPlayerEntity for HumanoidEnemy {
    fn draw(&self, ctx: &mut ScreenUpdateContext) {
        ctx.render_layers.entities.calls.push(DrawCall::Animation(
            self.animation,
            self.anim_frame,
            self.pos.floor().x - 4.0,
            self.pos.floor().y - 8.0,
            Some(DrawTextureParams {
                flip_x: self.velocity.x < 0.0,
                ..Default::default()
            }),
        ));
    }
    fn update(&mut self, map: &Map, ctx: &mut ScreenUpdateContext) {
        self.anim_frame += 1000 / 60;
        let mut forces = Vec2::ZERO;
        let player_delta = ctx.player.pos - self.pos;

        // move towards player
        if player_delta.length() >= 8.0 {
            forces.x = player_delta.x;
            forces = forces.clamp_length_max(1.0) * self.speed;

            // handle special pathfinding
            let tile = (self.pos / 8.0).round();
            let tile = map.get_special_tile(tile.x as _, tile.y as _);

            let should_jump = tile != 0
                && match tile - 1 {
                    0 => true,
                    1 => player_delta.x < 0.0,
                    2 => player_delta.x > 0.0,

                    _ => false,
                };
            if should_jump && self.on_ground {
                forces.y -= 8.5;
            }
        } else {
            // attack player
            ctx.player.health -= 10.0;
        }
        forces.x -= self.velocity.x
            * if self.on_ground {
                GROUND_FRICTION
            } else {
                AIR_DRAG
            };

        let result =
            update_physics_entity(&mut self.pos, &mut forces, &mut self.velocity, true, map);
        self.on_ground = result.0;
    }
}

pub fn update_physics_entity(
    pos: &mut Vec2,
    forces: &mut Vec2,
    velocity: &mut Vec2,
    tall: bool,
    map: &Map,
) -> (bool, bool) {
    forces.y += GRAVITY;

    *velocity += *forces;

    let mut new = *pos + *velocity;

    let tile_x = pos.x / 8.0;
    let tile_y = pos.y / 8.0;

    let mut tiles_y = vec![
        (tile_x.trunc(), ceil_g(new.y / 8.0)),
        (ceil_g(tile_x), ceil_g(new.y / 8.0)),
        (tile_x.trunc(), (new.y / 8.0).trunc()),
        (ceil_g(tile_x), (new.y / 8.0).trunc()),
    ];
    if tall {
        tiles_y.push((tile_x.trunc(), (new.y / 8.0).trunc() - 1.0));
        tiles_y.push((ceil_g(tile_x), (new.y / 8.0).trunc() - 1.0));
    }

    let mut on_ground = false;
    for (tx, ty) in tiles_y {
        let tile = map.get_collision_tile(tx as _, ty as _);
        if tile != 0 {
            let c = if velocity.y < 0.0 {
                tile_y.floor() * 8.0
            } else {
                on_ground = true;
                tile_y.ceil() * 8.0
            };
            new.y = c;
            velocity.y = 0.0;
            break;
        }
    }
    let mut tiles_x = vec![
        ((new.x / 8.0).trunc(), ceil_g(new.y / 8.0)),
        (ceil_g(new.x / 8.0), ceil_g(new.y / 8.0)),
        (ceil_g(new.x / 8.0), (new.y / 8.0).trunc()),
        ((new.x / 8.0).trunc(), (new.y / 8.0).trunc()),
    ];
    if tall {
        tiles_x.push(((new.x / 8.0).trunc(), (new.y / 8.0).trunc() - 1.0));
        tiles_x.push((ceil_g(new.x / 8.0), (new.y / 8.0).trunc() - 1.0));
    }

    for (tx, ty) in tiles_x {
        let tile = if tx >= 0.0 {
            map.get_collision_tile(tx as _, ty as _)
        } else {
            1
        };
        if tile != 0 {
            let c = if velocity.x < 0.0 {
                tile_x.floor() * 8.0
            } else {
                tile_x.ceil() * 8.0
            };
            new.x = c;
            velocity.x = 0.0;
            break;
        }
    }

    // check if head covered
    let m = if tall { 2.0 } else { 1.0 };
    let tiles_head = [
        ((new.x / 8.0).trunc(), ceil_g(new.y / 8.0) - m),
        (ceil_g(new.x / 8.0), ceil_g(new.y / 8.0) - m),
    ];
    let mut head_covered = false;
    for (tx, ty) in tiles_head {
        let tile = map.get_collision_tile(tx as _, ty as _);
        if tile != 0 {
            head_covered = true;
        }
    }

    if velocity.x.abs() <= 0.3 {
        velocity.x = 0.0;
    }
    *pos = new;

    (on_ground, head_covered)
}
