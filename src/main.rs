// This is directly based on Peter Shirley's "Ray Tracing in One
// Weekend".

use rand::prelude::*;
use rt::MaterialResponse::*;
use rt::*;
use std::f64;

fn color<T: Hitable + ?Sized>(ray: &Ray, world: &T, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, f64::MAX) {
        if depth < 50 {
            match hit.material.scatter(ray, &hit) {
                Absorbed => Vec3::zero(),
                Scattered { attenuation, ray } => attenuation * color(&ray, world, depth + 1),
            }
        } else {
            Vec3::zero()
        }
    } else {
        // Draw gradient background.
        let unit_direction = Vec3::unit_vector(ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        let white = Vec3(1.0, 1.0, 1.0);
        let blue = Vec3(0.5, 0.7, 1.0);

        // Interpolate between white and "blue".
        (1.0 - t) * white + t * blue
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let nx = 600;
    let ny = 300;
    let ns = 50;

    println!("P3 {} {} 255", nx, ny);

    let lambertian1 = Lambertian {
        albedo: Vec3(0.8, 0.3, 0.3),
    };
    let lambertian2 = Lambertian {
        albedo: Vec3(0.8, 0.8, 0.0),
    };

    // Describe the world.
    let s1 = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(lambertian1),
    };

    let s2 = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(lambertian2),
    };

    let world = [s1, s2];

    let camera = Camera::new();

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();

            // Antialiasing by averaging of random samples.
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = camera.get_ray(u, v);
                col += color(&r, &world[..], 0);
            }

            col /= ns as f64;
            col.sqrt_coords(); // Basic gamma correction.
            col *= 255.99;

            println!("{} {} {}", col.r() as u32, col.g() as u32, col.b() as u32);
        }
    }
}
