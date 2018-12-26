mod ray;

use crate::ray::*;

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

fn main() {
    let env = Environment {
        gravity: vector(-4.0, -4.0, -4.0),
        wind: vector(3.0, 3.0, 3.0)
    };
    let mut projectile = Projectile {
        position: point(0.0, 0.0, 0.0),
        velocity: vector(1.0, 1.0, 1.0)
    };

    while projectile.position.x >= 0.0 {
        projectile = tick(env, projectile);
        println!("Next position: {:?}", projectile);
    }
}
