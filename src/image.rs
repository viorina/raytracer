use std::ops;

use getset::{CopyGetters, Getters};
use rayon::prelude::*;

use crate::Color;

#[derive(Debug, CopyGetters, Getters)]
pub struct Image {
    #[getset(get_copy = "pub")]
    width: usize,
    #[getset(get_copy = "pub")]
    height: usize,
    #[getset(get = "pub")]
    data: Vec<Color>,
}

impl Image {
    pub fn with_background(width: usize, height: usize, color: Color) -> Image {
        let data = vec![color; width * height];

        Image {
            width,
            height,
            data,
        }
    }

    pub fn new(width: usize, height: usize) -> Image {
        Image::with_background(width, height, Color::default())
    }

    pub fn fill(&mut self, color: impl Fn(usize, usize) -> Color + Sync) {
        self.data
            .par_chunks_mut(self.width)
            .enumerate()
            .for_each(|(row, slice)| {
                for (col, pixel) in slice.iter_mut().enumerate() {
                    *pixel = color(row, col);
                }
            });
    }
}

impl ops::Index<usize> for Image {
    type Output = [Color];

    fn index(&self, row: usize) -> &[Color] {
        assert!(row < self.height);

        let start = row * self.width;
        &self.data[start..start + self.width]
    }
}

impl ops::IndexMut<usize> for Image {
    fn index_mut(&mut self, row: usize) -> &mut [Color] {
        assert!(row < self.height);

        let start = row * self.width;
        &mut self.data[start..start + self.width]
    }
}

impl Default for Image {
    fn default() -> Self {
        Image::new(200, 130)
    }
}
