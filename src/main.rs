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

    let lambertian1 = Lambertian::new(Vec3(0.1, 0.2, 0.5));
    let lambertian2 = Lambertian::new(Vec3(0.8, 0.8, 0.0));
    let metal1 = Metal::new(Vec3(0.8, 0.6, 0.2), 0.0);
    let diel1 = Dielectric::new(1.5);

    // Describe the world.
    let s1 = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: &lambertian1,
    };

    let s2 = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: &lambertian2,
    };

    let s3 = Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: &metal1,
    };

    let s4 = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: &diel1,
    };

    let s5 = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: &diel1,
    };

    let world = [s1, s2, s3, s4, s5];

    let aspect = f64::from(nx) / f64::from(ny);

    let camera = Camera::new(
        Vec3(-2.0, 2.0, 1.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        90.0,
        aspect,
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();

            // Antialiasing by averaging of random samples.
            for _ in 0..ns {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(nx);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(ny);
                let r = camera.get_ray(u, v);
                col += color(&r, &world[..], 0);
            }

            col /= f64::from(ns);
            col.sqrt_coords(); // Basic gamma correction.
            col *= 255.99;

            println!("{} {} {}", col.r() as u32, col.g() as u32, col.b() as u32);
        }
    }
}
