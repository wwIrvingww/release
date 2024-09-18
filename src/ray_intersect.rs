use nalgebra_glm::Vec3;
use crate::material::Material;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Intersect {
    pub point: Vec3,       // El punto de impacto
    pub normal: Vec3,      // La normal en el punto de impacto
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
    pub u: f32,            // Coordenada U para el mapeo de textura
    pub v: f32,            // Coordenada V para el mapeo de textura
}

impl Intersect {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: Material, u: f32, v: f32) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
            u,
            v,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: 0.0,
            is_intersecting: false,
            material: Material::black(),
            u: 0.0,
            v: 0.0,
        }
    }
}

// Define el trait CloneBox
pub trait CloneBox {
    fn clone_box(&self) -> Box<dyn RayIntersect>;
}

// Implementa CloneBox para cualquier tipo que implemente RayIntersect y Clone
impl<T> CloneBox for T
where
    T: 'static + RayIntersect + Clone,
{
    fn clone_box(&self) -> Box<dyn RayIntersect> {
        Box::new(self.clone())
    }
}

// Define el trait RayIntersect, que ahora hereda de CloneBox
pub trait RayIntersect: CloneBox {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}

// Implementa Clone para Box<dyn RayIntersect> usando clone_box
impl Clone for Box<dyn RayIntersect> {
    fn clone(&self) -> Box<dyn RayIntersect> {
        self.clone_box()
    }
}
