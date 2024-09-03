use nalgebra_glm::Vec3;
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    // Función para calcular las coordenadas UV
    pub fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        // Calcular el vector desde el centro de la esfera hasta el punto de impacto
        let r = (point - self.center).normalize();

        // Calcular θ (theta) y φ (phi) usando coordenadas esféricas
        let theta = r.z.atan2(r.x);
        let phi = r.y.asin();

        // Convertir θ y φ a coordenadas UV
        let u = 0.5 + theta / (2.0 * std::f32::consts::PI);
        let v = 0.5 - phi / std::f32::consts::PI;

        (u, v)
    }
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let oc = ray_origin - self.center;

        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * oc.dot(ray_direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let distance = (-b - discriminant.sqrt()) / (2.0 * a);
            if distance > 0.0 {
                let point = ray_origin + ray_direction * distance; // Calcular el punto de impacto
                let normal = (point - self.center).normalize();    // Calcular la normal en el punto de impacto
                let (u, v) = self.get_uv(&point); // Obtener coordenadas UV
                return Intersect::new(point, normal, distance, self.material.clone(), u, v);
            }
        }

        Intersect::empty()
    }
}

