use sdl2::{
    event::Event,
    mouse::MouseUtil,
    video::{GLContext, Window},
    EventPump,
};

pub mod window_util;

pub struct SDLWindow {
    event_pump: EventPump,
    window: Window,
    _gl_context: GLContext,
    _mouse_util: MouseUtil,
}

impl SDLWindow {
    pub fn new() -> Self {
        let (event_pump, window, _gl_context, _mouse_util) =
            window_util::new_sdl_window_with_opengl_context();

        Self {
            event_pump,
            window,
            _gl_context,
            _mouse_util,
        }
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }

    pub fn swap(&self) {
        self.window.gl_swap_window();
    }

    pub fn capture_mouse(&mut self) {
        self._mouse_util.show_cursor(false);
        self._mouse_util.set_relative_mouse_mode(true);
    }
    pub fn release_mouse(&mut self) {
        self._mouse_util.show_cursor(true);
        self._mouse_util.set_relative_mouse_mode(false);
    }

}

impl Drop for SDLWindow {
    fn drop(&mut self) {
        #[cfg(not(feature = "debug_off"))]
        println!("window cleanup");
    }
}
