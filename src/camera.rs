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
    /// - `vfov`: vertical field of view (in degrees)
    /// - `aspect`: width / height ratio
    pub fn new(vfov: Elem, aspect: Elem) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        Camera {
            origin: Vec3::zero(),
            lower_left_corner: Vec3(-half_width, -half_height, -1.0),
            horizontal: Vec3(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0 * half_height, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
