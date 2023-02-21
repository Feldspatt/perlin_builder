pub mod noise_map;

use nannou::prelude::*;
use nannou::image::{DynamicImage};
use nannou::wgpu::WithDeviceQueuePair;
use noise_map::NoiseBuilder;

const DIMENSIONS: u32 = 1024;
const SCALE: f64 = 4.0;
const OCTAVES: u8 = 12;

fn main() {
    nannou::app(model)
        .event(event)
        .run();
}

struct Model {
    texture: Option<wgpu::Texture>,
    seed: u32,
    scale: f64,
    octaves: u8,
    dimensions: u32,
}

fn event(_app: &App, model: &mut Model, _event: Event) {
    // Match the event against the possible events we care about.
    match _event {
        // If the window is closed, exit the program.
        Event::WindowEvent { simple: Some(KeyPressed(key)), .. } => {
            match key {
                Key::N => {
                    let texture = new_texture(_app, model);
                    model.texture = Some(texture);
                }
                Key::S => {
                    let random_prefix = random::<u8>();
                    _app.main_window().capture_frame(format!("perlin_s{}_{}_{}.jpeg", model.scale, model.octaves, random_prefix));
                },
                _ => (),
            }
        },
        _ => (),
    }
}


fn new_texture(app: &App, model: &mut Model) -> wgpu::Texture {
    let random_seed = random::<u32>();
    model.seed = random_seed;

    let img_buf = NoiseBuilder::generate_rgb_image(model.dimensions, model.scale, model.octaves, model.seed);

    let gray_image = DynamicImage::ImageRgb8(img_buf);

    let usage = wgpu::TextureUsages::COPY_SRC |
        wgpu::TextureUsages::COPY_DST |
        wgpu::TextureUsages::RENDER_ATTACHMENT |
        wgpu::TextureUsages::TEXTURE_BINDING;

    app.with_device_queue_pair(|device, queue| {
        let texture = wgpu::Texture::load_from_image(device, queue, usage, &gray_image);

        texture
    })
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .view(view)
        .size_pixels(DIMENSIONS, DIMENSIONS)
        .build()
        .unwrap();

    let mut model = Model {
        texture: None, //new_texture(app, model),
        seed: random::<u32>(),
        scale: SCALE,
        octaves: OCTAVES,
        dimensions: DIMENSIONS,
    };

    let texture = new_texture(app, &mut model);

    model.texture = Some(texture);
    model
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    draw.texture(&model.texture.as_ref().unwrap());


    draw.to_frame(app, &frame).unwrap();
}
