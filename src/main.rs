fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[derive(PartialEq, Debug)]
    struct Tuple {
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    }

    impl Tuple {

        fn tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
            Tuple {
                x: x,
                y: y,
                z: z,
                w: w,
            }
        }

        fn point(x: f32, y: f32, z: f32) -> Tuple {
            Tuple::tuple(x, y, z, 1.0)
        }

        fn vector(x: f32, y: f32, z: f32) -> Tuple {
            Tuple::tuple(x, y, z, 0.0)
        }

        fn is_point(&self) -> bool {
            self.w == 1.0
        }

        fn is_vector(&self) -> bool {
            self.w != 1.0
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
        let a = Tuple::tuple(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);
    }

    #[test]
    fn factories() {
        let p = Tuple::point(4f32, -4f32, 3f32);
        let v = Tuple::vector(4f32, -4f32, 3f32);

        assert_eq!(p, Tuple::tuple(4f32, -4f32, 3f32, 1f32));
        assert_eq!(v, Tuple::tuple(4f32, -4f32, 3f32, 0f32));
    }

    #[test]
    fn add_tuples() {
        let a1 = Tuple::tuple(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::tuple(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1 + a2, Tuple::tuple(1.0 ,1.0, 6.0, 1.0))
    }

    #[test]
    fn subtract_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0))
    }
    
    #[test]
    fn subtract_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }
}