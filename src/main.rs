extern crate sdl2;

mod highscores;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use highscores::{save_highscores_and_lines, load_highscores_and_lines};

const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) =
        texture_creator.create_texture_target(None, size, size) {
            canvas.with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Green =>
                        texture.set_draw_color(Color::RGB(0, 255, 0)),
                    TextureColor::Blue =>
                        texture.set_draw_color(Color::RGB(0, 0, 255)),
                }
                texture.clear();
            }).expect("Failed to color a texture");
            Some(square_texture)
        } else {
            None
        }
}

pub fn main() {
    let highscores = [15, 25, 24, 24];
    let number_of_lines = [20, 19, 21, 20];
    save_highscores_and_lines(&highscores, &number_of_lines);
    println!("{:?}", load_highscores_and_lines());

    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video()
        .expect("Couldn't get SDL video subsystem");

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let green_square = create_texture_rect(&mut canvas,
            &texture_creator, TextureColor::Green, TEXTURE_SIZE)
        .expect("Failed to create a texture");
    let blue_square = create_texture_rect(&mut canvas,
            &texture_creator, TextureColor::Blue, TEXTURE_SIZE)
        .expect("Failed to create a texture");

    sdl2::image::init(INIT_PNG | INIT_JPG)
        .expect("Couldn't initialize image context");

    let image_texture = texture_creator.load_texture("assets/cat.jpg")
        .expect("Couldn't load image");

    let mut event_pump = sdl_context.event_pump()
        .expect("Failed to get SDL event pump");

    let start_time = SystemTime::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0)); // red
        canvas.clear();

        canvas.copy(&image_texture, None, None).expect("Render failed");

        let square_texture =
            if start_time.elapsed().expect("Elapsed fail").as_secs() % 2 == 0 {
                &green_square
            } else {
                &blue_square
            };

        canvas.copy(square_texture,
                None,
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldn't copy texture into window");

        canvas.present();
        
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
