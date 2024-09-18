use nalgebra_glm::{Vec3, Vec4};
use crate::camera::Camera;

pub struct Frustum {
    planes: [Vec4; 6], // Seis planos del frustum: left, right, top, bottom, near, far
}

impl Frustum {
    // Crear un nuevo Frustum basado en la cámara
    pub fn new(camera: &Camera) -> Self {
        // Obtener la matriz de vista/proyección de la cámara
        let view_proj = camera.get_view_projection_matrix();

        // Extraer los planos del frustum
        let left = view_proj.column(3) + view_proj.column(0);
        let right = view_proj.column(3) - view_proj.column(0);
        let top = view_proj.column(3) - view_proj.column(1);
        let bottom = view_proj.column(3) + view_proj.column(1);
        let near = view_proj.column(3) + view_proj.column(2);
        let far = view_proj.column(3) - view_proj.column(2);

        Frustum {
            planes: [left, right, top, bottom, near, far],
        }
    }

    // Verifica si una esfera (definida por su centro y radio) está dentro del frustum
    pub fn is_sphere_in_frustum(&self, center: Vec3, radius: f32) -> bool {
        for plane in &self.planes {
            let distance = (plane.x * center.x + plane.y * center.y + plane.z * center.z + plane.w) * 19.0;
            if distance < -radius {
                return false; // Si está completamente fuera de uno de los planos, está fuera del frustum
            }
        }
        true // Si pasa todas las pruebas, está dentro del frustum
    }
}
