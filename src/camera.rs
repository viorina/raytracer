mod frame;
mod defocus_disk;
mod viewport;

use frame::Frame;
use defocus_disk::DefocusDisk;
use viewport::Viewport;

use crate::{primitive::Vec3, Ray};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    viewport: Viewport,
    defocus_disk: DefocusDisk,
}

impl Camera {
    pub fn new(
        point_of_view: Vec3,
        view_up: Vec3,
        center_of_view: Vec3,
        field_of_view_angle: f32,
        focus_distance: f32,
        aspect_ratio: f32,
        defocus_angle: f32,
    ) -> Camera {
        let camera_frame = {
            let w = (point_of_view - center_of_view).unit();
            let u = view_up.cross(w).unit();
            let v = w.cross(u);

            Frame::new(u, v, w)
        };

        let viewport = Viewport::new(
            point_of_view,
            camera_frame,
            field_of_view_angle,
            focus_distance,
            aspect_ratio,
        );
        let defocus_disk = DefocusDisk::new(camera_frame, focus_distance, defocus_angle);

        Camera {
            origin: point_of_view,
            viewport,
            defocus_disk,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let origin = self.origin + self.defocus_disk.sample();
        let direction = self.viewport.upper_left_corner()
            + self.viewport.frame().u() * u
            + self.viewport.frame().v() * v
            - origin;

        Ray::new(origin, direction)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let point_of_view = Vec3::new(-2.0, 2.0, 1.0);
        let view_up = Vec3::new(0.0, 1.0, 0.0);
        let center_of_view = Vec3::new(0.0, 0.0, -1.0);

        let field_of_view_angle = 20.0;
        let focus_distance = 3.4;
        let aspect_ratio = 16.0 / 9.0;

        let defocus_angle = 10.0;

        Camera::new(
            point_of_view,
            view_up,
            center_of_view,
            field_of_view_angle,
            focus_distance,
            aspect_ratio,
            defocus_angle,
        )
    }
}
