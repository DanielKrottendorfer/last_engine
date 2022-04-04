#[allow(dead_code)]
pub mod geometry;
pub mod shader;

pub mod rendertarget;

pub mod loader;

pub struct Texture(u32);
  
impl Texture {
    pub fn new(t: u32) -> Self {
        Texture(t)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        #[cfg(not(feature = "debug_off"))]
        println!("texture cleanup {}", self.0);
        unsafe {
            gl::DeleteTextures(1, &self.0);
        }
    }
}
