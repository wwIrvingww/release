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
        let mut u = 0.0;
        let mut v = 0.0;
    
        // Front face
        if normal.z.abs() > 0.99 {
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
        }
        // Back face
        else if normal.z.abs() > 0.01 && normal.z < 0.0 {
            u = (self.max.x - point.x) / (self.max.x - self.min.x);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
        }
        // Left face
        else if normal.x.abs() > 0.99 && normal.x < 0.0 {
            u = (self.max.z - point.z) / (self.max.z - self.min.z);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
        }
        // Right face
        else if normal.x.abs() > 0.99 && normal.x > 0.0 {
            u = (point.z - self.min.z) / (self.max.z - self.min.z);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
        }
        // Top face
        else if normal.y.abs() > 0.99 && normal.y > 0.0 {
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (self.max.z - point.z) / (self.max.z - self.min.z);
        }
        // Bottom face
        else if normal.y.abs() > 0.99 && normal.y < 0.0 {
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (point.z - self.min.z) / (self.max.z - self.min.z);
        }
    
        (u, v)
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

            // Determinar la normal utilizando el centro
            // Determinar la normal utilizando el centro
            let mut normal = Vec3::new(0.0, 0.0, 0.0);

            let epsilon = 0.001; // Pequeña tolerancia para comparar valores cercanos
            let center_to_point = point - self.center;

            if (center_to_point.x.abs() - (self.max.x - self.center.x)).abs() < epsilon {
                normal.x = center_to_point.x.signum(); // Normal en el eje X
            } else if (center_to_point.x.abs() - (self.center.x - self.min.x)).abs() < epsilon {
                normal.x = -center_to_point.x.signum(); // Normal en el eje X (cara opuesta)
            } else if (center_to_point.y.abs() - (self.max.y - self.center.y)).abs() < epsilon {
                normal.y = center_to_point.y.signum(); // Normal en el eje Y
            } else if (center_to_point.y.abs() - (self.center.y - self.min.y)).abs() < epsilon {
                normal.y = -center_to_point.y.signum(); // Normal en el eje Y (cara opuesta)
            } else if (center_to_point.z.abs() - (self.max.z - self.center.z)).abs() < epsilon {
                normal.z = center_to_point.z.signum(); // Normal en el eje Z
            } else if (center_to_point.z.abs() - (self.center.z - self.min.z)).abs() < epsilon {
                normal.z = -center_to_point.z.signum(); // Normal en el eje Z (cara opuesta)
            }
       
            // Obtener coordenadas UV
            let (u, v) = self.get_uv(&point, &normal);

            return Intersect {
                distance,
                point,
                normal,
                material: self.material.clone(),
                is_intersecting: true, // Hay una intersección
                u, // Coordenada u obtenida de get_uv
                v, // Coordenada v obtenida de get_uv
            };
        }

        // Si no hay intersección, devolvemos un Intersect vacío
        Intersect {
            is_intersecting: false,
            distance: 0.0,
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: self.material.clone(),
            u: 0.0,
            v: 0.0,
        }
    }
}

