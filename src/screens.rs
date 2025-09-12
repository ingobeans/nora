use macroquad::prelude::*;
use struct_iterable::Iterable;

use crate::{
    assets::{Animation, Assets},
    entity::{HumanoidEnemy, NonPlayerEntity},
    player::Player,
    utils::*,
};

pub struct CameraBundle {
    /// World layers are not cleared every frame, as they are intended to be static.
    pub world: Camera2D,
    /// World layers are not cleared every frame, as they are intended to be static.
    pub world_foreground: Camera2D,
    /// Render layer entities are drawn onto.
    pub entities: Camera2D,
    /// Unused as of yet. Should be unaffected to manipulations to the rendering.
    pub ui: Camera2D,
}
impl CameraBundle {
    pub fn new() -> Self {
        Self {
            world: create_camera(SCREEN_WIDTH, SCREEN_HEIGHT),
            entities: create_camera(SCREEN_WIDTH, SCREEN_HEIGHT),
            world_foreground: create_camera(SCREEN_WIDTH, SCREEN_HEIGHT),
            ui: create_camera(SCREEN_WIDTH, SCREEN_HEIGHT),
        }
    }
    pub fn get_redrawn(&self) -> [&Camera2D; 2] {
        [&self.entities, &self.ui]
    }
    pub fn get_all(&self) -> [&Camera2D; 4] {
        [
            &self.world,
            &self.entities,
            &self.world_foreground,
            &self.ui,
        ]
    }
}

pub struct ScreenUpdateContext<'a> {
    pub player: &'a mut Player,
    pub assets: &'a Assets,
    pub render_layers: &'a CameraBundle,
}
pub enum ScreenUpdateResult {
    /// Does nothing special
    Pass,
}

#[expect(unused_variables)]
pub trait Screen {
    fn on_load(&mut self, ctx: ScreenUpdateContext) {}
    fn update(&mut self, ctx: ScreenUpdateContext) -> ScreenUpdateResult {
        ScreenUpdateResult::Pass
    }
    fn draw(&mut self, ctx: ScreenUpdateContext) {}
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum ScreenID {
    Test,
}
pub struct ScreensRegistry {
    screens: Vec<Box<dyn Screen>>,
}
impl ScreensRegistry {
    pub fn new(assets: &Assets) -> Self {
        let mut screens = Vec::new();
        for id in enum_iterator::all::<ScreenID>() {
            screens.push(Self::load_screen_from_id(id, assets));
        }

        Self { screens }
    }
    pub fn get(&self, id: ScreenID) -> &Box<dyn Screen> {
        &self.screens[id as usize]
    }
    pub fn get_mut(&mut self, id: ScreenID) -> &mut Box<dyn Screen> {
        &mut self.screens[id as usize]
    }
    fn load_screen_from_id(id: ScreenID, assets: &Assets) -> Box<dyn Screen> {
        match id {
            ScreenID::Test => Box::new(TilemapScreen::new(
                include_str!("../assets/screens/test.tmx"),
                vec![Box::new(HumanoidEnemy::new(
                    Vec2::new(25.0, 17.0) * 8.0,
                    Animation::from_file(include_bytes!("../assets/entities/enemy.ase")),
                    0.4,
                ))],
            )),
        }
    }
}

type Tiles = Vec<usize>;

#[derive(Iterable)]
pub struct Map {
    background: Tiles,
    walls: Tiles,
    collision: Tiles,
    detail: Tiles,
    detail2: Tiles,
    special: Tiles,
}
impl Map {
    pub fn get_collision_tile(&self, x: usize, y: usize) -> usize {
        if x >= 48 {
            return 0;
        }
        if y >= 27 {
            return 0;
        }
        self.collision[x + y * 48]
    }
    pub fn get_special_tile(&self, x: usize, y: usize) -> usize {
        if x >= 48 {
            return 0;
        }
        if y >= 27 {
            return 0;
        }
        self.special[x + y * 48]
    }
    fn layers(&self) -> [&Tiles; 5] {
        [
            &self.background,
            &self.walls,
            &self.collision,
            &self.detail,
            &self.detail2,
        ]
    }
    fn draw(&self, ctx: &ScreenUpdateContext) {
        set_camera(&ctx.render_layers.world);
        for layer in self.layers().iter() {
            for (index, tile) in layer.iter().enumerate() {
                if let Some(tile) = tile.checked_sub(1) {
                    let x = (index % 48) as f32;
                    let y = (index / 48) as f32;

                    ctx.assets.tileset.draw_sprite(
                        x * 8.0,
                        y * 8.0,
                        (tile % 64) as f32,
                        (tile / 64) as f32,
                        None,
                    );
                }
            }
        }
    }
    fn from_file(data: &str) -> Self {
        Self {
            background: parse_tilemap_layer(&data, "Background"),
            walls: parse_tilemap_layer(&data, "Walls"),
            collision: parse_tilemap_layer(&data, "Collision"),
            detail: parse_tilemap_layer(&data, "Detail"),
            detail2: parse_tilemap_layer(&data, "Detail2"),
            special: parse_tilemap_layer(&data, "Special"),
        }
    }
}
fn parse_tilemap_layer(xml: &str, layer_name: &str) -> Tiles {
    let pattern = format!("name=\"{layer_name}\" ");
    let xml = xml
        .split_once(&pattern)
        .unwrap()
        .1
        .split_once("<data encoding=\"csv\">")
        .unwrap()
        .1
        .split_once("</data>")
        .unwrap()
        .0;
    let mut split = xml.split(',');
    let mut data: Tiles = Vec::new();
    while let Some(tile) = split.next() {
        let tile = tile.trim().parse::<usize>().unwrap();
        data.push(tile);
    }
    data
}
struct TilemapScreen {
    map: Map,
    entities: Vec<Box<dyn NonPlayerEntity>>,
}
impl TilemapScreen {
    fn new(file: &str, entities: Vec<Box<dyn NonPlayerEntity>>) -> Self {
        Self {
            map: Map::from_file(file),
            entities,
        }
    }
}
impl Screen for TilemapScreen {
    fn on_load(&mut self, ctx: ScreenUpdateContext) {
        self.map.draw(&ctx);
    }
    fn update(&mut self, mut ctx: ScreenUpdateContext) -> ScreenUpdateResult {
        for entity in self.entities.iter_mut() {
            entity.update(&self.map, &mut ctx);
        }
        ctx.player.update(&self.map);
        ScreenUpdateResult::Pass
    }
    fn draw(&mut self, ctx: ScreenUpdateContext) {
        for entity in self.entities.iter() {
            entity.draw(&ctx);
        }
        ctx.player.draw(&ctx);
        set_camera(&ctx.render_layers.ui);
        let width = 64.0;
        let height = 8.0;
        draw_rectangle(4.0, 4.0, width, height, BLACK);
        if ctx.player.health > 0.0 {
            draw_rectangle(
                4.0,
                4.0,
                ctx.player.health / ctx.player.max_health * width,
                height,
                GREEN,
            );
        }
    }
}
