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

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use super::{ Color };
    use crate::{ color };

    #[test]
    fn colors_are_tuples() {
        let c = color(-0.5, 0.4, 1.7);

        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);

        assert_color_eq(c1 + c2, color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);

        assert_color_eq(c1 - c2, color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = color(0.2, 0.3, 0.4);

        assert_color_eq(c * 2.0, color(0.4, 0.6, 0.8))
    }

    #[test]
    fn multiplying_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);

        assert_color_eq(c1 * c2, color(0.9, 0.2, 0.04));
    }

    fn assert_color_eq(a: Color, b: Color) {
        const EPSILON: f32 = 0.00001;

        assert_approx_eq!(a.red(), b.red(), EPSILON);
        assert_approx_eq!(a.green(), b.green(), EPSILON);
        assert_approx_eq!(a.blue(), b.blue(), EPSILON);
    }
}
