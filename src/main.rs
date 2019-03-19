//! This is directly based on Peter Shirley's "Ray Tracing in One
//! Weekend".

use rand::prelude::*;
use rayon::prelude::*;
use rt::MaterialResponse::*;
use rt::*;
use std::f32;
use std::sync::Arc;

fn color<T: Hitable + ?Sized>(ray: &Ray, world: &T, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, f32::MAX) {
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

fn random_scene() -> Vec<Sphere> {
    let mut scene = Vec::with_capacity(500);
    scene.push(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian::new(Vec3(0.5, 0.5, 0.5))),
    });

    let mut rng = rand::thread_rng();
    for a in -11..11_i16 {
        for b in -11..11_i16 {
            let choose_mat: f32 = rng.gen();
            let mut rnd = || rng.gen::<f32>();
            let center = Vec3(f32::from(a) + 0.9 * rnd(), 0.2, f32::from(b) + 0.9 * rnd());
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // Diffuse.
                    let albedo = Vec3(rnd() * rnd(), rnd() * rnd(), rnd() * rnd());
                    material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // Metal.
                    let albedo = Vec3(
                        0.5 * (1.0 + rnd()),
                        0.5 * (1.0 + rnd()),
                        0.5 * (1.0 + rnd()),
                    );
                    let fuzz = 0.5 * rnd();
                    material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    // Glass.
                    let refr_index = 1.5;
                    material = Arc::new(Dielectric::new(refr_index));
                }

                scene.push(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }

    scene.push(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric::new(1.5)),
    });

    scene.push(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::new(Vec3(0.4, 0.2, 0.1))),
    });

    scene.push(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)),
    });

    scene
}

fn main() {

    let nx = 600_i16;
    let ny = 400_i16;
    let ns = 10_i16;

    println!("P3 {} {} 255", nx, ny);

    let world = random_scene();

    let aspect = f32::from(nx) / f32::from(ny);
    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(CameraSettings {
        look_from,
        look_at,
        v_up: Vec3(0.0, 1.0, 0.0),
        v_fov: 20.0,
        aspect,
        aperture: 0.1,
        focus_dist: 10.0,
    });

    // for j in (0..ny).rev() {
    (0..ny).into_par_iter().for_each(|j| {
        let mut rng = rand::thread_rng();
        for i in 0..nx {
            let mut col = Vec3::zero();

            // Antialiasing by averaging of random samples.
            for _ in 0..ns {
                let u = (f32::from(i) + rng.gen::<f32>()) / f32::from(nx);
                let v = (f32::from(j) + rng.gen::<f32>()) / f32::from(ny);
                let r = camera.get_ray(u, v);
                col += color(&r, &world[..], 0);
            }

            col /= f32::from(ns);
            col.sqrt_coords(); // Basic gamma correction.
            col *= 255.99;

            println!("{} {} {}", col.r() as u32, col.g() as u32, col.b() as u32);
        }
    });
    // }
}
