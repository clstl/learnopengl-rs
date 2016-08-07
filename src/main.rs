#[macro_use]
extern crate glium;
extern crate time;

use glium::glutin::{Api};
use glium::glutin::Event::{Closed, KeyboardInput};
use glium::glutin::VirtualKeyCode;
use glium::glutin::GlRequest;
use std::string::String;

mod shader;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

fn main() {
    use glium::{DisplayBuild, Surface};

    //Window
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("learnopengl")
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync()
        .build_glium()
        .unwrap();

    let vertex1 = Vertex { position: [0.5, 0.5, 0.0], color : [1.0, 0.0, 0.0]};
    let vertex2 = Vertex { position: [ 0.5,  -0.5, 0.0], color : [0.0, 1.0, 0.0] };
    let vertex3 = Vertex { position: [ -0.5, -0.5, 0.0], color : [0.0, 0.0, 1.0] };
    let vertex4 = Vertex { position: [ -0.5, 0.5, 0.0], color : [1.0, 1.0, 1.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices_list:[u32; 6] = [0,1,3,1,2,3];
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                      &indices_list).unwrap();


    let shader = shader::Shader::new(&String::from("tutorial"));
    let program = glium::Program::from_source(&display, shader.vertex.as_str(),
         shader.fragment.as_str(), None).unwrap();

    loop {
        let time_value = time::precise_time_s() as f32;
        let green: f32 = time_value.sin()/2.0f32 + 0.5f32;
        let our_color: [f32; 4] = [0.0,green,0.0,1.0];

        let mut target = display.draw();
        target.clear_color(0.1, 0.3, 0.3, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform!{ourColor: our_color},
            &Default::default()).unwrap();
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
