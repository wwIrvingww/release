mod color;
mod framebuffer;
mod render;
mod camera;
mod material;
mod ray_intersect;
mod sphere;
mod cube;
mod grid; // Asegúrate de agregar el módulo del grid
mod light;
mod cast_ray;

use framebuffer::Framebuffer;
use render::render;
use camera::Camera;
use material::{Material, Texture};
use color::Color;
use nalgebra_glm::{Vec3, vec3};
use sphere::Sphere;
use cube::Cube;
use grid::{Grid3D, GridObject}; // Importa el grid y sus objetos
use light::Light;
use once_cell::sync::OnceCell;
use minifb::{Key, Window, WindowOptions};
use std::sync::Arc;
use crate::ray_intersect::RayIntersect;

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

    // Crear un grid de 10x10x10
    let mut grid = Grid3D::new(10);

    // Posicionar objetos en el grid
    grid.place_object(1, 1, 1, GridObject::Cube);   // Posicionar un cubo en (1, 1, 1)
    grid.place_object(5, 5, 5, GridObject::Sphere); // Posicionar una esfera en (5, 5, 5)

    let mut objects: Vec<Box<dyn RayIntersect>> = Vec::new();

    // Crear objetos en el grid según lo que hemos colocado
    for x in 0..10 {
        for y in 0..10 {
            for z in 0..10 {
                match grid.get_object(x, y, z) {
                    GridObject::Cube => {
                        let cube = Cube::new(
                            Vec3::new(x as f32, y as f32, -(z as f32 + 5.0)), // Posicionar el cubo en la escena
                            1.0,
                            cube_material.clone(),
                        );
                        objects.push(Box::new(cube));
                    }
                    GridObject::Sphere => {
                        let sphere = Sphere::new(
                            Vec3::new(x as f32, y as f32, -(z as f32 + 5.0)), // Posicionar la esfera en la escena
                            0.5,
                            sphere_material.clone(),
                        );
                        objects.push(Box::new(sphere));
                    }
                    _ => {}
                }
            }
        }
    }

    // Luz ambiental e iluminación de la escena
    let light = Light::new(
        Vec3::new(0.0, 5.0, 5.0),   // Luz desde arriba y un poco detrás
        Color::new(255, 255, 255),   // Color de la luz
        5.0,                         // Intensidad de la luz
    );

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 15.0),   // Colocar la cámara más lejos para ver más del grid
        Vec3::new(0.0, 0.0, 0.0),    // La cámara apunta al centro de la escena
        Vec3::new(0.0, 1.0, 0.0),    // Vector "arriba" de la cámara
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
    let camera_rotate_speed = 0.05;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Controlar el movimiento de la cámara
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

        // Controlar la rotación de la cámara
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -camera_rotate_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, camera_rotate_speed);
        }
        if window.is_key_down(Key::Left) {
            camera.orbit(-camera_rotate_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(camera_rotate_speed, 0.0);
        }

        // Renderizar la escena
        render(&mut framebuffer, objects.as_slice(), &camera, &light);

        window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height).unwrap();
    }
}
