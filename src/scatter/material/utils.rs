use rand_distr::{Distribution, UnitSphere};

use crate::primitive::Vec3;

pub(super) fn random_in_unit_sphere() -> Vec3 {
    UnitSphere.sample(&mut rand::thread_rng()).into()
}
