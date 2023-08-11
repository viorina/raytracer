use derive_more::Constructor;
use getset::CopyGetters;

use crate::Vec3;

#[derive(Debug, Default, Clone, Copy, Constructor, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Color {
        assert!(Vec3::default() <= v && v <= Vec3::ones());

        Color::new(
            (v.x() * 255.0) as u8,
            (v.y() * 255.0) as u8,
            (v.z() * 255.0) as u8,
        )
    }
}
