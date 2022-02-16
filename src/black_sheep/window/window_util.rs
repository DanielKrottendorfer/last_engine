use crate::black_sheep::settings::INIT_WINDOW_SIZE;
use cgmath::Matrix4;

pub fn new_sdl_window_with_opengl_context() -> (
    sdl2::EventPump,
    sdl2::video::Window,
    sdl2::video::GLContext,
    sdl2::mouse::MouseUtil,
) {
    let sdl_context = sdl2::init().unwrap();
    let video_context = sdl_context.video().unwrap();

    video_context.gl_attr().set_context_minor_version(4);
    video_context.gl_attr().set_context_minor_version(5);

    video_context.gl_attr().set_double_buffer(true);
    video_context.gl_attr().set_depth_size(24);

    let mouse = sdl_context.mouse();

    let sdl_window = {
        video_context
            .window("spam", INIT_WINDOW_SIZE[0], INIT_WINDOW_SIZE[1])
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .unwrap()
    };

    let sdl_gl = sdl_window.gl_create_context().unwrap();
    gl::load_with(|symbol| video_context.gl_get_proc_address(symbol) as *const _);

    #[cfg(feature = "vsync_off")]
    video_context.gl_set_swap_interval(0).unwrap();

    //toggle_wiregrid();

    let event_pump = sdl_context.event_pump().unwrap();

    (event_pump, sdl_window, sdl_gl, mouse)
}

pub fn toggle_wiregrid() {
    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }
}

pub fn set_viewport(w: i32, h: i32) {
    unsafe {
        gl::Viewport(0, 0, w, h);
    }
}

pub fn clear_drawbuffer() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
    }
}

pub fn init_rendersetup() {
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendEquation(gl::FUNC_ADD);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
}

pub fn three_d_rendering_setup() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Disable(gl::SCISSOR_TEST);
    }
}

pub fn imgui_rendering_setup() {
    unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::Enable(gl::SCISSOR_TEST);
    }
}
