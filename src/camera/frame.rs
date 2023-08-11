use derive_more::Constructor;
use getset::CopyGetters;

use crate::primitive::Vec3;

#[derive(Debug, Clone, Copy, Constructor, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Frame {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}
