mod lambertian;
mod metal;
mod utils;

use derive_more::Constructor;
use getset::CopyGetters;

use crate::{hit::HitRecord, Ray, Vec3};

pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<ScatteredRay>;
}

#[derive(Debug, Constructor, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct ScatteredRay {
    ray: Ray,
    attenuation: Vec3,
}
