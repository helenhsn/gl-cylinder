use std::{mem::{size_of_val, size_of}};
use glfw::{Action, Context, Key, ffi::{glfwGetTime}};
use cgmath::{Vector3, Matrix4, Rad, Deg, SquareMatrix, Vector2};

mod camera;
use camera::{Camera, Direction};

mod shader;
use shader::Shader;

mod cylinder;
use cylinder::Cylinder;

use crate::cylinder::Vertex;


pub fn upload_data<T>(buffer_type: gl::types::GLenum, data: &[T], usage: gl::types::GLenum) {
    unsafe {
        gl::BufferData(
            buffer_type, // target
            (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
            data.as_ptr() as *const gl::types::GLvoid, // pointer to data
            usage, // usage
        );
    }
}

fn main() {

    let resolution = Vector2{x:1000.0 , y:800.0}; 
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(resolution[0] as u32, resolution[1] as u32, "OpenGL Test", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    

    // camera definition & settings
    let mut camera: Camera = Camera::new(Vector3::new(0., 3., 5.), Vector3::new(0., 1., 0.), 2.5, 0.1, 5.2);
    window.set_key_polling(true);
    let mut delta_time: f64 = 0.;
    let mut last_frame_time: f64 = 0.;
    let mut mouse_x = 0.;
    let mut mouse_y = 0.;

    //setting up our vertices of our triangle (in NDC coordinates)
    let cyl:Cylinder = Cylinder::new(50, 1., 0.5);
    
    
    // setting up vbo (vertex buffer object) and vao (vertex array object)
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);

    }
    
    let mut vbo = 0;
    unsafe{
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // copying our vertices into the vbo (bounded to the GL_ARRAY_BUFFER)
        upload_data(gl::ARRAY_BUFFER, &cyl.get_vertices(), gl::STATIC_DRAW);

        // how OpenGL should interpret the data inside the vbo currently bounded to GL_ARRAY_BUFFER = position attribute
        gl::VertexAttribPointer(
            0, // location of the vertex attribute we want to configure
            3, // each position is vec3 so 3 values
            gl::FLOAT, // each value (coordinate) is a float
            gl::FALSE, 
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        // enabling the vertex attribut by giving its location
        gl::EnableVertexAttribArray(0);

        // normal attribute
        gl::VertexAttribPointer(
            1, // location of the vertex attribute we want to configure
            3, // each position is vec3 so 3 values
            gl::FLOAT, // each value (coordinate) is a float
            gl::FALSE, 
            size_of::<Vertex>().try_into().unwrap(),
            (std::mem::size_of::<[f32;3]>()) as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(1);
    }

    // setting up ebo (element buffer object)
    let mut ebo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        upload_data(gl::ELEMENT_ARRAY_BUFFER, &cyl.get_indices(), gl::STATIC_DRAW);
    
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    
    }



    // building the shader program
    let shader_pgrm: Shader = Shader::new("./src/shaders/vertex.glsl", "./src/shaders/frag.glsl");
    shader_pgrm.use_program();

    shader_pgrm.set_uniform_mat4("model", Matrix4::identity());
    shader_pgrm.set_uniform_3float("u_lightpos", Vector3 { x: 0.0, y: 5.0, z: -10.0 });
    // enabling depth test
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // render loop
    while !window.should_close() {
        
        // rendering / setting up the color
        unsafe {
            gl::ClearColor(0.2, 0.1, 0.3, 1.0);
            gl::DepthMask(1);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // camera origin uniform
        shader_pgrm.set_uniform_3float("u_camera", camera.get_origin());

        // per-frame time
        let current_frame_time = glfw.get_time();
        delta_time = current_frame_time - last_frame_time;
        last_frame_time = current_frame_time;
        
        // projection matrix
        let proj : Matrix4<f32> = cgmath::perspective(Rad::from(Deg(45.0)), (resolution[0] as f32)/(resolution[1] as f32), 0.1, 100.0);
        shader_pgrm.set_uniform_mat4("proj", proj);

        // view matrix
        let view = camera.get_view_matrix();
        shader_pgrm.set_uniform_mat4("view", view);


        // activate the shader program
        shader_pgrm.use_program();

        // updating all uniforms
        unsafe {
            // time
            shader_pgrm.set_uniform_float("u_time", glfwGetTime());
            // resolution
            shader_pgrm.set_uniform_2float("u_resolution", resolution);
        }

        // drawing triangle
        unsafe{
            // mode = primitive we would like to draw, first = starting index of the vao we'd like to draw
            // count = how many vertices are used in the EBO (3 per triangle, for ex)
            gl::DrawElements(gl::TRIANGLES, (cyl.get_indices().len()*3).try_into().unwrap(), gl::UNSIGNED_INT, 0 as *const _);
        }



        // poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },

                glfw::WindowEvent::Key(Key::J, _, Action::Repeat, _) => {
                    camera.process_keyboard(Direction::LEFT, delta_time as f32);
                },

                glfw::WindowEvent::Key(Key::L, _, Action::Repeat, _) => {
                    camera.process_keyboard(Direction::RIGHT, delta_time as f32);
                },

                glfw::WindowEvent::Key(Key::K, _, Action::Repeat, _) => {
                    camera.process_keyboard(Direction::BWD, delta_time as f32);
                },

                glfw::WindowEvent::Key(Key::I, _, Action::Repeat, _) => {
                    camera.process_keyboard(Direction::FWD, delta_time as f32);  
                },
                glfw::WindowEvent::CursorPos(mouse_x, mouse_y) => {
                    camera.process_mouse(mouse_x as f32, mouse_y as f32);
                }
                _ => {},
            }
        }

        // swap front and back buffers
        window.swap_buffers();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
    }
}

