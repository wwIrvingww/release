#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Constructor que recibe valores RGB en u8
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    // Constructor que recibe un valor HEX
    pub fn from_hex(hex: &str) -> Result<Color, &'static str> {
        if hex.len() != 6 {
            return Err("Hex color must be 6 characters long.");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex value")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex value")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex value")?;

        Ok(Color { r, g, b })
    }

    // Sumar dos colores sin sobrepasar el valor de 255
    pub fn add(&self, other: &Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }

    // Multiplicar un color por un número
    pub fn multiply(&self, scalar: f32) -> Color {
        Color {
            r: (self.r as f32 * scalar).min(255.0).max(0.0) as u8,
            g: (self.g as f32 * scalar).min(255.0).max(0.0) as u8,
            b: (self.b as f32 * scalar).min(255.0).max(0.0) as u8,
        }
    }
}

// Implementación del operador Mul para Color
impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        self.multiply(scalar)
    }
}

// Implementación del operador Add para Color
impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
}

