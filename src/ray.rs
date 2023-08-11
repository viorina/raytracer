use derive_more::Constructor;
use getset::CopyGetters;

use crate::primitive::Vec3;

#[derive(Debug, Clone, Copy, Constructor, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
