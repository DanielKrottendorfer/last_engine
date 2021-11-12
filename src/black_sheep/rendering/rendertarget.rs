use gl;

pub struct RenderTarget {
    pub frame_buffer: u32,
    pub render_texture: u32,
    pub depth_render_buffer: u32,
}

impl RenderTarget {
    pub fn new(width: i32, height: i32) -> Self {
        let rt = Self {
            frame_buffer: gen_framebuffer(),
            render_texture: gen_empty_texture(width, height),
            depth_render_buffer: gen_depthbuffer(width, height),
        };
        unsafe {
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, rt.render_texture, 0);

            gl::DrawBuffer(gl::COLOR_ATTACHMENT0);
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("framebuffer not complete");
            }
        }
        rt
    }

    pub fn bind_framebuffer(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.frame_buffer);
        }
    }

    pub fn bind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.render_texture);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.render_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                0 as *const std::ffi::c_void,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.depth_render_buffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height);
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::RENDERBUFFER,
                self.depth_render_buffer,
            );
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.render_texture);
            gl::DeleteFramebuffers(1, &self.frame_buffer);
            gl::DeleteRenderbuffers(1, &self.depth_render_buffer);
        }
    }
}

fn gen_depthbuffer(width: i32, height: i32) -> u32 {
    let mut depthrenderbuffer = 0;
    unsafe {
        gl::GenRenderbuffers(1, &mut depthrenderbuffer);
        gl::BindRenderbuffer(gl::RENDERBUFFER, depthrenderbuffer);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height);
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            gl::RENDERBUFFER,
            depthrenderbuffer,
        );
    }
    depthrenderbuffer
}

pub fn unbind_framebuffer() {
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
}

pub fn gen_framebuffer() -> u32 {
    let mut buffer_name = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut buffer_name);
        gl::BindFramebuffer(gl::FRAMEBUFFER, buffer_name);
    }
    buffer_name
}

pub fn gen_empty_texture(width: i32, height: i32) -> u32 {
    let mut texture_name = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_name);
        gl::BindTexture(gl::TEXTURE_2D, texture_name);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width,
            height,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            0 as *const std::ffi::c_void,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }
    texture_name
}

impl Drop for RenderTarget {
    fn drop(&mut self) {
        #[cfg(not(feature = "debug_off"))]
        println!(
            "RenderTarget cleanup t: {}, fb: {}, dp: {}",
            self.render_texture, self.frame_buffer, self.depth_render_buffer
        );
        self.cleanup();
    }
}
