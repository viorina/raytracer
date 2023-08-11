use getset::CopyGetters;

use crate::primitive::Vec3;

use super::Frame;

#[derive(Debug, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Viewport {
    upper_left_corner: Vec3,
    frame: Frame,
}

impl Viewport {
    pub fn new(
        point_of_view: Vec3,
        camera_frame: Frame,
        field_of_view_angle: f32,
        focus_distance: f32,
        aspect_ratio: f32,
    ) -> Viewport {
        let height = 2.0 * (field_of_view_angle.to_radians() / 2.0).tan() * focus_distance;
        let width = height * aspect_ratio;

        let frame = Frame::new(
            width * camera_frame.u(),
            height * (-camera_frame.v()),
            Vec3::default(),
        );

        let upper_left_corner =
            point_of_view - camera_frame.w() * focus_distance - frame.u() / 2.0 - frame.v() / 2.0;

        Viewport {
            upper_left_corner,
            frame,
        }
    }
}
