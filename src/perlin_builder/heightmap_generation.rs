use nannou::image::{ImageBuffer, Rgba};
use nannou::prelude::map_range;
use crate::perlin_builder;

pub (crate) fn generate_height_map(width: u32, height: u32, scale: f64, octaves: u8, seed: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    perlin_builder::generate_texture(map_height_to_grayscale, width, height, scale, octaves, seed)
}

fn map_height_to_grayscale(height: f32) -> Rgba<u8> {
    let norm_height = map_range(height as f32, -100.0, 100.0, 0.0, 255.0);
    Rgba([norm_height as u8, norm_height as u8, norm_height as u8, 255])
}