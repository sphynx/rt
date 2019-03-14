// This is directly based on Peter Shirley's "Ray Tracing in One
// Weekend".

use rand::prelude::*;
use rt::*;
use std::f64;

#[allow(dead_code)]
fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut v;
    loop {
        v = 2.0 * Vec3(rng.gen(), rng.gen(), rng.gen()) - Vec3::ones();
        if v.length_squared() < 1.0 {
            break;
        }
    }
    v
}

fn color<T: Hitable + ?Sized>(rng: &mut ThreadRng, ray: &Ray, world: &T) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, f64::MAX) {
        // Diffuse.
        let target = hit.point + hit.normal + random_in_unit_sphere(rng);
        let new_ray = Ray::new(hit.point, target - hit.point);
        0.5 * color(rng, &new_ray, world)
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

    // Describe the world.
    let s1 = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let s2 = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    let world = [s1, s2];

    let camera = Camera::new();

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = camera.get_ray(u, v);
                col += color(&mut rng, &r, &world[..]);
            }

            col /= ns as f64;
            col.sqrt_coords(); // Basic gamma correction.
            col *= 255.99;

            println!("{} {} {}", col.r() as u32, col.g() as u32, col.b() as u32);
        }
    }
}
