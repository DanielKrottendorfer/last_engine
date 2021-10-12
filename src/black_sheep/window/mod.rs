use sdl2::{EventPump, event::Event, mouse::MouseUtil, video::{GLContext, Window}};

mod util;

pub struct SDLWindow {
    event_pump: EventPump,
    sdl_window: Window,
    gl_context: GLContext,
    mouse_util: MouseUtil,
}

impl SDLWindow {
    pub fn new() -> Self {
        let (event_pump, sdl_window, gl_context, mouse_util) = util::new_sdl_window_with_opengl_context();

        Self {
            event_pump,
            sdl_window,
            gl_context,
            mouse_util,
        }
    }
    
    pub fn poll_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }
}

