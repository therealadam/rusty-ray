#![allow(dead_code)]

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

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

}

impl core::ops::Mul for Color {
    type Output = Color;

    // Hadamard/Schur product
    fn mul(self, other: Color) -> Color {
        color(
            self.red() * other.red(),
            self.green() * other.green(),
            self.blue() * other.blue()
        )
    }
}

impl Tuple {

    pub fn tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::tuple(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::tuple(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w != 1.0
    }

    pub fn magnitude(&self) -> f32 {
        (
            self.x.powi(2) +
                self.y.powi(2) +
                self.z.powi(2) +
                self.w.powi(2)
        ).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        tuple(
            self.x / self.magnitude(),
            self.y / self.magnitude(),
            self.z / self.magnitude(),
            self.w / self.magnitude(),
        )
    }

    pub fn dot(&self, other: Tuple) -> f32 {
        self.x * other.x +
            self.y * other.y +
            self.z * other.z +
            self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }

}

impl core::ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w.max(other.z)
        }
    }
}

impl core::ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: (self.w - other.w).max(0.0)
        }
    }
}

impl core::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        let zero = tuple(0.0, 0.0, 0.0, 0.0);

        zero - self
    }
}

impl core::ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f32) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl core::ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, other: f32) -> Tuple {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

// TODO move to prelude
pub fn tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
    Tuple::tuple(x, y, z, w)
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::point(x, y, z)
}

pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::vector(x, y, z)
}

pub fn color(r: f32, g: f32, b: f32) -> Color {
    // such hax
    point(r, g, b)
}

#[cfg(test)]
mod tests {
    //    use super::*;
    use crate::ray::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn point_tuple() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }

    #[test]
    fn vector_tuple() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);
    }

    #[test]
    fn factories() {
        let p = point(4f32, -4f32, 3f32);
        let v = vector(4f32, -4f32, 3f32);

        assert_eq!(p, tuple(4f32, -4f32, 3f32, 1f32));
        assert_eq!(v, tuple(4f32, -4f32, 3f32, 0f32));
    }

    #[test]
    fn add_tuples() {
        let a1 = tuple(3.0, -2.0, 5.0, 1.0);
        let a2 = tuple(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1 + a2, tuple(1.0 ,1.0, 6.0, 1.0))
    }

    #[test]
    fn subtract_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, point(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);

        assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_a_tuple() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-a, tuple(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);

        assert_tuple_eq(a * 3.5, tuple(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiply_tuple_by_faction() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);

        assert_tuple_eq(a * 0.5, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);

        assert_tuple_eq(a / 2.0, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn vector_magnitudes() {
        let v1 = vector(1.0, 0.0, 0.0);
        let v2 = vector(0.0, 1.0, 0.0);
        let v3 = vector(0.0, 0.0, 1.0);
        let v4 = vector(1.0, 2.0, 3.0);
        let v5 = vector(-1.0, -2.0, -3.0);

        assert_eq!(v1.magnitude(), 1.0);
        assert_eq!(v2.magnitude(), 1.0);
        assert_eq!(v3.magnitude(), 1.0);
        assert_eq!(v4.magnitude(), 14f32.sqrt());
        assert_eq!(v5.magnitude(), 14f32.sqrt());
    }

    #[test]
    fn vector_normalize() {
        let v1 = vector(4.0, 0.0, 0.0);
        let v2 = vector(1f32, 2f32, 3f32);
        let norm = v2.normalize();

        assert_eq!(v1.normalize(), vector(1f32, 0f32, 0f32));
        assert_tuple_eq(norm, vector(0.26726, 0.53452, 0.80178));
        assert_approx_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);

        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);

        assert_eq!(a.cross(&b), vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), vector(1.0, -2.0, 1.0));
    }

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

    fn assert_tuple_eq(a: Tuple, b: Tuple) {
        const EPSILON: f32 = 0.00001;

        assert_approx_eq!(a.x, b.x, EPSILON);
        assert_approx_eq!(a.y, b.y, EPSILON);
        assert_approx_eq!(a.z, b.z, EPSILON);
        assert_approx_eq!(a.w, b.w, EPSILON);
    }

    fn assert_color_eq(a: Color, b: Color) {
        const EPSILON: f32 = 0.00001;

        assert_approx_eq!(a.red(), b.red(), EPSILON);
        assert_approx_eq!(a.green(), b.green(), EPSILON);
        assert_approx_eq!(a.blue(), b.blue(), EPSILON);
    }
}
