mod color;
mod framebuffer;
mod render;
mod camera;
mod material;
mod ray_intersect;
mod sphere;
mod light;
mod cast_ray; // Asegúrate de agregar esto al principio de main.rs

use framebuffer::Framebuffer;
use render::render;
use camera::Camera;
use material::{Material, Texture}; // Asegúrate de importar tanto Material como Texture
use color::Color;
use nalgebra_glm::{Vec3, vec3};
use sphere::Sphere;
use light::Light;
use once_cell::sync::OnceCell; // Importa OnceCell
use minifb::{Key, Window, WindowOptions}; // Importaciones para manejar la ventana y las teclas
use std::sync::Arc;

// Usa OnceCell para crear una referencia estática a la textura
static TEXTURE: OnceCell<Arc<Texture>> = OnceCell::new();

fn get_texture() -> Arc<Texture> {
    TEXTURE.get_or_init(|| {
        // Cargar la textura desde un archivo
        let texture_path = "C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/texture.png";
        Texture::load_from_file(texture_path) // Ya retorna un Arc<Texture>
    }).clone() // Retorna una referencia clonada de Arc
}

fn main() {
    let width = 800;
    let height = 600;

    // Crear un framebuffer de 800x600
    let mut framebuffer = Framebuffer::new(width, height);

    // Load the texture only once
    let texture = get_texture();

    //Definir el material de la esfera con textura
    let textured_material = Material {
        diffuse: Color::new(255, 0, 0),  // Cambia a gris
        specular: 50.0,
        albedo: [0.6, 0.3, 0.1, 0.0],
        refractive_index: 1.5,
        transparency: 0.0,
        texture: Some(texture.clone()),  // Clonar el Arc
        has_texture: true,               // Indicar que el material tiene una textura
    };
    

    // let textured_material = Material {
    //     diffuse: Color::new(255, 0, 0),  // Cambia a rojo puro para probar
    //     specular: 50.0,
    //     albedo: [0.6, 0.3, 0.1, 0.0],
    //     refractive_index: 1.5,
    //     transparency: 0.0,
    //     texture: None,  // Sin textura para probar
    //     has_texture: false,
    // };
    

    // Crear una esfera con el material texturizado
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, textured_material);

    // Definir la luz en la escena
    let light = Light::new(Vec3::new(2.0, 4.0, 3.0), Color::new(255, 255, 255), 2.0);  // Intensidad de la luz aumentada

    // Definir los objetos en la escena
    let objects = vec![sphere];

    // Crear la cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    // Crear la ventana
    let mut window = Window::new(
        "Raytracer with Textured Sphere",
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
