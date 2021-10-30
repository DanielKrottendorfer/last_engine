use cgmath::{Vector2, Vector3};
use imgui::FontAtlasTexture;
use std::path::Path;

pub fn load_texture_from_path(path: &str) -> Option<u32> {
    use image::io::Reader as ImageReader;

    let im = match ImageReader::open(path) {
        Ok(im) => im.decode().unwrap().flipv(),
        Err(_) => return None,
    };

    let im = match im {
        image::DynamicImage::ImageRgba8(img) => img,
        img => img.to_rgba8(),
    };

    let dim = im.dimensions();

    Some(gen_texture(im.as_ptr() as *mut std::ffi::c_void, dim, true))
}

pub fn gen_texture(data: *mut std::ffi::c_void, dim: (u32, u32), mipmap: bool) -> u32 {
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            dim.0 as i32,
            dim.1 as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data,
        );

        if mipmap {
            // ... nice trilinear filtering ...
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            // ... which requires mipmaps. Generate them automatically.
            gl::GenerateMipmap(gl::TEXTURE_2D);
        } else {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        }
    }
    texture
}

pub fn load_texture_fontatlas(atlas: &FontAtlasTexture) -> u32 {
    // let im:ImageBuffer<Rgba<u8>,&[u8]> = ImageBuffer::from_raw(atlas.width, atlas.height, atlas.data).unwrap();
    // im.save("./image.png").unwrap();
    let dim = (atlas.width, atlas.height);
    gen_texture(atlas.data.as_ptr() as *mut std::ffi::c_void, dim, false)
}
