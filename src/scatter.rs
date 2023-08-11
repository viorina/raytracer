pub(crate) mod material;

use derive_more::Constructor;
use getset::CopyGetters;

use crate::{hit::HitRecord, primitive::Vec3, Ray};

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<ScatteredRay>;
}

#[derive(Debug, Constructor, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct ScatteredRay {
    ray: Ray,
    attenuation: Vec3,
}
