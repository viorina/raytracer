use crate::vec3::Vec3;

#[derive(Debug, Clone, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
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
