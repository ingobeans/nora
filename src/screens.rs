use struct_iterable::Iterable;

use crate::assets::Assets;

pub struct ScreenDrawContext<'a> {
    pub assets: &'a Assets,
}
pub struct ScreenUpdateContext {}
pub enum ScreenUpdateResult {
    /// Does nothing special
    Pass,
}

pub trait Screen {
    fn update(&mut self, ctx: ScreenUpdateContext) -> ScreenUpdateResult {
        ScreenUpdateResult::Pass
    }
    fn draw(&mut self, ctx: ScreenDrawContext) {}
}

pub struct ScreensRegistry {
    screens: Vec<Box<dyn Screen>>,
}
impl ScreensRegistry {
    pub fn new() -> Self {
        let mut screens = Vec::new();
        for id in enum_iterator::all::<ScreenID>() {
            screens.push(Self::load_screen_from_id(id));
        }

        Self { screens }
    }
    pub fn get(&self, id: ScreenID) -> &Box<dyn Screen> {
        &self.screens[id as usize]
    }
    pub fn get_mut(&mut self, id: ScreenID) -> &mut Box<dyn Screen> {
        &mut self.screens[id as usize]
    }
    fn load_screen_from_id(id: ScreenID) -> Box<dyn Screen> {
        match id {
            ScreenID::Test => Box::new(TestScreen::default()),
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
}
impl Map {
    fn draw(&self, assets: &Assets) {
        for (_, layer) in self.iter() {
            if let Some(layer) = layer.downcast_ref::<Tiles>() {
                for (index, tile) in layer.iter().enumerate() {
                    if let Some(tile) = tile.checked_sub(1) {
                        let x = (index % 24) as f32;
                        let y = (index / 24) as f32;
                        assets.tileset.draw_sprite(
                            x * 16.0,
                            y * 16.0,
                            (tile % 32) as f32,
                            (tile / 32) as f32,
                            None,
                        );
                    }
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum ScreenID {
    Test,
}
struct TestScreen {
    map: Map,
}
impl Screen for TestScreen {
    fn draw(&mut self, ctx: ScreenDrawContext) {
        self.map.draw(ctx.assets);
    }
}
impl Default for TestScreen {
    fn default() -> Self {
        Self {
            map: Map::from_file(include_str!("../assets/screens/street.tmx")),
        }
    }
}
