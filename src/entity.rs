use macroquad::prelude::*;

use crate::{
    screens::{Map, ScreenUpdateContext},
    utils::*,
};

fn ceil_g(a: f32) -> f32 {
    if a < 0.0 { a.floor() } else { a.ceil() }
}

#[expect(unused_variables)]
pub trait Entity {
    fn update(&mut self, ctx: &Map) {}
    fn draw(&self, ctx: &ScreenUpdateContext) {}
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
        let tile = map.get_collision_tile(tx as _, ty as _);
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
