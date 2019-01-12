#![allow(dead_code)]

mod tuple;
mod color;
mod canvas;

pub use crate::tuple::Tuple;
use crate::color::Color;
use crate::canvas::Canvas;

pub fn tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
    Tuple::new(x, y, z, w)
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::point(x, y, z)
}

pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::vector(x, y, z)
}

pub fn color(r: f32, g: f32, b: f32) -> Color {
    // HAX!
    point(r, g, b)
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    let pixels = vec!(color(0.0, 0.0, 0.0); width * height);

    Canvas::new(width, height, pixels)
}

#[cfg(test)]
mod tests {

}
