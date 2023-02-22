pub mod perlin_builder;

use nannou::image::{ImageBuffer, Rgba};
use nannou::prelude::*;
use nannou::wgpu::WithDeviceQueuePair;

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
    width: u32,
    height: u32,
    gen_funcs: Vec<fn(u32, u32, f64, u8, u32) -> ImageBuffer<Rgba<u8>, Vec<u8>>>,
    gen_func_index: usize,
}

fn event(_app: &App, model: &mut Model, _event: Event) {
    // Match the event against the possible events we care about.
    match _event {
        // If the window is closed, exit the program.
        Event::WindowEvent { simple: Some(KeyPressed(key)), .. } => {
            match key {
                Key::S => {
                    let random_prefix = random::<u8>();
                    _app.main_window().capture_frame(format!("perlin_s{}_o{}_{}.jpeg", model.scale.ceil(), model.octaves, random_prefix));
                },
                Key::R => {
                    model.seed = random::<u32>();
                    update_texture(_app, model);
                },
                Key::M => {
                    model.gen_func_index = (model.gen_func_index + 1) % model.gen_funcs.len();
                    update_texture(_app, model);
                },
                //if two keys are pressed at the same time
                Key::NumpadAdd => {
                    let keys = &_app.keys.down;
                    if keys.contains(&Key::O) {
                        // I did not cap but above 12 the changes are unsignificant.
                        model.octaves += 1;
                        update_texture(_app, model);
                    } else if keys.contains(&Key::Z) {
                        model.scale += model.scale * 0.2;
                        update_texture(_app, model);
                    } else if keys.contains(&Key::W) {
                        model.width += 100;
                        update_texture(_app, model);
                        _app.main_window().set_inner_size_pixels(model.width, model.height)
                    } else if keys.contains(&Key::H) {
                        model.height = model.height+100;
                        update_texture(_app, model);
                        _app.main_window().set_inner_size_pixels(model.width, model.height)
                    }
                },
                Key::NumpadSubtract => {
                    let keys = &_app.keys.down;
                    if keys.contains(&Key::O)  && model.octaves > 1{
                        model.octaves -= 1;
                        update_texture(_app, model);
                    } else if keys.contains(&Key::Z) {
                        model.scale -= model.scale * 0.2;
                        update_texture(_app, model);
                    } else if keys.contains(&Key::W) && model.width > 100{
                        model.width -= 100;
                        update_texture(_app, model);
                        _app.main_window().set_inner_size_pixels(model.width, model.height)
                    } else if keys.contains(&Key::H) && model.height > 100{
                        model.height -= 100;
                        update_texture(_app, model);
                        _app.main_window().set_inner_size_pixels(model.width, model.height)
                    }
                },
                _ => (),
            }
        },
        _ => (),
    }
}

fn update_texture(app: &App, model: &mut Model) {
    let texture = new_texture(app, model);
    model.texture = Some(texture);
}

fn new_texture(app: &App, model: &mut Model) -> wgpu::Texture {

    let img_buf = (model.gen_funcs[model.gen_func_index])(model.width, model.height, model.scale, model.octaves, model.seed);

    let usage = wgpu::TextureUsages::COPY_SRC |
        wgpu::TextureUsages::COPY_DST |
        wgpu::TextureUsages::RENDER_ATTACHMENT |
        wgpu::TextureUsages::TEXTURE_BINDING;

    app.with_device_queue_pair(|device, queue| {
        let texture = wgpu::Texture::load_from_image_buffer(device, queue, usage, &img_buf);

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
        width: DIMENSIONS,
        height: DIMENSIONS,
        gen_funcs: vec![perlin_builder::terrain_generation::generate_terrain_map, perlin_builder::heightmap_generation::generate_height_map],
        gen_func_index: 0,
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
