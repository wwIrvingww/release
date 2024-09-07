#[allow(dead_code)]
use nalgebra_glm::Vec3;

pub struct Camera {
    pub eye: Vec3,    // Posición de la cámara en el espacio mundial
    pub center: Vec3, // Punto que la cámara está mirando
    pub up: Vec3,     // Vector "arriba" de la cámara
}

impl Camera {
    // Constructor para crear una nueva cámara
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera { eye, center, up }
    }

    // Método para obtener la dirección de la vista (center - eye)
    pub fn view_direction(&self) -> Vec3 {
        (self.center - self.eye).normalize()
    }

    // Cambio de base
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize(); // Eje z negativo
        let right = forward.cross(&self.up).normalize();     // Eje x
        let up = right.cross(&forward).normalize();          // Eje y

        let rotated = vector.x * right + vector.y * up - vector.z * forward;

        rotated.normalize()
    }

    // Método para mover la cámara alrededor del centro en órbita
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        // Calcular el vector desde el centro al ojo (vector de radio) y medir la distancia
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        // Calcular yaw actual (rotación alrededor del eje Y)
        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        // Calcular pitch actual (rotación alrededor del eje X)
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        // Aplicar las rotaciones delta
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * std::f32::consts::PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-std::f32::consts::PI / 2.0 + 0.1, std::f32::consts::PI / 2.0 - 0.1);

        // Calcular la nueva posición del ojo
        let new_eye = self.center + Vec3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos()
        );

        self.eye = new_eye;
    }

    // Método para actualizar la posición de la cámara
    pub fn move_camera(&mut self, delta: Vec3) {
        self.eye += delta;
        self.center += delta;
    }
}
