use nannou::image::{ImageBuffer, Rgba};
use nannou::prelude::map_range;
use crate::perlin_builder;

pub (crate) fn generate_terrain_map(width: u32, heigth: u32, scale: f64, octaves: u8, seed: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    perlin_builder::generate_texture(map_heigth_to_terrain_color, width, heigth, scale, octaves, seed)
}

fn map_heigth_to_terrain_color(height: f32) -> Rgba<u8> {
    let layer = TerrainLayer::get_layer(&height);
    layer.get_color(&height)
}

enum TerrainLayer {
    Water,
    Grove,
    Grass,
    Rock,
    Snow,
}

impl TerrainLayer {
    fn get_height(&self) -> f32 {
        match self {
            TerrainLayer::Water => -35.0,
            TerrainLayer::Grove => -10.0,
            TerrainLayer::Grass => 40.0,
            TerrainLayer::Rock => 70.0,
            TerrainLayer::Snow => 100.0,
        }
    }

    fn get_layer(&height: &f32) -> TerrainLayer {
        if height < TerrainLayer::Water.get_height() {
            TerrainLayer::Water
        } else if height < TerrainLayer::Grove.get_height() {
            TerrainLayer::Grove
        } else if height < TerrainLayer::Grass.get_height() {
            TerrainLayer::Grass
        } else if height < TerrainLayer::Rock.get_height() {
            TerrainLayer::Rock
        } else {
            TerrainLayer::Snow
        }
    }

    fn get_previous_layer(&self) -> Option<TerrainLayer> {
        match self {
            TerrainLayer::Water => None,
            TerrainLayer::Grove => Some(TerrainLayer::Water),
            TerrainLayer::Grass => Some(TerrainLayer::Grove),
            TerrainLayer::Rock => Some(TerrainLayer::Grass),
            TerrainLayer::Snow => Some(TerrainLayer::Rock),
        }
    }

    fn get_plage(&self) -> (f32, f32) {
        let height = self.get_height();
        let previous_height = match self.get_previous_layer() {
            Some(layer) => layer.get_height(),
            None => -100.
        };

        (previous_height, height)
    }

    fn get_color(&self, &height: &f32) -> Rgba<u8> {
        let (min_height, max_height) = self.get_plage();

        // Normalize the height to 0-255 to ease the color calculation.
        let norm_height = map_range(height as f32, min_height, max_height, 0.0, 255.0);

        match self {
            // This whole part, should be replaced by something easier to tweak.
            TerrainLayer::Water => Rgba([0, (255. - norm_height) as u8 / 3, norm_height as u8,255]),
            TerrainLayer::Grove => Rgba([(255. - norm_height) as u8 / 3, 128, 30 + (255.-norm_height) as u8 / 6, 255]),
            TerrainLayer::Grass => Rgba([70 + (norm_height / 3.5) as u8, 160 - norm_height as u8 / 8, 50 + (norm_height / 4.) as u8, 255]),
            TerrainLayer::Rock => Rgba([75 + norm_height as u8 / 2, 75 + norm_height as u8 / 2, 100 + norm_height as u8 / 2,255]),
            TerrainLayer::Snow => Rgba([255, 255, 255, 255])
        }
    }
}
