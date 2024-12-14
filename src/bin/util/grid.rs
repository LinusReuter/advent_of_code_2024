// 2d grid backed by a single vector. Designed to be used with the Point struct.
use crate::bin::util::point::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    width: i32,
    height: i32,
    data: Vec<T>,
}

impl Grid<u8> {
    #[must_use]
    #[inline]
    pub fn parse_ascii(input: &str) -> Self {
        let bytes: Vec<_> = input.lines().map(str::as_bytes).collect();
        let height = bytes.len() as i32;
        let width = bytes[0].len() as i32;
        let mut data = Vec::with_capacity((width * height) as usize);
        bytes.iter().for_each(|line| data.extend_from_slice(line));
        Self {
            width,
            height,
            data,
        }
    }
}

impl<T> Grid<T> {
    #[must_use]
    #[inline]
    pub fn new(width: i32, height: i32, fill: T) -> Self
    where
        T: Clone,
    {
        Self {
            width,
            height,
            data: vec![fill; (width * height) as usize],
        }
    }

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    #[inline]
    pub fn find(&self, to_find: T) -> Option<Point>
    where
        T: PartialEq + Copy,
    {
        let to_point =
            |index: usize| Point::new((index as i32) % self.width, (index as i32) / self.width);
        self.data.iter().position(|&x| x == to_find).map(to_point)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> {
        self.data.iter().enumerate().map(move |(i, value)| {
            (
                Point::new(i as i32 % self.width, i as i32 / self.width),
                value,
            )
        })
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, point: Point) -> &Self::Output {
        &self.data[(point.y * self.width + point.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.data[(point.y * self.width + point.x) as usize]
    }
}
