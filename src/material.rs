use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],  // Cambiado de 2 a 4 para incluir reflectividad y transparencia
    pub refractive_index: f32, // Índice de refracción
    pub transparency: f32, // Transparencia del material
}

impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 4], refractive_index: f32, transparency: f32) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            transparency,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 1.0,
            transparency: 0.0,
        }
    }
}
