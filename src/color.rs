use crate::tuple::{Tuple};

pub type Color = Tuple;

impl Color {

    pub fn red(&self) -> f32 {
        self.x
    }

    pub fn green(&self) -> f32 {
        self.y
    }

    pub fn blue(&self) -> f32 {
        self.z
    }

    pub fn ppm_str(&self) -> String {
        let scaled_r = clamped(self.red(), 255.0);
        let scaled_g = clamped(self.green(), 255.0);
        let scaled_b = clamped(self.blue(), 255.0);

        format!("{} {} {}", scaled_r, scaled_g, scaled_b)
    }
}

// XXX put this some place sensible
fn clamped(part: f32, scale: f32) -> u8 {
    let n = if part > 1.0 {
        1.0
    } else if part < 0.0 {
        0.0
    } else {
        part
    };

    (n * scale).round() as u8
}

impl core::ops::Mul for Color {
    type Output = Color;

    // Hadamard/Schur product
    fn mul(self, other: Color) -> Color {
        Color {
            x: self.red() * other.red(),
            y: self.green() * other.green(),
            z: self.blue() * other.blue(),
            w: 0.0
        }
    }
}

