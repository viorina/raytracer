use std::ops;

use derive_more::Constructor;
use getset::CopyGetters;
use rand::Rng;
use rand_distr::{Distribution, UnitSphere};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Constructor, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn ones() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        UnitSphere.sample(&mut rand::thread_rng()).into()
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0)
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn sqrt(self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - normal * self.dot(normal) * 2.0
    }

    pub fn refract(self, normal: Vec3, eta: f32) -> Vec3 {
        let cos_theta = (-self).dot(normal).min(1.0);

        let vec_perpendicular = eta * (self + cos_theta * normal);
        let vec_parallel = -(1.0 - vec_perpendicular.length_squared()).abs().sqrt() * normal;

        vec_perpendicular + vec_parallel
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        self * Vec3::new(rhs, rhs, rhs)
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self, self, self) * rhs
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(v: [f32; 3]) -> Self {
        Vec3::new(v[0], v[1], v[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_product() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(i.cross(j), k);
        assert_eq!(j.cross(k), i);
        assert_eq!(k.cross(i), j);

        assert_eq!(j.cross(i), -k);
        assert_eq!(k.cross(j), -i);
        assert_eq!(i.cross(k), -j);

        assert_eq!((i * 2.0).cross(j * 2.0), k * 4.0);
        assert_eq!((j * 2.0).cross(k * 2.0), i * 4.0);
        assert_eq!((k * 2.0).cross(i * 2.0), j * 4.0);
    }

    #[test]
    fn test_dot() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(a.dot(i), 1.0);
        assert_eq!(a.dot(j), 2.0);
        assert_eq!(a.dot(k), 3.0);
        assert_eq!(a.dot(b), 6.0);
    }

    #[test]
    fn test_basic_ops() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 4.0, 6.0);
        let n = 2.0;

        assert_eq!(a * n, b);
        assert_eq!(a - b, -a);
        assert_eq!(a + b, Vec3::new(3.0, 6.0, 9.0));
        assert_eq!(a * b, Vec3::new(2.0, 8.0, 18.0));
    }

    #[test]
    fn test_assign_ops() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 4.0, 6.0);
        let n = 2.0;

        a *= n;
        assert_eq!(a, b);

        a /= n;
        assert_eq!(a, Vec3::new(1.0, 2.0, 3.0));

        a += b;
        assert_eq!(a, Vec3::new(3.0, 6.0, 9.0));

        a -= b;
        assert_eq!(a, Vec3::new(1.0, 2.0, 3.0));

        a *= b;
        assert_eq!(a, Vec3::new(2.0, 8.0, 18.0));
    }
}
