mod color;
mod framebuffer;
mod render;
mod camera;
mod material;
mod ray_intersect;
mod sphere;
mod cube; // Asegúrate de agregar el módulo del cubo
mod light;
mod cast_ray;

use framebuffer::Framebuffer;
use render::render;
use camera::Camera;
use material::{Material, Texture}; 
use color::Color;
use nalgebra_glm::{Vec3, vec3};
use sphere::Sphere;
use cube::Cube; // Importa el cubo
use light::Light;
use once_cell::sync::OnceCell; 
use minifb::{Key, Window, WindowOptions}; 
use std::sync::Arc;
use crate::ray_intersect::RayIntersect; // <-- Agregar esta línea para importar el trait RayIntersect

static TEXTURE: OnceCell<Arc<Texture>> = OnceCell::new();

fn get_texture() -> Arc<Texture> {
    TEXTURE.get_or_init(|| {
        let texture_path = "C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/texture.png";
        Texture::load_from_file(texture_path)
    }).clone()
}

fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer = Framebuffer::new(width, height);

    let texture = get_texture();

    let sphere_material = Material {
        diffuse: Color::new(255, 182, 193), // Color rosa pastel
        specular: 50.0,
        albedo: [0.6, 0.3, 0.1, 0.0],
        refractive_index: 1.5,
        transparency: 0.0,
        texture: None,  
        has_texture: false,
    };

    let cube_material = Material {
        diffuse: Color::new(255, 255, 255), 
        specular: 50.0,
        albedo: [0.6, 0.3, 0.1, 0.0],
        refractive_index: 1.5,
        transparency: 0.0,
        texture: Some(texture.clone()),
        has_texture: true,
    };

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, sphere_material);

    let cube = Cube::new(
        Vec3::new(2.0, 0.0, -6.0),  // Centro del cubo
        1.0,                        // Tamaño del cubo
        cube_material,
    );
    

    let light = Light::new(Vec3::new(2.0, 4.0, 3.0), Color::new(255, 255, 255), 2.0);

    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(sphere),
        Box::new(cube),
    ];

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut window = Window::new(
        "Raytracer with Textured Cube and Pastel Pink Sphere",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let camera_speed = 0.1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::W) {
            camera.move_camera(vec3(0.0, 0.0, -camera_speed));
        }
        if window.is_key_down(Key::S) {
            camera.move_camera(vec3(0.0, 0.0, camera_speed));
        }
        if window.is_key_down(Key::A) {
            camera.move_camera(vec3(-camera_speed, 0.0, 0.0));
        }
        if window.is_key_down(Key::D) {
            camera.move_camera(vec3(camera_speed, 0.0, 0.0));
        }

        render(&mut framebuffer, objects.as_slice(), &camera, &light);

        window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height).unwrap();
    }
}
