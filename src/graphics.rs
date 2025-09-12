use crate::{
    assets::{AnimationID, Assets},
    utils::*,
};
use macroquad::prelude::*;

pub enum DrawCall {
    Animation(AnimationID, u32, f32, f32, Option<DrawTextureParams>),
    Rect(f32, f32, f32, f32, Color),
    Tileset(f32, f32, f32, f32),
    Clear(Color),
}

pub struct RenderLayer {
    pub camera: Camera2D,
    pub calls: Vec<DrawCall>,
}
impl RenderLayer {
    pub fn new() -> Self {
        Self {
            camera: create_camera(SCREEN_WIDTH, SCREEN_HEIGHT),
            calls: Vec::new(),
        }
    }
    pub fn draw(&mut self, assets: &Assets) {
        for call in self.calls.drain(..) {
            match call {
                DrawCall::Animation(id, time, x, y, params) => {
                    draw_texture_ex(
                        assets.animations.get(id).get_at_time(time),
                        x,
                        y,
                        WHITE,
                        params.unwrap_or_default(),
                    );
                }
                DrawCall::Clear(color) => {
                    clear_background(color);
                }
                DrawCall::Rect(x, y, w, h, color) => {
                    draw_rectangle(x, y, w, h, color);
                }
                DrawCall::Tileset(x, y, sx, sy) => {
                    assets.tileset.draw_sprite(sx, sy, x, y);
                }
            }
        }
    }
}

pub struct RenderLayers {
    /// Should mostly be static
    pub world: RenderLayer,
    /// Should mostly be static. Drawn over entity layer.
    pub world_foreground: RenderLayer,
    /// Render layer entities are drawn onto.
    pub entities: RenderLayer,
    /// Unused as of yet. Should be unaffected to manipulations to the rendering.
    pub ui: RenderLayer,
}

impl RenderLayers {
    pub fn new() -> Self {
        Self {
            world: RenderLayer::new(),
            entities: RenderLayer::new(),
            world_foreground: RenderLayer::new(),
            ui: RenderLayer::new(),
        }
    }
    pub fn get_redrawn(&mut self) -> [&mut RenderLayer; 2] {
        [&mut self.entities, &mut self.ui]
    }
    pub fn get_all(&mut self) -> [&mut RenderLayer; 4] {
        [
            &mut self.world,
            &mut self.entities,
            &mut self.world_foreground,
            &mut self.ui,
        ]
    }
}
