mod vec;
use rand::prelude::*;
pub use vec::*;

/// Abstracts away an object which can be hit by a ray.
pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: Elem, tmax: Elem) -> Option<HitRecord>;
}

/// Packs together all the details of a ray hitting an object at
/// particular moment of space and time.
pub struct HitRecord<'a> {
    /// Time, i.e. 't' parameter value when we hit an object.
    pub time: Elem,

    /// The point of contact at which we hit an object.
    pub point: Vec3,

    /// Normal unit vector at the hit point.
    pub normal: Vec3,

    /// Reference to material at hit point.
    pub material: &'a dyn Material,
}

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
    pub fn new(refraction_index: Elem) -> Self {
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

/// Sphere object (has center, radius and material).
pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: Elem,
    pub material: &'a dyn Material,
}

impl<'a> Hitable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: Elem, t_max: Elem) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let dir = ray.direction();

        let a = dir.dot(&dir);
        let b = oc.dot(&dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mk_hit_record = |t| {
                let p = ray.point_at_parameter(t);
                Some(HitRecord {
                    time: t,
                    point: p,
                    normal: (p - self.center) / self.radius,
                    material: self.material,
                })
            };

            let t_small = (-b - discriminant.sqrt()) / a;
            let t_big = (-b + discriminant.sqrt()) / a;

            if t_small > t_min && t_small < t_max {
                mk_hit_record(t_small)
            } else if t_big > t_min && t_big < t_max {
                mk_hit_record(t_big)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<T: Hitable> Hitable for [T] {
    fn hit(&self, ray: &Ray, t_min: Elem, t_max: Elem) -> Option<HitRecord> {
        self.iter()
            .filter_map(|h| h.hit(ray, t_min, t_max))
            .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap())
    }
}

/// Defines a ray of light by using origin (a point) and a direction
/// (a vector). Both are represented as `Vec3` though.
pub struct Ray {
    from: Vec3,
    to: Vec3,
}

impl Ray {
    pub fn new(from: Vec3, to: Vec3) -> Ray {
        Ray { from, to }
    }

    pub fn origin(&self) -> Vec3 {
        self.from
    }

    pub fn direction(&self) -> Vec3 {
        self.to
    }

    pub fn point_at_parameter(&self, b: Elem) -> Vec3 {
        self.from + b * self.to
    }
}

/// Defines the screen on which scene is projected and the origin
/// (i.e. the point of view).
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Vec3::zero(),
            lower_left_corner: Vec3(-2.0, -1.0, -1.0),
            horizontal: Vec3(4.0, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}

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
