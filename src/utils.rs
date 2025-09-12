use std::marker::PhantomData;

use enum_iterator::Sequence;
use macroquad::prelude::*;


pub const SCREEN_WIDTH: f32 = 384.0;
pub const SCREEN_HEIGHT: f32 = 216.0;

pub const MAX_RUN_VELOCITY: f32 = 2.2;
pub const GROUND_FRICTION: f32 = 0.21;
pub const PLAYER_SPEED: f32 = 2.0;
pub const AIR_DRAG: f32 = 0.07;
pub const GRAVITY: f32 = 0.9;

pub fn create_camera(w: f32, h: f32) -> Camera2D {
    let rt = render_target(w as u32, h as u32);
    rt.texture.set_filter(FilterMode::Nearest);

    Camera2D {
        render_target: Some(rt),
        zoom: Vec2::new(1.0 / w * 2.0, 1.0 / h * 2.0),
        target: Vec2::new(w / 2.0, h / 2.0),
        ..Default::default()
    }
}

pub struct Registry<A, T> {
    values: Vec<T>,
    id_type: PhantomData<A>,
}
impl<A: Sequence + Into<usize>, T> Registry<A, T> {
    pub fn new(create_function: Box<dyn Fn(A) -> T>) -> Self {
        let mut screens = Vec::new();
        for id in enum_iterator::all::<A>() {
            screens.push(create_function(id));
        }

        Self {
            values: screens,
            id_type: PhantomData,
        }
    }
    pub fn get(&self, id: A) -> &T {
        &self.values[id.into()]
    }
    pub fn get_mut(&mut self, id: A) -> &mut T {
        &mut self.values[id.into()]
    }
}
