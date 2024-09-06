use nalgebra_glm::Vec3;
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::material::Material;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub center: Vec3,  // Agregar el centro
    pub material: Material,
}

impl Cube {
    // Constructor para crear un nuevo cubo usando el centro y el tamaño
    pub fn new(center: Vec3, size: f32, material: Material) -> Self {
        let half_size = size / 2.0;
        let min = center - Vec3::new(half_size, half_size, half_size);
        let max = center + Vec3::new(half_size, half_size, half_size);
        
        Cube {
            min,
            max,
            center,
            material,
        }
    }

    // Función para calcular las coordenadas UV
    pub fn get_uv(&self, point: &Vec3, normal: &Vec3) -> (f32, f32) {
        if normal.x.abs() > 0.99 {
            let u = (point.z - self.min.z) / (self.max.z - self.min.z);
            let v = (point.y - self.min.y) / (self.max.y - self.min.y);
            return (u, v);
        } else if normal.y.abs() > 0.99 {
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (point.z - self.min.z) / (self.max.z - self.min.z);
            return (u, v);
        } else if normal.z.abs() > 0.99 {
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (point.y - self.min.y) / (self.max.y - self.min.y);
            return (u, v);
        }
        (0.0, 0.0)
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let t_min = (self.min - ray_origin).component_div(ray_direction);
        let t_max = (self.max - ray_origin).component_div(ray_direction);

        let t1 = t_min.zip_map(&t_max, |a, b| a.min(b));
        let t2 = t_min.zip_map(&t_max, |a, b| a.max(b));

        let t_near = t1.max();
        let t_far = t2.min();

        if t_near < t_far && t_far > 0.0 {
            let distance = t_near;
            let point = ray_origin + ray_direction * distance;

            // Determinar la normal
            let mut normal = if (point.x - self.min.x).abs() < 1e-3 {
                Vec3::new(-1.0, 0.0, 0.0)
            } else if (point.x - self.max.x).abs() < 1e-3 {
                Vec3::new(1.0, 0.0, 0.0)
            } else if (point.y - self.min.y).abs() < 1e-3 {
                Vec3::new(0.0, -1.0, 0.0)
            } else if (point.y - self.max.y).abs() < 1e-3 {
                Vec3::new(0.0, 1.0, 0.0)
            } else if (point.z - self.min.z).abs() < 1e-3 {
                Vec3::new(0.0, 0.0, -1.0)
            } else {
                Vec3::new(0.0, 0.0, 1.0)
            };

            // Verificar si la normal debe invertirse
            let direction_to_center = (self.center - point).normalize();
            if normal.dot(&direction_to_center) > 0.0 {
                normal = -normal;  // Invertir la normal si está "dentro" del cubo
            }

            let (u, v) = self.get_uv(&point, &normal);
            return Intersect::new(point, normal, distance, self.material.clone(), u, v);
        }

        Intersect::empty()
    }
}
