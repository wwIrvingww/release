use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::ray_intersect::RayIntersect; // Usamos RayIntersect en lugar de Sphere
use crate::cast_ray::cast_ray;
use crate::camera::Camera;
use nalgebra_glm::Vec3;

pub fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn RayIntersect>], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_x = screen_x * aspect_ratio;

            let ray_direction = camera.basis_change(&Vec3::new(screen_x, screen_y, -1.0));

            // Pasamos depth como argumento y usamos trait objects (RayIntersect)
            let pixel_color = cast_ray(&camera.eye, &ray_direction, objects, light, 0);

            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x as isize, y as isize);
        }
    }
}
