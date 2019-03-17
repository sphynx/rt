use crate::vec::*;
use crate::material::*;

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
