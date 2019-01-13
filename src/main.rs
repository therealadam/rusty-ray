use rusty_ray::{Tuple, vector, point, canvas, color};
use std::fs::File;
use std::result::Result::Ok;
use std::error::Error;
use std::io::Write;

#[derive(Debug, Copy, Clone)]
struct Environment {
    gravity: Tuple,
    wind: Tuple
}

#[derive(Debug, Copy, Clone)]
struct Projectile {
    position: Tuple,
    velocity: Tuple
}

fn tick(env: Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + env.gravity + env.wind;

    Projectile { position, velocity }
}

fn write(name: &str, data: &str) -> () {
    let mut f = match File::create(name) {
        Err(why) => panic!("couldn't create {}: {}", name, why.description()),
        Ok(file) => file,
    };
    let contents = data.as_bytes();

    match f.write_all(contents) {
        Err(why) => panic!("couldn't write to {}: {}", name, why.description()),
        Ok(_) => {},
    }
}

fn main() {
    let mut canvas = canvas(900, 550);

    let mut projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.8, 0.0).normalize() * 11.25
    };
    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0)
    };
    let color = color(1.0, 0.2, 0.2);

    projectile = tick(env, projectile);
    while projectile.position.x >= 0.0 && projectile.position.y >= 0.0 {
        let pos = (
            projectile.position.x as usize,
            canvas.height - projectile.position.y as usize
        );
        canvas.write_pixel(pos.0, pos.1, color);
        projectile = tick(env, projectile);
//        println!("Next position: {:?}", projectile);
    }

    let ppm = canvas.to_ppm();
    write("ray.ppm", &ppm);

//    println!("Ok!");
}
