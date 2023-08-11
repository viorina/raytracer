use crate::primitive::{Basis, Rectangle, Vec3};

#[derive(Debug)]
pub struct ViewPort(Rectangle);

impl ViewPort {
    pub fn new(field_of_view_angle: f32, focus_distance: f32, aspect_ratio: f32) -> ViewPort {
        let height = 2.0 * (field_of_view_angle.to_radians() / 2.0).tan() * focus_distance;
        let width = height * aspect_ratio;

        ViewPort(Rectangle::new(width, height))
    }

    pub fn height(&self) -> f32 {
        self.0.height()
    }

    pub fn width(&self) -> f32 {
        self.0.width()
    }

    pub fn frame(&self, u: Vec3, v: Vec3) -> Basis {
        Basis::new(self.width() * u, self.height() * (-v), Vec3::default())
    }
}
