extern crate glutin_window;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use graphics::color::BLACK;
use graphics::Context;
use piston::WindowSettings;
use opengl_graphics::{GlGraphics, OpenGL};

use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, ButtonState, Key};
use piston::{ButtonEvent, RenderEvent};

fn render(c: Context, g: &mut GlGraphics) {
    graphics::clear(BLACK, g);


}

fn main() {
    let opengl = OpenGL::V4_5;
    let settings = WindowSettings::new("RustDungeonCrawler", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not build window");
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                render(c, g);
            })

        }
    }
}
