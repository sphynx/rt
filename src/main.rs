// This is directly based on Peter Shirley's "Ray Tracing in One
// Weekend".

use rt::{Ray, Vec3};

fn color(r: &Ray) -> Vec3 {
    let center = Vec3(0.0, 0.0, -1.0);
    let red = Vec3(1.0, 0.0, 0.0);
    if hit_sphere(&center, 0.5, r) {
        return red;
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    let white = Vec3(1.0, 1.0, 1.0);
    let blue = Vec3(0.5, 0.7, 1.0);

    (1.0 - t) * white + t * blue
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let dir = r.direction();
    let a = dir.dot(&dir);
    let b = 2.0 * oc.dot(&dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn main() {
    let nx = 400;
    let ny = 200;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    // Describe the plane.
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let dir = lower_left_corner + u * horizontal + v * vertical;
            let r = Ray::new(origin, dir);
            let c = 255.99 * color(&r);
            println!("{} {} {}", c.r() as u32, c.g() as u32, c.b() as u32);
        }
    }
}
