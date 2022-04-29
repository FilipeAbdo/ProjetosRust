use sdl2::keyboard::Keycode;
use std::{thread, time};

extern crate gl;
extern crate sdl2;

pub mod render_gl;

fn main() {
    #[warn(unused_variables)]
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 1920, 950)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    use std::ffi::CString;
    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    shader_program.set_used();

    let mut vao: gl::types::GLuint = 0;
    // let mut vao: gl::types::GLuint = 0;
  
    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.15, 0.15, 0.3, 1.0);
    }

    let duration: time::Duration = time::Duration::from_millis(10);
    
    let mut count: i16 = 0;

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    break 'main
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    if matches!(detect_pressing(Keycode::A, KeyStatus::DOWN),KeyEvent::Pressed) {
                        println!("Pressed A");
                    }
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::A), ..} => {
                    if matches!(detect_pressing(Keycode::A, KeyStatus::UP),KeyEvent::Pressed) {
                        println!("Pressed A");
                    }
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::B), ..} => println!("Keycode B"),
                _ => {},
            }

        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let angle_value: f32 = ((count as f32) * (2.0 * std::f32::consts::PI)) / 360.0;
        let radius: f32 = f32::powf(f32::powf( 0.5, 2.0) + f32::powf( 0.5, 2.0), 0.5);
        // print!("radius {} angle_value {} -> ", radius, angle_value);
        
        let p1_x:f32 = radius * (angle_value + 7.0 * std::f32::consts::PI / 6.0).cos();
        let p1_y:f32 = radius * (angle_value + 7.0 * std::f32::consts::PI / 6.0).sin();
        
        let p2_x:f32 = radius * (angle_value + -std::f32::consts::PI / 6.0).cos();
        let p2_y:f32 = radius * (angle_value + -std::f32::consts::PI / 6.0).sin();
        
        let p3_x:f32 = radius * (angle_value + std::f32::consts::PI / 2.0).cos();
        let p3_y:f32 = radius * (angle_value + std::f32::consts::PI / 2.0).sin();

        // println!("p1_x {}, p1_y {} --> p2_x {}, p2_y {} --> p3_x {}, p3_y {}", p1_x, p1_y, p2_x, p2_y, p3_x, p3_y);
        
        let move_vertices: Vec<f32> = vec![
            // positions      // colors
                p1_x, p1_y, 0.0,   1.0, 0.0, 0.0,    // right
                p2_x, p2_y, 0.0,   0.0, 1.0, 0.0,    // left
                p3_x, p3_y, 0.0,   0.0, 0.0, 1.0     // center
        ];
        
        vao = draw_triangle(move_vertices);

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }

        window.gl_swap_window();

        count = (count + 2)%360;
        // println!("Angle {:03}°", count);
        
        thread::sleep(duration);
    }
}

fn draw_triangle(vertices:Vec<f32>) -> gl::types::GLuint{
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }
    
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );

        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );
    
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    return vao;
}

struct KeyInfo{
    key:Option<Keycode>,
    status:KeyStatus,
}

#[derive(PartialEq)]
enum KeyStatus{
    DOWN,
    UP,
}

#[derive(PartialEq)]
enum KeyEvent{
    Pressed,
    Released,
    NotPressed,
}

impl Default for KeyEvent {
    fn default() -> Self {
        Self::Pressed
    }
}

fn detect_pressing(key:Keycode, up_down:KeyStatus) -> KeyEvent{
    static mut LAST_KEY:KeyInfo = KeyInfo{ key:None, status:KeyStatus::UP};
    let mut return_value:KeyEvent = KeyEvent::NotPressed;
    unsafe{
        if LAST_KEY.key == Some(key) && matches!(up_down, KeyStatus::UP) && matches!(LAST_KEY.status, KeyStatus::DOWN) {
            return_value = KeyEvent::Pressed;
        }else{
            return_value = KeyEvent::NotPressed;
        }
        LAST_KEY.key = Some(key);
        LAST_KEY.status = up_down;
    }
    return  return_value;
}