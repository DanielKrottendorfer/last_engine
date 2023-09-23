use std::cell::RefCell;

use gl::DEBUG_TYPE_ERROR;
use std::ffi::c_void;

extern "system" fn message_callback(
    _source: u32,
    type_: u32,
    _id: u32,
    _severity: u32,
    _length: i32,
    message: *const i8,
    _userParam: *mut c_void,
) {
    unsafe {
        if let Some(m) = std::ffi::CStr::from_ptr(message).to_str().ok() {
            if type_ == DEBUG_TYPE_ERROR {
                println!("error :{}", m);
            } else {
                println!("warning: {}", m);
            }
        } else {
            println!("error :message error");
        }
    }
}

pub fn setup_debug() {
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(message_callback), std::ptr::null());
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
    }
}

#[derive(PartialEq)]
enum QState {
    Ready,
    Running,
    Ended,
}

pub struct Query {
    id: u32,
    state: RefCell<QState>,
}

impl Query {
    pub fn new() -> Self {
        let mut id = 0;

        unsafe {
            gl::GenQueries(1, &mut id);
        }

        Self {
            id,
            state: RefCell::new(QState::Ready),
        }
    }

    pub fn start(&self) {
        let mut state = self.state.try_borrow_mut().ok().unwrap();
        if *state == QState::Ready {
            unsafe {
                gl::BeginQuery(gl::TIME_ELAPSED, self.id);
            }
            *state = QState::Running;
        }
    }

    pub fn end(&self) {
        let mut state = self.state.try_borrow_mut().ok().unwrap();
        if *state == QState::Running {
            unsafe {
                gl::EndQuery(gl::TIME_ELAPSED);
            }
            *state = QState::Ended;
        }
    }

    pub fn get_time(&self) -> Option<u32> {
        let mut state = self.state.try_borrow_mut().ok().unwrap();
        if *state == QState::Ended {
            let mut temp = 0;
            unsafe {
                gl::GetQueryObjectuiv(self.id, gl::QUERY_RESULT_NO_WAIT, &mut temp);
            }
            if temp > 0 {
                *state = QState::Ready;
                Some(temp)
            } else {
                None
            }
        } else {
            None
        }
    }
}
