use crate::geometry::*;
use crate::vec::*;
use rand::prelude::*;

/// Abstracts away details of materials affecting how the rays
/// scatter.
pub trait Material {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> MaterialResponse;
}

/// How the material responses to a ray.
pub enum MaterialResponse {
    Scattered { attenuation: Vec3, ray: Ray },
    Absorbed,
}

/// Lambertian defines diffused materials which reflect light
/// randomly.
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    /// Takes `albedo` parameter which in fact defines object's own
    /// color. Hitting rays will be attenuated based on this
    /// parameter.
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hr: &HitRecord) -> MaterialResponse {
        let target = hr.point + hr.normal + random_in_unit_sphere();
        let sc_ray = Ray::new(hr.point, target - hr.point);
        MaterialResponse::Scattered {
            attenuation: self.albedo,
            ray: sc_ray,
        }
    }
}

/// Metal surfaces, reflecting light deterministically.
pub struct Metal {
    albedo: Vec3,
    fuzz: Elem,
}

impl Metal {
    /// Albedo and fuzzing parameters. Fuzzing determines amount of
    /// randomization of the reflection (0 - no randomization, 1 -
    /// maximum allowed randomization).
    pub fn new(albedo: Vec3, fuzz: Elem) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz > 1.0 { 1.0 } else { fuzz },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> MaterialResponse {
        let reflected_dir = reflect(Vec3::unit_vector(ray.direction()), hr.normal);
        let scattered = Ray::new(
            hr.point,
            reflected_dir + self.fuzz * random_in_unit_sphere(),
        );
        if scattered.direction().dot(&hr.normal) > 0.0 {
            MaterialResponse::Scattered {
                attenuation: self.albedo,
                ray: scattered,
            }
        } else {
            MaterialResponse::Absorbed
        }
    }
}

/// Materials which partially reflect and partially refract light
/// (glass, diamonds, etc.)
pub struct Dielectric {
    refraction_index: Elem,
}

impl Dielectric {
    /// Takes a refraction index as a sole input.
    pub fn new(refraction_index: Elem) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> MaterialResponse {
        let d1 = Vec3::dot2(hr.normal, ray.direction());
        let back = d1 > 0.0;
        let (outward_normal, ni_over_nt, cosine) = if back {
            let cos = self.refraction_index * d1 / ray.direction().length();
            (-hr.normal, self.refraction_index, cos)
        } else {
            let cos = -d1 / ray.direction().length();
            (hr.normal, 1.0 / self.refraction_index, cos)
        };

        let reflected_ray = Ray::new(hr.point, reflect(ray.direction(), hr.normal));
        let sc_ray = match refract(ray.direction(), outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.refraction_index);
                if rand::random::<Elem>() < reflect_prob {
                    reflected_ray
                } else {
                    Ray::new(hr.point, refracted)
                }
            }
            None => reflected_ray,
        };

        MaterialResponse::Scattered {
            attenuation: Vec3(1.0, 1.0, 1.0),
            ray: sc_ray,
        }
    }
}

//
// Helper functions.
//

fn random_in_unit_sphere() -> Vec3 {
    let mut v;
    let mut rng = rand::thread_rng();
    loop {
        v = 2.0 * Vec3(rng.gen(), rng.gen(), rng.gen()) - Vec3::ones();
        if v.length_squared() < 1.0 {
            break;
        }
    }
    v
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot2(v, normal) * normal
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: Elem) -> Option<Vec3> {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot2(uv, n);
    let discr = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discr > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discr.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: Elem, ref_idx: Elem) -> Elem {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}
