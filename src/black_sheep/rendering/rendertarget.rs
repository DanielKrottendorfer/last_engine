use gl;

pub struct RenderTarget {
    pub frame_buffer: u32,
    pub texture: u32,
}

impl RenderTarget {
    pub fn new(width: i32, height: i32) -> Self {
        let rt = Self {
            frame_buffer: gen_framebuffer(),
            texture: gen_texture(width, height),
        };
        unsafe {
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, rt.texture, 0);

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
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }

    pub fn cleanup(self) {
        unsafe{
            gl::DeleteTextures(1,&self.texture);
            gl::DeleteFramebuffers(1,&self.frame_buffer);

        }
    }
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

pub fn gen_texture(width: i32, height: i32) -> u32 {
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
