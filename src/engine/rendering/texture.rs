use image::{ImageReader};
use std::{ffi::c_void, path::Path};

/// A structure containing the corresponding texture ID OpenGL assigned to it.
/// The texture data does not need saving, as it becomes copied to GPU during loading.
pub struct Texture {
    /// The texture ID is None as long as the texture has not been (successfully) loaded.
    texture_id: Option<u32>
}

impl Texture {
    /// Returns an empty texture that can not be used for rendering.
    pub fn new() -> Texture {
        Texture {
            texture_id: None
        }
    }

    /// Loads the texture at the specified path into the struct.
    /// texture_id will remain None, if loading fails.
    pub fn load(&mut self, texture_path: String) {
        // Prepare the OpenGL texture
        let mut opengl_texture: u32 = 0;
        unsafe {
            // Safe, as opengl_texture always is a valid reference
            gl::GenTextures(1, &mut opengl_texture);
            gl::BindTexture(gl::TEXTURE_2D, opengl_texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }

        // Load texture as image from filesystem
        let mut image = match ImageReader::open(&Path::new(&texture_path)){
            Ok(i) => {i}
            Err(e) => {println!("Error loading texture {}: {}", texture_path, e); return;}
        };

        // Decode image
        let mut image = match image.decode() {
            Ok(i) => {i},
            Err(e) => {
                println!("Error decoding texture {}: {}", texture_path, e); return;
            }
        };

        // Apply necessary data transformationd
        image = image.flipv();
        let image_data = image.to_rgba8().into_raw();

        // Attach image to OpenGL texture
        unsafe {
            // Safe, because image has been matched and not consumed, image_data generation can not fail.
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                // Safe, because image_data is accessed at no other location, and gets copied to GPU here
                &image_data[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        self.texture_id = Some(opengl_texture);
    }

    /// Makes OpenGL assign the corresponding texture ID.
    pub fn bind(&self) {
        let texture_id = match self.texture_id {
            Some(i) => i,
            None => {println!("Attempted to bind not initialized Texture."); return;}
        };

        unsafe {
            // Safe as texture_id has been checked before
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }
    }
}