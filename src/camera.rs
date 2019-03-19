use crate::geometry::*;
use crate::vec::*;
use rand::prelude::*;
use std::f32::consts::PI;

/// Defines the screen on which scene is projected and the origin
/// (i.e. the point of view).
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: Elem,
}

/// Camera settings used to define a Camera.
pub struct CameraSettings {
    /// a point to look from (origin)
    pub look_from: Vec3,
    /// a point to look at (will end up in the center of the matrix).
    /// `look_from` and `look_at` together specify the axis along
    /// which we can still tilt our camera
    pub look_at: Vec3,
    /// up direction for the camera (specifies a tilt of camera)
    pub v_up: Vec3,
    /// vertical field of view (in degrees)
    pub v_fov: Elem,
    /// width / height ratio of the picture (i.e. matrix of the camera)
    pub aspect: Elem,
    /// width of camera aperture
    pub aperture: Elem,
    /// distance to focus plance
    pub focus_dist: Elem,
}

impl Camera {
    pub fn new(s: CameraSettings) -> Camera {
        let theta = s.v_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = s.aspect * half_height;
        let origin = s.look_from;
        let w = Vec3::unit_vector(s.look_from - s.look_at);
        let u = Vec3::unit_vector(Vec3::cross(s.v_up, w));
        let v = Vec3::cross(w, u);

        Camera {
            origin,
            lower_left_corner: origin - s.focus_dist * (half_width * u + half_height * v + w),
            horizontal: 2.0 * s.focus_dist * half_width * u,
            vertical: 2.0 * s.focus_dist * half_height * v,
            lens_radius: s.aperture / 2.0,
            u,
            v,
        }
    }

    /// Calculates the ray to a particular point on the camera matrix,
    /// specified by (s, t) coordinates.
    pub fn get_ray(&self, s: Elem, t: Elem) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut v;
    let mut rng = rand::thread_rng();
    loop {
        v = 2.0 * Vec3(rng.gen(), rng.gen(), 0.0) - Vec3(1.0, 1.0, 0.0);
        if v.length_squared() < 1.0 {
            break;
        }
    }
    v
}
