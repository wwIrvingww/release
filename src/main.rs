mod color;
mod framebuffer;
mod render;
mod camera;
mod material;
mod ray_intersect;
mod sphere;
mod light;
mod cast_ray;

use framebuffer::Framebuffer;
use render::render;
use camera::Camera;
use material::Material;
use color::Color;
use nalgebra_glm::{Vec3, vec3};
use sphere::Sphere;
use light::Light;
use minifb::{Key, Window, WindowOptions}; // Importaciones para manejar la ventana y las teclas

fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer = Framebuffer::new(width, height);

    // Material de la primera esfera (reflexiva)
    let reflective_material = Material::new(Color::new(255, 0, 0), 50.0, [0.6, 0.3, 0.8, 0.0], 1.0, 0.0);

    // Material de la segunda esfera (transparente)
    let refractive_material = Material::new(Color::new(0, 255, 0), 50.0, [0.4, 0.3, 0.0, 0.9], 1.5, 0.9);

    let sphere1 = Sphere::new(Vec3::new(-1.5, 0.0, -5.0), 1.0, reflective_material);
    let sphere2 = Sphere::new(Vec3::new(1.5, 0.0, -5.0), 1.0, refractive_material);

    let light = Light::new(Vec3::new(5.0, 5.0, 5.0), Color::new(255, 255, 255), 1.0);

    let objects = vec![sphere1, sphere2];

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut window = Window::new(
        "Raytracer with Reflections and Refractions",
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
