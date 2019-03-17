use crate::geometry::*;
use crate::vec::*;
use std::f64::consts::PI;

/// Defines the screen on which scene is projected and the origin
/// (i.e. the point of view).
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// # Arguments
    ///
    /// - `look_from` a point to look from (origin)
    /// - `look_at` a point to look at (will end up in the center of the matrix)
    ///
    /// `look_from` and `look_at` together specify the axis along which we can still tilt our camera
    ///
    /// - `v_up` up direction for the camera (specifies a tilt of camera)
    /// - `v_fov`: vertical field of view (in degrees)
    /// - `aspect`: width / height ratio of the picture (i.e. matrix of the camera)
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: Elem, aspect: Elem) -> Camera {
        let theta = v_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;

        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(v_up, w));
        let v = Vec3::cross(w, u);

        Camera {
            origin,
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
