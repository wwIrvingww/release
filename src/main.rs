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

    // Crear un framebuffer de 800x600
    let mut framebuffer = Framebuffer::new(width, height);

    // Definir los materiales de las esferas
    let red_material = Material::new(Color::new(255, 0, 0), 50.0, [0.6, 0.3], 0.5); // 50% reflexión
    let blue_material = Material::new(Color::new(0, 0, 255), 50.0, [0.6, 0.3], 0.2); // 20% reflexión
    let green_material = Material::new(Color::new(0, 255, 0), 50.0, [0.6, 0.3], 0.8); // 80% reflexión

    // Crear las esferas
    let sphere1 = Sphere::new(Vec3::new(-2.0, 0.0, -5.0), 1.0, red_material);
    let sphere2 = Sphere::new(Vec3::new(2.0, 0.0, -6.0), 1.0, blue_material);
    let sphere3 = Sphere::new(Vec3::new(0.0, -1.5, -4.0), 1.0, green_material);

    // Definir la luz en la escena
    let light = Light::new(Vec3::new(2.0, 4.0, 3.0), Color::new(255, 255, 255), 1.0);

    // Definir los objetos en la escena
    let objects = vec![sphere1, sphere2, sphere3];

    // Crear la cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    // Crear la ventana
    let mut window = Window::new(
        "Raytracer with Reflections and Shadows",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Variables para controlar la velocidad de la cámara
    let camera_speed = 0.1;

    // Loop principal
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Manejar el movimiento de la cámara con las teclas WASD
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

        // Renderizar la escena
        render(&mut framebuffer, objects.as_slice(), &camera, &light);

        // Actualizar la ventana con el framebuffer renderizado
        window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height).unwrap();
    }
}
