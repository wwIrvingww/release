use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 2], // Albedo (valores de reflectancia y difusión)
    pub reflectivity: f32, // Índice de reflexión
}

impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 2], reflectivity: f32) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            reflectivity,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0],
            reflectivity: 0.0,
        }
    }
}
