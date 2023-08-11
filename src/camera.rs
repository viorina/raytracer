mod viewport;

use viewport::ViewPort;

use crate::{
    primitive::{Basis, Vec3},
    Ray,
};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    upper_left_corner: Vec3,
    viewport_frame: Basis,
}

impl Camera {
    pub fn new(
        point_of_view: Vec3,
        view_up: Vec3,
        center_of_view: Vec3,
        field_of_view_angle: f32,
        focus_distance: f32,
        aspect_ratio: f32,
    ) -> Camera {
        let camera_frame = {
            let w = (point_of_view - center_of_view).unit();
            let u = view_up.cross(w).unit();
            let v = w.cross(u);

            Basis::new(u, v, w)
        };

        let viewport = ViewPort::new(field_of_view_angle, focus_distance, aspect_ratio);
        let viewport_frame = viewport.frame(camera_frame.u(), camera_frame.v());

        let upper_left_corner = point_of_view
            - focus_distance * camera_frame.w()
            - viewport_frame.u() / 2.0
            - viewport_frame.v() / 2.0;

        Camera {
            origin: point_of_view,
            upper_left_corner,
            viewport_frame,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction =
            self.upper_left_corner + self.viewport_frame.u() * u + self.viewport_frame.v() * v
                - self.origin;

        Ray::new(self.origin, direction)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let point_of_view = Vec3::new(-2.0, 2.0, 1.0);
        let view_up = Vec3::new(0.0, 1.0, 0.0);
        let center_of_view = Vec3::new(0.0, 0.0, -1.0);

        let field_of_view_angle = 90.0;
        let focus_distance = 3.4;
        let aspect_ratio = 16.0 / 9.0;

        Camera::new(
            point_of_view,
            view_up,
            center_of_view,
            field_of_view_angle,
            focus_distance,
            aspect_ratio,
        )
    }
}
