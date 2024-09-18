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

    let mut last_frame_time = Instant::now();
    let mut fps = 60.0;
    let fps_threshold = 30.0;

    let mut framebuffer = Framebuffer::new(width, height);

    // Definir los materiales
    let materials = define_materials();
    let moss_material = materials[0].clone();
    let dirt_material = materials[1].clone();
    let stone_material = materials[2].clone();
    let redstone_material = materials[3].clone();
    let violetstone_material = materials[4].clone();
    let blackstone_material = materials[5].clone();
    let wood_material = materials[6].clone();
    let door_material = materials[7].clone();
    let log_material = materials[8].clone();
    let leaves_material = materials[9].clone();
    let water_material = materials[10].clone();

    let cube_size = 0.5;

     //Objetos estatiscos
    let mut static_objects: Vec<Box<dyn RayIntersect>> = Vec::new();

    //**MOSS**
    // Borde superior e inferior del rectángulo de moss (sin espacio entre los cubos)
    for x in 0..5 {
        // Parte superior (fila 1)
        let cube_top = Cube::new(
            Vec3::new(x as f32 * cube_size, 0.0, 0.0), // Posicionar los cubos pegados en el nivel z = 0
            cube_size,                                  // Tamaño del cubo
            moss_material.clone(),  // Usar Arc<Material> para optimizar memoria
        );
        static_objects.push(Box::new(cube_top));

        // Parte inferior (fila 6)
        let cube_bottom = Cube::new(
            Vec3::new(x as f32 * cube_size, 0.0, -cube_size * 5.0), // Posicionar los cubos pegados en la fila inferior
            cube_size,                                               // Tamaño del cubo
            moss_material.clone(),  // Usar Arc<Material> para optimizar memoria
        );
        static_objects.push(Box::new(cube_bottom));
    }

    // Lados del rectángulo de moss (sin espacio entre los cubos)
    for z in 1..5 {
        // Lado izquierdo
        let cube_left = Cube::new(
            Vec3::new(0.0, 0.0, -(z as f32 * cube_size)), // Posicionar los cubos pegados en el lado izquierdo
            cube_size,                                    // Tamaño del cubo
            moss_material.clone(),  // Usar Arc<Material> para optimizar memoria
        );
        static_objects.push(Box::new(cube_left));

        // Lado derecho
        let cube_right = Cube::new(
            Vec3::new(4.0 * cube_size, 0.0, -(z as f32 * cube_size)), // Posicionar los cubos pegados en el lado derecho
            cube_size,                                                 // Tamaño del cubo
            moss_material.clone(),  // Usar Arc<Material> para optimizar memoria
        );
        static_objects.push(Box::new(cube_right));
    }

    //**DIRT CUBOS BASE DEL FONDO**//
    // Añadir los 30 cubos de dirt (dos paredes de 15 cubos cada una)
    for z in 4..=5 { // Z = 4 para la primera pared, Z = 5 para la segunda
        for x in 0..5 {
            // Bloques de dirt en z-index 2, 3, y 4 encima del borde del fondo
            for y in 1..=3 {
                let cube_dirt = Cube::new(
                    Vec3::new(x as f32 * cube_size, (y as f32) * cube_size, -(z as f32 * cube_size)), // Posición en el grid
                    cube_size,                                                                       // Tamaño del cubo
                    dirt_material.clone(),  // Usar Arc<Material> para optimizar memoria
                );
                static_objects.push(Box::new(cube_dirt));
            }
        }
    }

    // Añadir dos cubos de dirt en las posiciones especificadas
    let dirt_cube_1 = Cube::new(
        Vec3::new(-cube_size, 2.0 * cube_size, -4.0 * cube_size),  // Posición para el primer cubo de dirt
        cube_size,  // Tamaño del cubo
        dirt_material.clone(),  // Usar el material de dirt
            );
        
    let dirt_cube_2 = Cube::new(
        Vec3::new(-cube_size, 1.0 * cube_size, -4.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        dirt_material.clone(),  // Usar el material de dirt
    );
    
    let dirt_cube_3 = Cube::new(
        Vec3::new(-cube_size, 2.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        dirt_material.clone(),  // Usar el material de dirt
    );
            
    //A la izquierda de este, debe ir al agua
    let dirt_cube_4 = Cube::new(
   Vec3::new(cube_size * 4.0, 2.0 * cube_size, -2.0 * cube_size),  // Posición para el segundo cubo de dirt
           cube_size,  // Tamaño del cubo
        dirt_material.clone(),  // Usar el material de dirt
    );
    
    let dirt_cube_5 = Cube::new(
        Vec3::new(cube_size * 4.0, 2.0 * cube_size, -1.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        dirt_material.clone(),  // Usar el material de dirt
    );
    
    let dirt_cube_6 = Cube::new(
        Vec3::new(cube_size * 4.0, 1.0 * cube_size, -2.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        dirt_material.clone(),  // Usar el material de dirt
    );
    
    let dirt_cube_7 = Cube::new(
        Vec3::new(cube_size * 1.0, 2.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        dirt_material.clone(),  // Usar el material de dirt
    );
    
    let dirt_cube_8 = Cube::new(
        Vec3::new(cube_size * 2.0, 1.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        dirt_material.clone(),  // Usar el material de dirt
    );
    
    let dirt_cube_9 = Cube::new(
        Vec3::new(cube_size * 3.0, 1.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        dirt_material.clone(),  // Usar el material de dirt
    );
        
    // Añadir los cubos a la lista de objetos
    static_objects.push(Box::new(dirt_cube_1));
    static_objects.push(Box::new(dirt_cube_2));
    static_objects.push(Box::new(dirt_cube_3));
    static_objects.push(Box::new(dirt_cube_4));
    static_objects.push(Box::new(dirt_cube_5));
    static_objects.push(Box::new(dirt_cube_6));
    static_objects.push(Box::new(dirt_cube_7));
    static_objects.push(Box::new(dirt_cube_8));
    static_objects.push(Box::new(dirt_cube_9));

    // Añadir una fila de cubos de dirt desde la primera hasta la última posición
    for x in 0..=4 {
        let dirt_cube = Cube::new(
            Vec3::new(x as f32 * cube_size, 3.0 * cube_size, -3.0 * cube_size),  // Posición en la fila
            cube_size,  // Tamaño del cubo
            dirt_material.clone(),  // Usar el material de dirt
        );
        // Añadir el cubo a la lista de objetos
        static_objects.push(Box::new(dirt_cube));
    }

    //STONE FILA DE LA IZQUIERDA
    for y in 1..=3 { // 3 bloques de altura
        let cube_stone = Cube::new(
            Vec3::new(-cube_size, (y as f32) * cube_size, -5.0 * cube_size), // Columna izquierda, en el eje X = -cube_size
            cube_size,  // Tamaño del cubo
            stone_material.clone(),  // Usar Arc<Material> para optimizar memoria
        );
        static_objects.push(Box::new(cube_stone));
    }

    // Añadir un cubo individual de stone en la posición correcta
    // Posición en la misma coordenada Z de la columna eliminada, a la misma altura que los otros dirts
    let single_stone_cube = Cube::new(
        Vec3::new(-cube_size, 3.0 * cube_size, -4.0 * cube_size),  // Ajusta la posición aquí
        cube_size,  // Tamaño del cubo
        stone_material.clone(),  // Usar el material de stone
    );

     // Posición en la misma coordenada Z de la columna eliminada, a la misma altura que los otros dirts
     let single_stone_cube2 = Cube::new(
        Vec3::new(cube_size * 2.0, 2.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        stone_material.clone(),  // Usar el material de stone
    );

     // Posición en la misma coordenada Z de la columna eliminada, a la misma altura que los otros dirts
     let single_stone_cube3 = Cube::new(
        Vec3::new(cube_size * 3.0, 2.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        stone_material.clone(),  // Usar el material de stone
    );

    // Añadir el cubo a la lista de objetos
    static_objects.push(Box::new(single_stone_cube));
    static_objects.push(Box::new(single_stone_cube2));
    static_objects.push(Box::new(single_stone_cube3));

    //**REDSTONE**//
    // Añadir un cubo de redstone en una posición que tú elijas
    let redstone_material = materials[3].clone();  // Usar el material de redstone

    let redstone_material2 = materials[3].clone();  // Usar el material de redstone

    let redstone_cube = Cube::new(
        Vec3::new(-cube_size, 3.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        redstone_material.clone(),  // Usar el material de redstone
    );


    let redstone_cube2 = Cube::new(
        Vec3::new(-cube_size, 1.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        redstone_material.clone(),  // Usar el material de redstone
    );

    let redstone_cube3 = Cube::new(
        Vec3::new(cube_size * 0.0, 2.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        redstone_material.clone(),  // Usar el material de redstone
    );

    // Añadir el cubo a la lista de objetos
    static_objects.push(Box::new(redstone_cube));
    static_objects.push(Box::new(redstone_cube2));
    static_objects.push(Box::new(redstone_cube3));

    //VIOLET STONE//
    // Crear un cubo de violetstone en una posición específica
    let violetstone_cube = Cube::new(
        Vec3::new(cube_size, 1.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        violetstone_material.clone(),  // Usar el material de violetstone
    );

    // Crear un cubo de violetstone en una posición específica
    let violetstone_cube2 = Cube::new(
        Vec3::new(cube_size * 0.0, 1.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        violetstone_material.clone(),  // Usar el material de violetstone
    );

    // Añadir el cubo a la lista de objetos
    static_objects.push(Box::new(violetstone_cube));
    static_objects.push(Box::new(violetstone_cube2));

    //**BLACKSTONE**

    // Crear un cubo de blackstone en una posición específica
    let blackstone_cube = Cube::new(
        Vec3::new(cube_size * 4.0, 1.0 * cube_size, -1.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        blackstone_material.clone(),  // Usar el material de blackstone
    );

    let blackstone_cube2 = Cube::new(
        Vec3::new(cube_size * 3.0, 1.0 * cube_size, 0.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        blackstone_material.clone(),  // Usar el material de blackstone
    );

    let blackstone_cube3 = Cube::new(
        Vec3::new(cube_size * 2.0, 1.0 * cube_size, 0.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        blackstone_material.clone(),  // Usar el material de blackstone
    );

    let blackstone_cube4 = Cube::new(
        Vec3::new(cube_size * 1.0, 1.0 * cube_size, -1.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        blackstone_material.clone(),  // Usar el material de blackstone
    );

    let blackstone_cube5 = Cube::new(
        Vec3::new(cube_size * 1.0, 1.0 * cube_size, -2.0 * cube_size),  // Posición para el segundo cubo de dirt
        cube_size,  // Tamaño del cubo
        blackstone_material.clone(),  // Usar el material de blackstone
    );

    // Añadir el cubo a la lista de objetos
    static_objects.push(Box::new(blackstone_cube));
    static_objects.push(Box::new(blackstone_cube2));
    static_objects.push(Box::new(blackstone_cube3));
    static_objects.push(Box::new(blackstone_cube4));
    static_objects.push(Box::new(blackstone_cube5));


    //**WOOD**//
    
    // Añadir tres cubos de wood en una misma columna desde la posición 4 a la 6 en el eje Y
    for y in 4..=6 { // Desde la posición 4 hasta la 6 en el eje Y
        let cube_wood = Cube::new(
            Vec3::new(-cube_size, (y as f32) * cube_size, -4.0 * cube_size), // Columna izquierda en el eje X = -cube_size, con la altura en Y de 4 a 6
            cube_size,  // Tamaño del cubo
            wood_material.clone(),  // Usar el material de wood
        );
        static_objects.push(Box::new(cube_wood));
    }

    for y in 4..=6 { // Desde la posición 4 hasta la 6 en el eje Y
        let cube_wood = Cube::new(
            Vec3::new(-cube_size, (y as f32) * cube_size, -3.0 * cube_size), // Columna izquierda en el eje X = -cube_size, con la altura en Y de 4 a 6
            cube_size,  // Tamaño del cubo
            wood_material.clone(),  // Usar el material de wood
        );
        static_objects.push(Box::new(cube_wood));
    }

    let wood_cube = Cube::new(
            Vec3::new(cube_size * 0.0, 6.0 * cube_size, -3.0 * cube_size),  // Posición para el segundo cubo de dirt
            cube_size,  // Tamaño del cubo
            wood_material.clone(),  // Usar el material de blackstone
    );

    let wood_cube2 = Cube::new(
        Vec3::new(1.0 * cube_size, 4.0 * cube_size, -3.0 * cube_size),  // Posición en la casilla superior (5)
        cube_size,  // Tamaño del cubo
        wood_material.clone(),  // Usar el material de blackstone
    );

    let wood_cube3 = Cube::new(
        Vec3::new(1.0 * cube_size, 5.0 * cube_size, -3.0 * cube_size),  // Posición en la casilla superior (5)
        cube_size,  // Tamaño del cubo
        wood_material.clone(),  // Usar el material de blackstone
    );

    let wood_cube4 = Cube::new(
        Vec3::new(1.0 * cube_size, 5.5 * cube_size, -3.0 * cube_size),  // Posición en la casilla superior (5)
        cube_size ,  // Tamaño del cubo
        wood_material.clone(),  // Usar el material de blackstone
    );


    // Añadir el cubo a la lista de objetos
    static_objects.push(Box::new(wood_cube));
    static_objects.push(Box::new(wood_cube2));
    static_objects.push(Box::new(wood_cube3));
    static_objects.push(Box::new(wood_cube4));

    for y in 4..=5 { // Desde la posición 4 hasta la 6 en el eje Y
        let cube_wood = Cube::new(
            Vec3::new(cube_size * 2.0, (y as f32) * cube_size, -3.0 * cube_size), // Columna izquierda en el eje X = -cube_size, con la altura en Y de 4 a 6
            cube_size,  // Tamaño del cubo
            wood_material.clone(),  // Usar el material de wood
        );
        static_objects.push(Box::new(cube_wood));
    }


    //--------------------------------------------------------------------------------------------------------------------------//

    //Objetos con animacion
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
