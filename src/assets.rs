use asefile::AsepriteFile;
use image::EncodableLayout;
use macroquad::prelude::*;

/// An immutable collection of all the game's assets.
///
/// Please dont mutate.
pub struct Assets {
    pub tileset: Spritesheet,
}
impl Assets {
    pub fn load() -> Self {
        Self {
            tileset: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/tileset.ase"), None),
                16.0,
            ),
        }
    }
}

pub struct Spritesheet {
    pub texture: Texture2D,
    pub sprite_size: f32,
}
impl Spritesheet {
    pub fn new(texture: Texture2D, sprite_size: f32) -> Self {
        Self {
            texture,
            sprite_size,
        }
    }
    pub fn draw_sprite(
        &self,
        screen_x: f32,
        screen_y: f32,
        tile_x: f32,
        tile_y: f32,
        params: Option<&DrawTextureParams>,
    ) {
        let mut p = params.cloned().unwrap_or(DrawTextureParams::default());
        p.dest_size = p
            .dest_size
            .or(Some(Vec2::new(self.sprite_size, self.sprite_size)));
        p.source = p.source.or(Some(Rect {
            x: tile_x * self.sprite_size,
            y: tile_y * self.sprite_size,
            w: self.sprite_size,
            h: self.sprite_size,
        }));
        draw_texture_ex(&self.texture, screen_x, screen_y, WHITE, p);
    }
}
fn load_ase_texture(bytes: &[u8], layer: Option<u32>) -> Texture2D {
    let img = AsepriteFile::read(bytes).unwrap();
    let img = if let Some(layer) = layer {
        img.layer(layer).frame(0).image()
    } else {
        img.frame(0).image()
    };
    let new = Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.as_bytes().to_vec(),
    };
    let texture = Texture2D::from_image(&new);
    texture.set_filter(FilterMode::Nearest);
    texture
}
pub struct Animation {
    frames: Vec<(Texture2D, u32)>,
    pub total_length: u32,
}
impl Animation {
    pub fn from_file(bytes: &[u8]) -> Self {
        let ase = AsepriteFile::read(bytes).unwrap();
        let mut frames = Vec::new();
        let mut total_length = 0;
        for index in 0..ase.num_frames() {
            let frame = ase.frame(index);
            let img = frame.image();
            let new = Image {
                width: img.width() as u16,
                height: img.height() as u16,
                bytes: img.as_bytes().to_vec(),
            };
            let duration = frame.duration();
            total_length += duration;
            let texture = Texture2D::from_image(&new);
            frames.push((texture, duration));
        }
        Self {
            frames,
            total_length,
        }
    }
    pub fn get_at_time(&self, mut time: u32) -> &Texture2D {
        time %= self.total_length;
        for (texture, length) in self.frames.iter() {
            if time >= *length {
                time -= length;
            } else {
                return texture;
            }
        }
        panic!()
    }
}
