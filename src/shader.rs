use std::fs::File;
use std::string::String;
use std::io::Read;

const vert_file_ext: &'static str = ".vert";
const frag_file_ext: &'static str = ".frag";

pub struct Shader {
    pub vertex: String,
    pub fragment: String,
}

impl Shader {
    pub fn new(filename: &str) -> Shader {
        let vert_shader_file: String = filename.to_string() + vert_file_ext;
        let frag_shader_file: String = filename.to_string() + frag_file_ext;
        let mut vert_shader = String::new();
        let mut frag_shader = String::new();

        match File::open(vert_shader_file) {
            Ok(mut file) => {
                file.read_to_string(&mut vert_shader);
            },
            Err(e) => panic!(e)
        }

        match File::open(frag_shader_file) {
            Ok(mut file) => {
                file.read_to_string(&mut frag_shader);
            },
            Err(e) => panic!(e)
        }

        Shader {
            vertex: vert_shader,
            fragment: frag_shader,
        }
    }
}
