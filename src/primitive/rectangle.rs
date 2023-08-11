use derive_more::Constructor;
use getset::CopyGetters;

#[derive(Debug, Default, Clone, Copy, Constructor, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Rectangle {
    width: f32,
    height: f32,
}
