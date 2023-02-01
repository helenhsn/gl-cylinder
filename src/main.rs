use cgmath::{Deg, Matrix4, Rad, SquareMatrix, Vector2, Vector3};
use glfw::{ffi::glfwGetTime, Action, Context, Key, MouseButton};
use std::mem::{size_of, size_of_val};

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
            buffer_type,                                                        // target
            (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
            data.as_ptr() as *const gl::types::GLvoid,                          // pointer to data
            usage,                                                              // usage
        );
    }
}

fn main() {
    // indicates if we allow camera movement w/ mouse or not
    let mut camera_move = false;

    let resolution = Vector2 {
        x: 1000.0,
        y: 800.0,
    };
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(
            resolution[0] as u32,
            resolution[1] as u32,
            "CylindersLand",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // camera definition & settings
    let mut camera: Camera = Camera::new(Vector3::new(0., 0., 10.), 90.0, 0.0, 8.5, 0.1, 45.);
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);
    let mut delta_time: f64 = 0.;
    let mut last_frame_time: f64 = 0.;
    let mut current_mouse: Vector2<f64> = Vector2 { x: 0., y: 0. };
    let mut last_mouse: Option<Vector2<f64>> = None;

    // setting up our vertices of our triangle (in NDC coordinates) for cylinder object
    let cyl: Cylinder = Cylinder::new(10, 1., 0.5);

    // cylinder pos
    let cyl_positions = [
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(2.0, 5.0, -15.0),
        Vector3::new(-1.5, -2.2, -2.5),
        Vector3::new(-3.8, -2.0, -12.3),
        Vector3::new(2.4, -0.4, -3.5),
        Vector3::new(-1.7, 3.0, -7.5),
        Vector3::new(1.3, -2.0, -2.5),
        Vector3::new(1.5, 2.0, -2.5),
        Vector3::new(1.5, 0.2, -20.5),
        Vector3::new(-1.3, 1.0, -1.5),
    ];

    // setting up vbo (vertex buffer object) and vao (vertex array object)
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);
    }

    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // copying our vertices into the vbo (bounded to the GL_ARRAY_BUFFER)
        upload_data(gl::ARRAY_BUFFER, &cyl.get_vertices(), gl::STATIC_DRAW);

        // how OpenGL should interpret the data inside the vbo currently bounded to GL_ARRAY_BUFFER = position attribute
        gl::VertexAttribPointer(
            0,         // location of the vertex attribute we want to configure
            3,         // each position is vec3 so 3 values
            gl::FLOAT, // each value (coordinate) is a float
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        // enabling the vertex attribut by giving its location
        gl::EnableVertexAttribArray(0);

        // normal attribute
        gl::VertexAttribPointer(
            1,         // location of the vertex attribute we want to configure
            3,         // each position is vec3 so 3 values
            gl::FLOAT, // each value (coordinate) is a float
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            (std::mem::size_of::<[f32; 3]>()) as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(1);
    }

    // setting up ebo (element buffer object)
    let mut ebo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        upload_data(
            gl::ELEMENT_ARRAY_BUFFER,
            &cyl.get_indices(),
            gl::STATIC_DRAW,
        );

        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    // building the shader program
    let shader_pgrm: Shader = Shader::new("./src/shaders/vertex.glsl", "./src/shaders/frag.glsl");
    shader_pgrm.use_program();
    shader_pgrm.set_uniform_3float(
        "u_lightpos",
        Vector3 {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        },
    );
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
        let proj: Matrix4<f32> = cgmath::perspective(
            Rad::from(camera.get_zoom()),
            (resolution[0] as f32) / (resolution[1] as f32),
            0.1,
            100.0,
        );
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

        for cyl_pos in cyl_positions {
            shader_pgrm.set_uniform_mat4("model", Matrix4::from_translation(cyl_pos));
            unsafe {
                // mode = primitive we would like to draw, first = starting index of the vao we'd like to draw
                // count = how many vertices are used in the EBO (3 per triangle, for ex)
                gl::DrawElements(
                    gl::TRIANGLES,
                    (cyl.get_indices().len() * 3).try_into().unwrap(),
                    gl::UNSIGNED_INT,
                    0 as *const _,
                );
            }
        }
        // drawing triangle

        // poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }

                glfw::WindowEvent::Key(Key::J, _, Action::Press | Action::Repeat, _) => {
                    camera.process_keyboard(Direction::LEFT, delta_time as f32);
                }

                glfw::WindowEvent::Key(Key::L, _, Action::Press | Action::Repeat, _) => {
                    camera.process_keyboard(Direction::RIGHT, delta_time as f32);
                }

                glfw::WindowEvent::Key(Key::K, _, Action::Press | Action::Repeat, _) => {
                    camera.process_keyboard(Direction::BWD, delta_time as f32);
                }

                glfw::WindowEvent::Key(Key::I, _, Action::Press | Action::Repeat, _) => {
                    camera.process_keyboard(Direction::FWD, delta_time as f32);
                }

                glfw::WindowEvent::MouseButton(glfw::MouseButtonLeft, Action::Press, _) => {
                    camera_move = !camera_move;
                }
                glfw::WindowEvent::Scroll(_, y) => {
                    camera.process_scroll(y);
                }
                _ => {}
            }
        }

        if camera_move {
            (current_mouse[0], current_mouse[1]) = window.get_cursor_pos();
            match last_mouse {
                None => last_mouse = Some(current_mouse), // initialization
                _ => (),
            };
            // mouse update
            if (0. <= current_mouse[0] && current_mouse[0] <= resolution[0].into())
                && (0. <= current_mouse[1] && current_mouse[1] <= resolution[1].into())
            {
                let offset = current_mouse - last_mouse.expect("None mouse position");
                last_mouse = Some(current_mouse);
                camera.process_mouse(offset);
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
