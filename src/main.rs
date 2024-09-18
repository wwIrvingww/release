mod color;
mod framebuffer;
mod render;
mod camera;
mod material;
mod ray_intersect;
mod cube;
mod grid;
mod light;
mod cast_ray;
mod frustum;

use framebuffer::Framebuffer;
use render::render;
use camera::Camera;
use material::{Material, Texture};
use color::Color;
use nalgebra_glm::{Vec3, vec3};
use cube::Cube;
use grid::{Grid3D, GridObject};
use light::Light;
use frustum::Frustum;
use once_cell::sync::OnceCell;
use minifb::{Key, Window, WindowOptions};
use std::sync::Arc;
use std::time::Instant;
use crate::ray_intersect::RayIntersect;

fn load_texture(path: &str) -> Arc<Texture> {
    Texture::load_from_file(path).into()
}

fn define_materials() -> Vec<Arc<Material>> {
    vec![
        Arc::new(Material {
            diffuse: Color::new(40, 150, 40),
            specular: 1.0,
            albedo: [0.1, 0.2, 0.2, 0.0],
            refractive_index: 0.3,
            transparency: 0.1,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/moss (1).png")),
            has_texture: true,
        }),
        Arc::new(Material {
            diffuse: Color::new(150, 100, 50),
            specular: 15.0,
            albedo: [0.2, 0.3, 0.1, 0.0],
            refractive_index: 1.0,
            transparency: 0.4,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/dirt.png")),
            has_texture: true,
        }),
        Arc::new(Material {
            diffuse: Color::new(40, 150, 200),
            specular: 50.0,
            albedo: [0.1, 0.9, 0.1, 0.0],
            refractive_index: 1.33,
            transparency: 5.8,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/water.png")),
            has_texture: true,
        }),
        Arc::new(Material {
            diffuse: Color::new(105, 105, 105),
            specular: 13.0,
            albedo: [0.2, 0.3, 0.1, 0.2],
            refractive_index: 1.0,
            transparency: 0.1,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/stone.png")),
            has_texture: true,
        }),
        Arc::new(Material {
            diffuse: Color::new(105, 105, 105),
            specular: 23.0,
            albedo: [0.2, 0.4, 0.1, 0.2],
            refractive_index: 1.0,
            transparency: 0.1,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/violetstone.png")),
            has_texture: true,
        }),
        Arc::new(Material {
            diffuse: Color::new(30, 30, 30),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.1, 0.0],
            refractive_index: 0.1,
            transparency: 0.0,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/blackstone.png")),
            has_texture: true,
        }),
        Arc::new(Material {
            diffuse: Color::new(139, 69, 19),
            specular: 26.0,
            albedo: [0.2, 0.3, 0.1, 0.2],
            refractive_index: 1.0,
            transparency: 0.5,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/wood.png")),
            has_texture: true,
        }),
        Arc::new(Material {
            diffuse: Color::new(160, 82, 45),
            specular: 11.0,
            albedo: [0.2, 0.3, 0.1, 0.2],
            refractive_index: 1.1,
            transparency: 0.0,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/door.png")),
            has_texture: true,
        }),
        Arc::new(Material {
            diffuse: Color::new(72, 60, 50),
            specular: 50.0,
            albedo: [0.2, 0.2, 0.1, 0.0],
            refractive_index: 0.9,
            transparency: 0.8,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/log.png")),
            has_texture: true,
        }),
        Arc::new(Material {
            diffuse: Color::new(255, 192, 203),
            specular: 28.0,
            albedo: [0.2, 0.3, 0.1, 0.1],
            refractive_index: 1.1,
            transparency: 0.2,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/leaves.png")),
            has_texture: true,
        }),
        Arc::new(Material {
            diffuse: Color::new(105, 105, 105),
            specular: 13.0,
            albedo: [0.2, 0.3, 0.1, 0.2],
            refractive_index: 1.0,
            transparency: 0.1,
            texture: Some(load_texture("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/release/textures/redstone.png")),
            has_texture: true,
        }),
    ]
}

fn main() {
    let width = 800;
    let height = 600;

    let mut last_frame_time = Instant::now();
    let mut fps = 60.0;
    let fps_threshold = 30.0;

    let mut framebuffer = Framebuffer::new(width, height);

    let materials = define_materials();
    let moss_material = materials[0].clone();
    let dirt_material = materials[1].clone();
    let water_material = materials[2].clone();
    let stone_material = materials[3].clone();
    let violetstone_material = materials[4].clone();
    let blackstone_material = materials[5].clone();
    let wood_material = materials[6].clone();
    let door_material = materials[7].clone();
    let log_material = materials[8].clone();
    let leaves_material = materials[9].clone();
    let redstone_material = materials[10].clone();

    let cube_size = 0.5;

    let mut static_objects: Vec<Box<dyn RayIntersect>> = Vec::new();

    // Insertar los bloques de la escena anterior
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

    let mut water_cubes: Vec<Cube> = vec![
        Cube::new(Vec3::new(3.0 * cube_size, 2.0 * cube_size, -2.0 * cube_size), cube_size, water_material.clone()),
        Cube::new(Vec3::new(2.0 * cube_size, 2.0 * cube_size, -2.0 * cube_size), cube_size, water_material.clone()),
        Cube::new(Vec3::new(3.0 * cube_size, 1.5 * cube_size, -1.0 * cube_size), cube_size, water_material.clone()),
        Cube::new(Vec3::new(2.0 * cube_size, 1.5 * cube_size, -1.0 * cube_size), cube_size, water_material.clone()),
    ];

    // Añadir bloques adicionales de la escena anterior con sus respectivos materiales
    let cube_stone = Cube::new(
        Vec3::new(-cube_size, 3.0 * cube_size, -4.0 * cube_size),
        cube_size,
        stone_material.clone(),
    );
    static_objects.push(Box::new(cube_stone));

    let violetstone_cube = Cube::new(
        Vec3::new(2.0 * cube_size, 2.0 * cube_size, -3.0 * cube_size),
        cube_size,
        violetstone_material.clone(),
    );
    static_objects.push(Box::new(violetstone_cube));

    let blackstone_cube = Cube::new(
        Vec3::new(4.0 * cube_size, 1.0 * cube_size, -1.0 * cube_size),
        cube_size,
        blackstone_material.clone(),
    );
    static_objects.push(Box::new(blackstone_cube));

    let wood_cube = Cube::new(
        Vec3::new(-cube_size, 4.0 * cube_size, -3.0 * cube_size),
        cube_size,
        wood_material.clone(),
    );
    static_objects.push(Box::new(wood_cube));

    let log_cube = Cube::new(
        Vec3::new(3.0 * cube_size, 4.0 * cube_size, -2.0 * cube_size),
        cube_size,
        log_material.clone(),
    );
    static_objects.push(Box::new(log_cube));

    let leaves_cube = Cube::new(
        Vec3::new(2.0 * cube_size, 7.0 * cube_size, -2.0 * cube_size),
        cube_size,
        leaves_material.clone(),
    );
    static_objects.push(Box::new(leaves_cube));

    let redstone_cube = Cube::new(
        Vec3::new(-cube_size, 3.0 * cube_size, -3.0 * cube_size),
        cube_size,
        redstone_material.clone(),
    );
    static_objects.push(Box::new(redstone_cube));

    let light = Light::new(
        Vec3::new(0.0, 12.0, 20.0),
        Color::new(116, 140, 153),
        3.0,
    );

    let mut camera = Camera::new(
        Vec3::new(0.0, 2.5, 6.5),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        width as f32 / height as f32,
        50.1,
        95.0,
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
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_frame_time);
        last_frame_time = current_time;

        fps = 1.0 / delta_time.as_secs_f32();
        let es_mucha_carga = fps < fps_threshold;
        let scale_factor = if es_mucha_carga { 0.5 } else { 1.0 };
        let scaled_width = (width as f32 * scale_factor) as usize;
        let scaled_height = (height as f32 * scale_factor) as usize;

        framebuffer = Framebuffer::new(scaled_width, scaled_height);

        let frustum = Frustum::new(&camera);

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

        let mut objects: Vec<Box<dyn RayIntersect>> = Vec::new();

        // Filtrar objetos dentro del frustum
        for obj in static_objects.iter() {
            let obj_center = obj.position();
            if frustum.is_sphere_in_frustum(obj_center, cube_size / 2.0) {
                objects.push(obj.as_ref().clone_box());
            }
        }

        for (i, cube) in water_cubes.iter_mut().enumerate() {
            let animated_y = cube.position().y + 0.05 * (0.1 * ((t + i as f32).sin()));
            cube.set_position(Vec3::new(cube.position().x, animated_y, cube.position().z));

            if frustum.is_sphere_in_frustum(cube.position(), cube_size / 2.0) {
                objects.push(Box::new(cube.clone()));
            }
        }

        render(&mut framebuffer, objects.as_slice(), &camera, &light);

        window.update_with_buffer(&framebuffer.buffer, scaled_width, scaled_height).unwrap();

        t += 0.03;
    }
}
