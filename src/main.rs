mod color;
mod framebuffer;
mod render;
mod camera;
mod material;
mod ray_intersect;
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
use cube::Cube;
use grid::{Grid3D, GridObject}; // Importa el grid y sus objetos
use light::Light;
use once_cell::sync::OnceCell;
use minifb::{Key, Window, WindowOptions};
use std::sync::Arc;
use crate::ray_intersect::RayIntersect;

// Crear el método para cargar texturas
// Crear el método para cargar texturas
fn load_texture(path: &str) -> Arc<Texture> {
    Texture::load_from_file(path).into()
}


// Define las texturas y materiales usando Arc con las rutas originales
fn define_materials() -> Vec<Arc<Material>> {
    vec![
        // Moss
        Arc::new(Material {
            diffuse: Color::new(40, 150, 40),  // Color verde musgo
            specular: 1.0,  // Sin brillo, no refleja luz
            albedo: [0.1,0.2, 0.2, 0.0],  // Absorción de luz en lugar de reflectividad
            refractive_index: 0.3,  // Sin distorsión visual
            transparency: 0.1,  // Moderada transparencia
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/moss (1).png")),
            has_texture: true,
        }),
        // Dirt
        Arc::new(Material {
            diffuse: Color::new(150, 100, 50),
            specular: 15.0,  // Bajo brillo
            albedo: [0.2, 0.3, 0.1, 0.0],
            refractive_index: 1.0,
            transparency: 0.4,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/dirt.png")),
            has_texture: true,
        }),
        // Stone
        Arc::new(Material {
            diffuse: Color::new(105, 105, 105),
            specular: 13.0,  // Mayor brillo que la tierra
            albedo: [0.2,0.3, 0.1, 0.2],  // Absorción de luz en lugar de reflectividad
            refractive_index: 1.0,
            transparency: 0.1,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/stone.png")),
            has_texture: true,
        }),
        // Redstone
        Arc::new(Material {
            diffuse: Color::new(105, 105, 105),
            specular: 13.0,  // Mayor brillo que la tierra
            albedo: [0.2,0.3, 0.1, 0.2],  // Absorción de luz en lugar de reflectividad
            refractive_index: 1.0,
            transparency: 0.1,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/redstone.png")),
            has_texture: true,
        }),
        // Violetstone
        Arc::new(Material {
            diffuse: Color::new(105, 105, 105),
            specular: 23.0,  // Mayor brillo que la tierra
            albedo: [0.2,0.4, 0.1, 0.2],  // Absorción de luz en lugar de reflectividad
            refractive_index: 1.0,
            transparency: 0.1,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/violetstone.png")),
            has_texture: true,
        }),
        // Blackstone
        Arc::new(Material {
            diffuse: Color::new(30, 30, 30),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.1, 0.0],
            refractive_index: 0.1,
            transparency: 0.0,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/blackstone.png")),
            has_texture: true,
        }),
        // Wood
        Arc::new(Material {
            diffuse: Color::new(139, 69, 19),
            specular: 26.0,
            albedo: [0.2,0.3, 0.1, 0.2],  // Absorción de luz en lugar de reflectividad
            refractive_index: 1.0,
            transparency: 0.5,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/wood.png")),
            has_texture: true,
        }),
        // Door
        Arc::new(Material {
            diffuse: Color::new(160, 82, 45),
            specular: 11.0,
            albedo: [0.2,0.3, 0.1, 0.2],  // Absorción de luz en lugar de reflectividad
            refractive_index: 1.1,
            transparency: 0.0,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/door.png")),
            has_texture: true,
        }),
        // Log
        Arc::new(Material {
            diffuse: Color::new(72, 60, 50),
            specular: 50.0,
            albedo: [0.2, 0.2, 0.1, 0.0],
            refractive_index: 0.9,
            transparency: 0.8,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/log.png")),
            has_texture: true,
        }),
        // Leaves
        Arc::new(Material {
            diffuse: Color::new(255, 192, 203),
            specular: 28.0,
            albedo: [0.2,0.3, 0.1, 0.1],  // Absorción de luz en lugar de reflectividad
            refractive_index: 1.1,
            transparency: 0.2,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/leaves.png")),
            has_texture: true,
        }),
        // Water
        Arc::new(Material {
            diffuse: Color::new(40, 150, 200),
            specular: 50.0,
            albedo: [0.1, 0.9, 0.1, 0.0],
            refractive_index: 1.33,
            transparency: 5.8,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/water.png")),
            has_texture: true,
        }),
    ]
}


fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer = Framebuffer::new(width, height);

    // Definir los materiales
    let materials = define_materials();
    let moss_material = materials[0].clone();
    let dirt_material = materials[1].clone();
    let water_material = materials[10].clone();

    let cube_size = 0.5;

    // Lista para cubos estáticos (que no cambian)
    let mut static_objects: Vec<Box<dyn RayIntersect>> = Vec::new();

    // Añadir cubos estáticos
    for x in 0..5 {
        let cube_top = Cube::new(
            Vec3::new(x as f32 * cube_size, 0.0, 0.0),
            cube_size,
            moss_material.clone(),
        );
        static_objects.push(Box::new(cube_top));

        let cube_bottom = Cube::new(
            Vec3::new(x as f32 * cube_size, 0.0, -cube_size * 5.0),
            cube_size,
            moss_material.clone(),
        );
        static_objects.push(Box::new(cube_bottom));
    }

    for z in 1..5 {
        let cube_left = Cube::new(
            Vec3::new(0.0, 0.0, -(z as f32 * cube_size)),
            cube_size,
            moss_material.clone(),
        );
        static_objects.push(Box::new(cube_left));

        let cube_right = Cube::new(
            Vec3::new(4.0 * cube_size, 0.0, -(z as f32 * cube_size)),
            cube_size,
            moss_material.clone(),
        );
        static_objects.push(Box::new(cube_right));
    }

    // Añadir cubos de dirt
    for z in 4..=5 {
        for x in 0..5 {
            for y in 1..=3 {
                let cube_dirt = Cube::new(
                    Vec3::new(x as f32 * cube_size, (y as f32) * cube_size, -(z as f32 * cube_size)),
                    cube_size,
                    dirt_material.clone(),
                );
                static_objects.push(Box::new(cube_dirt));
            }
        }
    }

    // Lista para cubos animados (cubos de agua)
    let mut water_cubes: Vec<Cube> = vec![
        Cube::new(Vec3::new(3.0 * cube_size, 2.0 * cube_size, -2.0 * cube_size), cube_size, water_material.clone()),
        Cube::new(Vec3::new(2.0 * cube_size, 2.0 * cube_size, -2.0 * cube_size), cube_size, water_material.clone()),
        Cube::new(Vec3::new(3.0 * cube_size, 1.5 * cube_size, -1.0 * cube_size), cube_size, water_material.clone()),
        Cube::new(Vec3::new(2.0 * cube_size, 1.5 * cube_size, -1.0 * cube_size), cube_size, water_material.clone()),
    ];

    let light = Light::new(
        Vec3::new(0.0, 12.0, 20.0),
        Color::new(116, 140, 153),
        3.0,
    );

    let mut camera = Camera::new(
        Vec3::new(0.0, 2.5, 6.5),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut window = Window::new(
        "Irving's Diorama",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let camera_speed = 0.1;
    let camera_rotate_speed = 0.05;

    let mut t = 0.0;

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

        // Crear una nueva lista de objetos que incluya los objetos estáticos y los cubos animados
        let mut objects: Vec<Box<dyn RayIntersect>> = Vec::new();
        objects.extend(static_objects.iter().map(|obj| obj.as_ref().clone_box()));

        // Animar los cubos de agua sin recrearlos
        for (i, cube) in water_cubes.iter_mut().enumerate() {
            let animated_y = cube.position().y + 0.05 * (0.1 * ((t + i as f32).sin()));
            cube.set_position(Vec3::new(cube.position().x, animated_y, cube.position().z));
        
            // Añadir los cubos de agua actualizados a la lista de objetos
            objects.push(Box::new(cube.clone()));
        }
        
        // Renderizar la escena
        render(&mut framebuffer, objects.as_slice(), &camera, &light);

        // Actualizar el buffer de la ventana
        window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height).unwrap();

        // Incrementar el tiempo para la animación
        t += 0.03;
    }
}
