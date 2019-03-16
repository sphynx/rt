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
    pub material: &'a Box<dyn Material>,
}

/// Abstracts away details of materials affecting how the rays
/// scatter.
pub trait Material {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> MaterialResponse;
}

pub enum MaterialResponse {
    Scattered { attenuation: Vec3, ray: Ray },
    Absorbed,
}

pub struct Lambertian {
    pub albedo: Vec3,
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

pub struct Sphere {
    pub center: Vec3,
    pub radius: Elem,
    pub material: Box<dyn Material>,
}

impl Hitable for Sphere {
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
                    normal: Vec3::unit_vector(p - self.center),
                    material: &self.material,
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
