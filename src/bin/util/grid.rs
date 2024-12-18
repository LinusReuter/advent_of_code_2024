use std::intrinsics::write_bytes;
// 2d grid backed by a single vector. Designed to be used with the Point struct.
use crate::bin::util::point::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self[Point::new(x, y)] as char);
            }
            println!();
        }
    }
}

impl Grid<bool> {
    pub fn reset(&mut self, value: bool) {
        unsafe {
            write_bytes(self.data.as_mut_ptr(), value as u8, self.data.len());
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

    #[inline]
    pub fn swap(&mut self, a: Point, b: Point) {
        self.data.swap(
            (a.y * self.width + a.x) as usize,
            (b.y * self.width + b.x) as usize,
        );
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn iter_with_points(&self) -> impl Iterator<Item = (Point, &T)> {
        self.data.iter().enumerate().map(move |(i, value)| {
            (
                Point::new(i as i32 % self.width, i as i32 / self.width),
                value,
            )
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        if self.contains(point) {
            Some(&self[point])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut T> {
        if self.contains(point) {
            Some(&mut self[point])
        } else {
            None
        }
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
