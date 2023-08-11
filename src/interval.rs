use derive_more::Constructor;
use getset::CopyGetters;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Constructor, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::new( f32::MAX, f32::MIN)
    }
}
