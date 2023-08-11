use derive_more::Constructor;

use crate::{primitive::Vec3, Ray};

#[derive(Debug, Clone, Copy, Constructor)]
pub struct Camera {
    origin: Vec3,
    upper_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction =
            self.upper_left_corner + self.horizontal * u + self.vertical * v - self.origin;

        Ray::new(self.origin, direction)
    }
}
