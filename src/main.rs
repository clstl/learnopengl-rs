#[macro_use]
extern crate glium;

use glium::Surface;
use glium::glutin::{Api};
use glium::glutin::Event::{Closed, KeyboardInput};
use glium::glutin::VirtualKeyCode;
use glium::glutin::GlRequest;


fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("learnopengl")
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync()
        .build_glium()
        .unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                Closed => return,
                KeyboardInput(_, _, key_code) => {
                    if let Some(key) = key_code {
                        match key {
                            VirtualKeyCode::Escape => return,
                            _ => ()
                        }
                    }
                }
                _ => ()
            }
        }
    }
}
