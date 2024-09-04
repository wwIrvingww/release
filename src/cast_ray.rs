use nalgebra_glm::{Vec3, dot};
use crate::color::Color;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::sphere::Sphere;
use crate::light::Light;

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

// Función para calcular sombras
fn cast_shadow(intersect: &Intersect, light: &Light, objects: &[Sphere]) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let shadow_ray_origin = intersect.point + intersect.normal * 1e-3; // Desplazamos el origen del rayo

    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance > 1e-3 {
            shadow_intensity = 1.0;
            break;
        }
    }

    shadow_intensity
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k < 0.0 {
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere], light: &Light, depth: u32) -> Color {
    if depth > 3 {
        return Color::new(135, 206, 235); // Color del cielo si se supera la profundidad
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return Color::new(135, 206, 235); // Color de fondo
    }

     // Depurar la textura con las coordenadas UV
     let test_texture_color = intersect.material.get_diffuse_color(intersect.u, intersect.v);
     println!("Color de la textura en el punto UV: {:?}", test_texture_color);

    // Calculamos reflexión y refracción
    let mut reflect_color = Color::black();
    let mut refract_color = Color::black();
    let reflectivity = intersect.material.albedo[2];
    let transparency = intersect.material.albedo[3];

    if reflectivity > 0.0 {
        let reflect_dir = reflect(&-ray_direction, &intersect.normal).normalize();
        let reflect_origin = intersect.point + intersect.normal * 1e-3;
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, depth + 1);
    }

    if transparency > 0.0 {
        let refract_dir = refract(ray_direction, &intersect.normal, intersect.material.refractive_index).normalize();
        let refract_origin = intersect.point - intersect.normal * 1e-3;
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, depth + 1);
    }

    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&(-light_dir), &intersect.normal);

    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let diffuse = intersect.material.diffuse * intersect.material.albedo[0] * diffuse_intensity * light.intensity;

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light.intensity;

    (diffuse + specular) * (1.0 - reflectivity - transparency) + (reflect_color * reflectivity) + (refract_color * transparency)
}


