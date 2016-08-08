#[macro_use]
extern crate glium;
extern crate time;
extern crate cgmath;
extern crate image;

use std::string::String;

use glium::glutin::{Api};
use glium::glutin::Event::{Closed, KeyboardInput};
use glium::glutin::VirtualKeyCode;
use glium::glutin::GlRequest;

use cgmath::{Matrix4, Rad, Deg, Vector3};

use std::io::Cursor;

mod shader;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    texture: [f32; 2],
}

implement_vertex!(Vertex, position, texture);

fn main() {
    use glium::{DisplayBuild, Surface};

    //Texture loading
    let image = image::load(Cursor::new(&include_bytes!("../dev.png")[..]),
    image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

    //Window
    let display = glium::glutin::WindowBuilder::new()
    .with_dimensions(800, 800)
    .with_title("learnopengl")
    .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
    .with_multisampling(16)
    .with_depth_buffer(24)
    .with_vsync()
    .build_glium()
    .unwrap();

    // let vertex2 = Vertex { position: [ 0.5,  -0.5, 0.0], color : [0.0, 1.0, 0.0], texture: [1.0, 0.0] };
    // let vertex3 = Vertex { position: [ -0.5, -0.5, 0.0], color : [0.0, 0.0, 1.0], texture: [0.0, 0.0] };
    // let vertex4 = Vertex { position: [ -0.5, 0.5, 0.0], color : [1.0, 1.0, 1.0], texture: [0.0, 1.0] };
    let shape = vec![Vertex{position:[-0.5, -0.5, -0.5],texture:[ 0.0, 0.0]},
    Vertex{position:[0.5, -0.5, -0.5],texture:[   1.0, 0.0]},
    Vertex{position:[0.5,  0.5, -0.5],texture:[   1.0, 1.0]},
    Vertex{position:[0.5,  0.5, -0.5],texture:[   1.0, 1.0]},
    Vertex{position:[-0.5,  0.5, -0.5],texture:[   0.0, 1.0]},
    Vertex{position:[-0.5, -0.5, -0.5],texture:[   0.0, 0.0]},
    Vertex{position:[-0.5, -0.5,  0.5],texture:[   0.0, 0.0]},
    Vertex{position:[0.5, -0.5,  0.5],texture:[   1.0, 0.0]},
    Vertex{position:[0.5,  0.5,  0.5],texture:[   1.0, 1.0]},
    Vertex{position:[0.5,  0.5,  0.5],texture:[   1.0, 1.0]},
    Vertex{position:[-0.5,  0.5,  0.5],texture:[   0.0, 1.0]},
    Vertex{position:[-0.5, -0.5,  0.5],texture:[   0.0, 0.0]},
    Vertex{position:[-0.5,  0.5,  0.5],texture:[   1.0, 0.0]},
    Vertex{position:[-0.5,  0.5, -0.5],texture:[   1.0, 1.0]},
    Vertex{position:[-0.5, -0.5, -0.5],texture:[   0.0, 1.0]},
    Vertex{position:[-0.5, -0.5, -0.5],texture:[   0.0, 1.0]},
    Vertex{position:[-0.5, -0.5,  0.5],texture:[   0.0, 0.0]},
    Vertex{position:[-0.5,  0.5,  0.5],texture:[   1.0, 0.0]},
    Vertex{position:[0.5,  0.5,  0.5],texture:[   1.0, 0.0]},
    Vertex{position:[0.5,  0.5, -0.5],texture:[   1.0, 1.0]},
    Vertex{position:[0.5, -0.5, -0.5],texture:[   0.0, 1.0]},
    Vertex{position:[0.5, -0.5, -0.5],texture:[   0.0, 1.0]},
    Vertex{position:[0.5, -0.5,  0.5],texture:[   0.0, 0.0]},
    Vertex{position:[0.5,  0.5,  0.5],texture:[   1.0, 0.0]},
    Vertex{position:[-0.5, -0.5, -0.5],texture:[   0.0, 1.0]},
    Vertex{position:[0.5, -0.5, -0.5],texture:[   1.0, 1.0]},
    Vertex{position:[0.5, -0.5,  0.5],texture:[   1.0, 0.0]},
    Vertex{position:[0.5, -0.5,  0.5],texture:[   1.0, 0.0]},
    Vertex{position:[-0.5, -0.5,  0.5],texture:[   0.0, 0.0]},
    Vertex{position:[-0.5, -0.5, -0.5],texture:[   0.0, 1.0]},
    Vertex{position:[-0.5,  0.5, -0.5],texture:[   0.0, 1.0]},
    Vertex{position:[0.5,  0.5, -0.5],texture:[   1.0, 1.0]},
    Vertex{position:[0.5,  0.5,  0.5],texture:[   1.0, 0.0]},
    Vertex{position:[0.5,  0.5,  0.5],texture:[   1.0, 0.0]},
    Vertex{position:[-0.5,  0.5,  0.5],texture:[   0.0, 0.0]},
    Vertex{position:[-0.5,  0.5, -0.5],texture:[   0.0, 1.0]}];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // let indices_list:[u32; 6] = [0,1,3,1,2,3];
    // let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
    //                                   &indices_list).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    let shader = shader::Shader::new(&String::from("tutorial"));
    let program = glium::Program::from_source(&display, shader.vertex.as_str(),
    shader.fragment.as_str(), None).unwrap();

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    //List of geom
    let cubes = [( 0.0,  0.0,  0.0),
     ( 2.0,  5.0, -15.0),
     (-1.5, -2.2, -2.5),
     (-3.8, -2.0, -12.3),
     ( 2.4, -0.4, -3.5),
     (-1.7,  3.0, -7.5),
     ( 1.3, -2.0, -2.5),
     ( 1.5,  2.0, -2.5),
     ( 1.5,  0.2, -1.5),
     (-1.3,  1.0, -1.5)];
    //MVP
    loop {
        let time_value = time::precise_time_s() as f32;

        let projection: [[f32; 4]; 4] = cgmath::perspective(Rad::new(std::f32::consts::PI/4.0), 1.0, 0.1, 100.0).into();

        let view: [[f32; 4]; 4] = cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, -3.0)).into();
        let mut target = display.draw();

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };



        target.clear_color_and_depth((0.1, 0.3, 0.3, 1.0), 1.0);
        for (i, cube) in cubes.iter().enumerate() {

            let model: [[f32; 4]; 4] = (
                cgmath::Matrix4::from_translation(cgmath::Vector3::new(cube.0, cube.1, cube.2))*
                cgmath::Matrix4::from_angle_x(Rad::new(time_value*(i as f32)))
                *cgmath::Matrix4::from_angle_z(Rad::new(time_value*(i as f32)))

            ).into();

            let uniforms = uniform!{
                tex: &texture,
                model: model,
                view: view,
                projection: projection
            };

            target.draw(&vertex_buffer, &indices, &program, &uniforms,
                &params).unwrap();


        }
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
