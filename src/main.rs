#[macro_use]
extern crate glium;

use glium::glutin::{Api};
use glium::glutin::Event::{Closed, KeyboardInput};
use glium::glutin::VirtualKeyCode;
use glium::glutin::GlRequest;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::{DisplayBuild, Surface};

    //Program
    let vertex_shader_src = r#"
    #version 330 core

    layout (location = 0) in vec3 position;

    void main()
    {
        gl_Position = vec4(position.x, position.y, position.z, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 330 core

    out vec4 color;

    void main()
    {
        color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }

    "#;


    //Window
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("learnopengl")
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync()
        .build_glium()
        .unwrap();

    //Triangle
    /*
    0.5f,  0.5f, 0.0f,  // Top Right
     0.5f, -0.5f, 0.0f,  // Bottom Right
    -0.5f, -0.5f, 0.0f,  // Bottom Left
    -0.5f,  0.5f, 0.0f   // Top Left
    */

    let vertex1 = Vertex { position: [0.5, 0.5, 0.0] };
    let vertex2 = Vertex { position: [ 0.5,  -0.5, 0.0] };
    let vertex3 = Vertex { position: [ -0.5, -0.5, 0.0] };
    let vertex4 = Vertex { position: [ -0.5, 0.5, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let indices_list:[u32; 6] = [0,1,3,1,2,3];
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                      &indices_list).unwrap();
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    loop {
        let mut target = display.draw();
        target.clear_color(0.1, 0.3, 0.3, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
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
