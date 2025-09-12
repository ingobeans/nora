use enum_iterator::Sequence;
use macroquad::prelude::*;
use struct_iterable::Iterable;

use crate::{
    assets::{Animation, AnimationID, Assets},
    entity::{HumanoidEnemy, NonPlayerEntity},
    graphics::{DrawCall, RenderLayers},
    player::Player,
    utils::*,
};

pub struct ScreenUpdateContext<'a> {
    pub player: &'a mut Player,
    pub render_layers: &'a mut RenderLayers,
}
pub enum ScreenUpdateResult {
    /// Does nothing special
    Pass,
    /// Requests change to a different screen
    ChangeScreen(ScreenID, usize),
}

#[expect(unused_variables)]
pub trait Screen {
    fn on_load(&mut self, ctx: ScreenUpdateContext, spawn_index: usize) {}
    fn update(&mut self, ctx: ScreenUpdateContext) -> ScreenUpdateResult {
        ScreenUpdateResult::Pass
    }
    fn draw(&mut self, ctx: ScreenUpdateContext) {}
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum ScreenID {
    Test,
    Street,
}
impl Into<usize> for ScreenID {
    fn into(self) -> usize {
        self as usize
    }
}

pub fn create_screen_registry() -> Registry<ScreenID, Box<dyn Screen>> {
    let f: Box<dyn Fn(ScreenID) -> Box<dyn Screen>> = Box::new(|id| match id {
        ScreenID::Test => Box::new(TilemapScreen::new(
            include_str!("../assets/screens/test.tmx"),
            vec![Box::new(HumanoidEnemy::new(
                Vec2::new(25.0, 17.0) * 8.0,
                AnimationID::PlayerSprint,
                0.4,
            ))],
            vec![(ScreenID::Test, 1), (ScreenID::Street, 0)],
        )),
        ScreenID::Street => Box::new(TilemapScreen::new(
            include_str!("../assets/screens/street.tmx"),
            vec![],
            vec![(ScreenID::Test, 1), (ScreenID::Test, 0)],
        )),
    });
    Registry::new(f)
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
            return 1;
        }
        if y >= 27 {
            return 1;
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
    pub fn find_special_tile(&self, tile_index: usize) -> Option<(usize, usize)> {
        for (i, tile) in self.special.iter().enumerate() {
            if *tile == tile_index + 1 {
                return Some((i % 48, i / 48));
            }
        }
        None
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
    fn draw(&self, ctx: &mut ScreenUpdateContext) {
        for layer in self.layers().iter() {
            for (index, tile) in layer.iter().enumerate() {
                if let Some(tile) = tile.checked_sub(1) {
                    let x = (index % 48) as f32;
                    let y = (index / 48) as f32;

                    ctx.render_layers.world.calls.push(DrawCall::Tileset(
                        (tile % 64) as f32,
                        (tile / 64) as f32,
                        x * 8.0,
                        y * 8.0,
                    ));
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
    linked_screens: Vec<(ScreenID, usize)>,
}
impl TilemapScreen {
    fn new(
        file: &str,
        entities: Vec<Box<dyn NonPlayerEntity>>,
        linked_screens: Vec<(ScreenID, usize)>,
    ) -> Self {
        Self {
            map: Map::from_file(file),
            entities,
            linked_screens,
        }
    }
}
impl Screen for TilemapScreen {
    fn on_load(&mut self, mut ctx: ScreenUpdateContext, spawn_index: usize) {
        self.map.draw(&mut ctx);
        if let Some((x, y)) = self.map.find_special_tile(7 + spawn_index) {
            ctx.player.pos = Vec2::new(x as f32 * 8.0, y as f32 * 8.0);
        }
    }
    fn update(&mut self, mut ctx: ScreenUpdateContext) -> ScreenUpdateResult {
        for entity in self.entities.iter_mut() {
            entity.update(&self.map, &mut ctx);
        }
        ctx.player.update(&self.map);

        // handle special tiles

        let tile_pos = (ctx.player.pos / 8.0).round();
        let tile = self.map.get_special_tile(tile_pos.x as _, tile_pos.y as _);
        if tile >= 4 && tile <= 7 {
            let l = self.linked_screens.len();
            if l <= tile - 4 {
                panic!(
                    "Attempt to load linked screen #{}, but there's only {} linked screens!",
                    tile - 4,
                    l
                );
            }
            let target = self.linked_screens[tile - 4];
            return ScreenUpdateResult::ChangeScreen(target.0, target.1);
        }
        ScreenUpdateResult::Pass
    }
    fn draw(&mut self, mut ctx: ScreenUpdateContext) {
        for layer in ctx.render_layers.get_redrawn() {
            layer.calls.push(DrawCall::Clear(BLACK.with_alpha(0.0)));
        }
        for entity in self.entities.iter() {
            entity.draw(&mut ctx);
        }
        ctx.player.draw(&mut ctx.render_layers.entities);

        let width = 64.0;
        let height = 8.0;
        ctx.render_layers
            .ui
            .calls
            .push(DrawCall::Rect(4.0, 4.0, width, height, BLACK));
        if ctx.player.health > 0.0 {
            ctx.render_layers.ui.calls.push(DrawCall::Rect(
                4.0,
                4.0,
                ctx.player.health / ctx.player.max_health * width,
                height,
                GREEN,
            ));
        }
    }
}
