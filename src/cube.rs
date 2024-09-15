use nalgebra_glm::Vec3;
use crate::ray_intersect::{RayIntersect, Intersect};
use std::sync::Arc;
use crate::material::Material;

pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Arc<Material>,  // Cambia esto para aceptar Arc<Material>
}

impl Cube {
    pub fn new(center: Vec3, size: f32, material: Arc<Material>) -> Self {
        Cube {
            center,
            size,
            material,
        }
    }

    // Calcula el punto mínimo del cubo (esquina inferior izquierda)
    pub fn min(&self) -> Vec3 {
        self.center - Vec3::new(self.size / 2.0, self.size / 2.0, self.size / 2.0)
    }

    // Calcula el punto máximo del cubo (esquina superior derecha)
    pub fn max(&self) -> Vec3 {
        self.center + Vec3::new(self.size / 2.0, self.size / 2.0, self.size / 2.0)
    }
}

// Función para calcular las coordenadas UV
impl Cube {
    pub fn get_uv(&self, point: &Vec3, normal: &Vec3) -> (f32, f32) {
        let mut u = 0.0;
        let mut v = 0.0;

        let min = self.min();
        let max = self.max();

        // Front face
        if normal.z.abs() > 0.99 {
            u = (point.x - min.x) / (max.x - min.x);
            v = (point.y - min.y) / (max.y - min.y);
        }
        // Back face
        else if normal.z.abs() > 0.01 && normal.z < 0.0 {
            u = (max.x - point.x) / (max.x - min.x);
            v = (point.y - min.y) / (max.y - min.y);
        }
        // Left face
        else if normal.x.abs() > 0.99 && normal.x < 0.0 {
            u = (max.z - point.z) / (max.z - min.z);
            v = (point.y - min.y) / (max.y - min.y);
        }
        // Right face
        else if normal.x.abs() > 0.99 && normal.x > 0.0 {
            u = (point.z - min.z) / (max.z - min.z);
            v = (point.y - min.y) / (max.y - min.y);
        }
        // Top face
        else if normal.y.abs() > 0.99 && normal.y > 0.0 {
            u = (point.x - min.x) / (max.x - min.x);
            v = (max.z - point.z) / (max.z - min.z);
        }
        // Bottom face
        else if normal.y.abs() > 0.99 && normal.y < 0.0 {
            u = (point.x - min.x) / (max.x - min.x);
            v = (point.z - min.z) / (max.z - min.z);
        }

        (u, v)
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let t_min = (self.min() - ray_origin).component_div(ray_direction);
        let t_max = (self.max() - ray_origin).component_div(ray_direction);

        let t1 = t_min.zip_map(&t_max, |a, b| a.min(b));
        let t2 = t_min.zip_map(&t_max, |a, b| a.max(b));

        let t_near = t1.max();
        let t_far = t2.min();

        if t_near < t_far && t_far > 0.0 {
            let distance = t_near;
            let point = ray_origin + ray_direction * distance;

            // Determinar la normal utilizando el centro
            let mut normal = Vec3::new(0.0, 0.0, 0.0);

            let epsilon = 0.001; // Pequeña tolerancia para comparar valores cercanos
            let center_to_point = point - self.center;

            if (center_to_point.x.abs() - (self.max().x - self.center.x)).abs() < epsilon {
                normal.x = center_to_point.x.signum(); // Normal en el eje X
            } else if (center_to_point.x.abs() - (self.center.x - self.min().x)).abs() < epsilon {
                normal.x = -center_to_point.x.signum(); // Normal en el eje X (cara opuesta)
            } else if (center_to_point.y.abs() - (self.max().y - self.center.y)).abs() < epsilon {
                normal.y = center_to_point.y.signum(); // Normal en el eje Y
            } else if (center_to_point.y.abs() - (self.center.y - self.min().y)).abs() < epsilon {
                normal.y = -center_to_point.y.signum(); // Normal en el eje Y (cara opuesta)
            } else if (center_to_point.z.abs() - (self.max().z - self.center.z)).abs() < epsilon {
                normal.z = center_to_point.z.signum(); // Normal en el eje Z
            } else if (center_to_point.z.abs() - (self.center.z - self.min().z)).abs() < epsilon {
                normal.z = -center_to_point.z.signum(); // Normal en el eje Z (cara opuesta)
            }

            // Obtener coordenadas UV
            let (u, v) = self.get_uv(&point, &normal);

            return Intersect {
                distance,
                point,
                normal,
                material: (*self.material).clone(),  // Desreferencia Arc<Material>
                is_intersecting: true,
                u,
                v,
            };
        }

        // Si no hay intersección, devolvemos un Intersect vacío
        Intersect {
            is_intersecting: false,
            distance: 0.0,
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: (*self.material).clone(),  // Desreferencia Arc<Material>
            u: 0.0,
            v: 0.0,
        }
    }
}
