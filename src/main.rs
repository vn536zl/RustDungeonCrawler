extern crate glutin_window;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use graphics::{Context, Transformed};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};

use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonState, Key};
use piston::{ButtonEvent, RenderEvent, ResizeEvent, MouseCursorEvent, Window, WindowSettings};

use std::io::Cursor;
use image::io::Reader;
use image::{imageops, RgbaImage};
use image::imageops::FilterType;

type Color = [f32; 4];

const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
const WHITE: Color = [1.0; 4];

const START_SIZE: [f64; 2] = [512.0, 384.0];

const START_ROOM: &str = "src/assets/dungeon/door.png";
const RIGHT_TURN: &str = "src/assets/dungeon/rightTurn.png";
const DEAD_END: &str = "src/assets/dungeon/deadend.png";

struct Room {
    width: f64,
    height: f64,
    area: String,
}

impl Room {
    fn start(size: &[f64; 2]) -> Self {
        Room {
            width: size[0],
            height: size[1]/1.5,
            area: START_ROOM.to_string(),
        }
    }
    fn right(size: &[f64; 2]) -> Self {
        Room {
            width: size[0],
            height: size[1]/1.5,
            area: RIGHT_TURN.to_string(),
        }
    }
    fn end(size: &[f64; 2]) -> Self {
        Room {
            width: size[0],
            height: size[1]/1.5,
            area: DEAD_END.to_string(),
        }
    }
}

fn load_image(size: &[f64;2], image: &str) -> Texture {

    let dynamic = Reader::open(image).unwrap().decode().unwrap();
    let load = dynamic.as_rgba8();
    let img: RgbaImage = imageops::resize(
        load.unwrap(),
        size[0] as u32,
        size[1] as u32,
        FilterType::Nearest,
    );
    Texture::from_image(&img, &TextureSettings::new())
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("RustDungeonCrawler", START_SIZE).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not build window");
    let mut gl = GlGraphics::new(opengl);
    let mut size = START_SIZE;
    let mut mouse_pos = [0.0, 0.0];

    let mut events = Events::new(EventSettings::new());
    let image = graphics::Image::new();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear(WHITE, g);

                let start_room = Room::start(&size);

                let texture = &load_image(&[start_room.width, start_room.height], &start_room.area);
                image.draw(
                    texture,
                    &c.draw_state,
                    c.transform,
                    g
                );
            })

        }

        if let Some(s) = e.resize_args() {
            size = s.window_size;

        }

        if let Some(m) = e.mouse_cursor_args() {
            mouse_pos = m;

        }

        if let Some(b) = e.button_args() {
            if b.state == ButtonState::Press {

            }
        }
    }
}
