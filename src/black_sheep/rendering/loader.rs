

use imgui::FontAtlasTexture;

use super::{
    Texture,
};

pub fn load_texture_from_path(path: &str) -> Option<Texture> {
    use image::io::Reader as ImageReader;

    let im = match ImageReader::open(path) {
        Ok(im) => im.decode().unwrap().flipv(),
        Err(_) => return None,
    };

    let im = match im.flipv() {
        image::DynamicImage::ImageRgba8(img) => img,
        img => img.to_rgba8(),
    };

    let dim = im.dimensions();

    Some(gen_texture(
        im.as_ptr() as *mut std::ffi::c_void,
        (dim.0 as i32, dim.1 as i32),
        true,
    ))
}

pub fn gen_texture(data: *mut std::ffi::c_void, dim: (i32, i32), mipmap: bool) -> Texture {
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            dim.0,
            dim.1,
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
    Texture::new(texture)
}

pub fn load_texture_fontatlas(atlas: &FontAtlasTexture) -> Texture {
    // let im:ImageBuffer<Rgba<u8>,&[u8]> = ImageBuffer::from_raw(atlas.width, atlas.height, atlas.data).unwrap();
    // im.save("./image.png").unwrap();
    gen_texture(
        atlas.data.as_ptr() as *mut std::ffi::c_void,
        (atlas.width as i32, atlas.height as i32),
        false,
    )
}
