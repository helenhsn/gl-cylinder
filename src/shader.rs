use std::{fs::read_to_string, ffi::CString};
use gl::types::GLenum;
use cgmath::{Matrix4, Vector2, Vector3};
pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let shader_program: u32;
        unsafe {
            let vertex:u32 = Self::compile_shader(vertex_path, gl::VERTEX_SHADER);
            let fragment:u32 = Self::compile_shader(fragment_path, gl::FRAGMENT_SHADER);
            
            shader_program = gl::CreateProgram();
            assert_ne!(shader_program, 0);

            //attaching the shader to the program
            gl::AttachShader(shader_program, vertex);
            gl::AttachShader(shader_program, fragment);

            //linking them
            gl::LinkProgram(shader_program);
            Self::check_compile_errors(shader_program, "PROGRAM");

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }
        Self{id: shader_program}
    }

    fn compile_shader(shader_path: &str, shader_type: GLenum) -> u32 {
        let source_code = read_to_string(&shader_path).expect(format!("Couldn't read the file: {}", shader_path).as_str());
        //shader object
        let shader_id;
        let shader_type_str = match shader_type {
            gl::VERTEX_SHADER => "VERTEX",
            gl::FRAGMENT_SHADER => "FRAGMENT",
            _ => "",
        };
        unsafe {
            //creating the shader
            shader_id = gl::CreateShader(shader_type);
            //binding the source code
            gl::ShaderSource(
                shader_id, 
                1, 
                &(source_code.as_bytes().as_ptr().cast()),
                &(source_code.len().try_into().unwrap())
            );
            //compiling
            gl::CompileShader(shader_id);
        }
        Self::check_compile_errors(shader_id, shader_type_str);
        shader_id
    }

    pub fn use_program(&self) {
        unsafe{
            gl::UseProgram(self.id);
        }
    }

    fn check_compile_errors(id: u32, shader_type: &str) {
        let mut success:i32=0;
        let mut info_log: Vec<u8> = Vec::with_capacity(1024);
        let mut info_log_size: i32 = 0;
        match shader_type {
            "VERTEX" | "FRAGMENT" => { //check shader compile error
                unsafe {
                    gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
                    if success==0 {
                        gl::GetShaderInfoLog(
                            id,
                            1024, 
                            &mut info_log_size, 
                            info_log.as_mut_ptr().cast()
                        );
                        info_log.set_len(info_log_size.try_into().unwrap());
                        println!(
                            "ERROR::SHADER_COMPILATION_ERROR of type: {}",
                            String::from_utf8_lossy(&info_log)
                        );
                    }
                }
            },
            _ => { //check program compile error 
                unsafe {
                    gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
                    if success==0 {
                        gl::GetProgramInfoLog(
                            id, 
                            1024, 
                            &mut info_log_size, 
                            info_log.as_mut_ptr().cast()
                        );
                        //error was filled with a code in C so we have to tell Rust what error's length actually is (for Rust)
                        info_log.set_len(info_log_size.try_into().unwrap());
                        println!(
                            "ERROR::PROGRAM_LINKING_ERROR of type: {}",
                            String::from_utf8_lossy(&info_log)
                        );
                    }
                }
            },
        }
    }

    pub fn set_uniform_float(&self, uniform_name: &str, value: f64) {
        unsafe {
            let cname = CString::new(uniform_name).expect("CString::new failed");
            gl::Uniform1f(gl::GetUniformLocation(self.id, cname.as_ptr().cast()), value as f32);
        }
    }

    pub fn set_uniform_2float(&self, uniform_name: &str, value: Vector2<f32>) {
        unsafe {
            let cname = CString::new(uniform_name).expect("CString::new failed");
            gl::Uniform2fv(gl::GetUniformLocation(self.id, cname.as_ptr().cast()), 1, &value[0] as *const f32);
        }
    }
    
    pub fn set_uniform_mat4(&self, uniform_name: &str, value: Matrix4<f32>) {
        unsafe {
            let cname = CString::new(uniform_name).expect("CString::new failed");
            let loc = gl::GetUniformLocation(self.id, cname.as_ptr().cast());
            if loc != -1 {
                gl::UniformMatrix4fv(loc, 1, gl::FALSE, &value[0][0]);
            }
        }
    }

    pub fn set_uniform_3float(&self, uniform_name: &str, value: Vector3<f32>) {
        unsafe {
            let cname = CString::new(uniform_name).expect("CString::new failed");
            let loc = gl::GetUniformLocation(self.id, cname.as_ptr().cast());
            if loc != -1 {
                gl::Uniform3fv(loc, 1,  &value[0] as *const f32);
            }
        }
    }
}