
pub fn new_sdl_window_with_opengl_context() -> (
    sdl2::EventPump,
    sdl2::video::Window,
    sdl2::video::GLContext,
    sdl2::mouse::MouseUtil,
) {
    let sdl_context = sdl2::init().unwrap();
    let video_context = sdl_context.video().unwrap();

    video_context.gl_attr().set_context_minor_version(4);
    video_context.gl_attr().set_context_minor_version(6);

    video_context.gl_attr().set_double_buffer(true);
    video_context.gl_attr().set_depth_size(24);

    let mouse = sdl_context.mouse();

    let sdl_window = {
        video_context
            .window("yarge", 1200, 800)
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .unwrap()
    };

    let sdl_gl = sdl_window.gl_create_context().unwrap();
    gl::load_with(|symbol| video_context.gl_get_proc_address(symbol) as *const _);

    // VSYNC Setting => video_context.gl_set_swap_interval(0).unwrap();
    // video_context.gl_set_swap_interval(0).unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    (event_pump, sdl_window, sdl_gl, mouse)
}