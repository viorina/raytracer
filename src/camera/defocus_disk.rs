use getset::CopyGetters;

use crate::primitive::Vec3;

use super::Frame;

#[derive(Debug, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct DefocusDisk {
    defocus_angle: f32,
    frame: Frame,
}

impl DefocusDisk {
    pub fn new(camera_frame: Frame, focus_distance: f32, defocus_angle: f32) -> DefocusDisk {
        let radius = focus_distance * (defocus_angle / 2.0).to_radians().tan();
        let frame = Frame::new(
            camera_frame.u() * radius,
            camera_frame.v() * radius,
            Vec3::default(),
        );

        DefocusDisk {
            defocus_angle,
            frame,
        }
    }

    pub fn sample(&self) -> Vec3 {
        if self.defocus_angle > 0.0 {
            let p = Vec3::random_in_unit_disk();
            p.x() * self.frame.u() + p.y() * self.frame.v()
        } else {
            Vec3::default()
        }
    }
}
