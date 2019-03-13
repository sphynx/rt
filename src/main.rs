// This is directly based on Peter Shirley's "Ray Tracing in One
// Weekend".

use rt::*;
use std::f64;

fn color<T: Hitable>(ray: &Ray, world: T) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0, f64::MAX) {
        // Visualise normals for hit objects.
        let normal = Vec3::unit_vector(hit.normal);
        0.5 * (normal + Vec3::ones())
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
    let nx = 600;
    let ny = 300;
    println!("P3 {} {} 255", nx, ny);

    // Describe the plane.
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3::zero();

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

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let dir = lower_left_corner + u * horizontal + v * vertical;
            let r = Ray::new(origin, dir);
            let c = 255.99 * color(&r, &world);
            println!("{} {} {}", c.r() as u32, c.g() as u32, c.b() as u32);
        }
    }
}
