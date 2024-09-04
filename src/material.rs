use crate::color::Color;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Texture {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.data[y * self.width + x]
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Arc<Self> {
        let img = image::open(path).expect("Failed to load texture");
        let img = img.to_rgb8();
        let (width, height) = img.dimensions();
        let data = img
            .pixels()
            .map(|p| Color::new(p[0], p[1], p[2]))
            .collect();
        Arc::new(Texture {
            data,
            width: width as usize,
            height: height as usize,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub transparency: f32,
    pub texture: Option<Arc<Texture>>, // Usa Arc para almacenar la textura
    pub has_texture: bool,              // Indicador de si el material tiene textura
}

impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        transparency: f32,
        texture: Option<Arc<Texture>>,
        has_texture: bool,
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            transparency,
            texture,
            has_texture,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 1.0,
            transparency: 0.0,
            texture: None,
            has_texture: false,
        }
    }

    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if self.has_texture {
            if let Some(texture) = &self.texture {
                let x = (u * (texture.width - 1) as f32) as usize;
                let y = (v * (texture.height - 1) as f32) as usize;
                texture.get_color(x, y)
            } else {
                self.diffuse
            }
        } else {
            self.diffuse
        }
    }
}
