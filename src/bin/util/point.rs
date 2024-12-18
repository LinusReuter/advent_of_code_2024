// A 2d point implementation.
// Designed to be used in the context of a 2d grid and movement on it.

use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use std::str::FromStr;

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const DIRECTIONS_ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[must_use]
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }

    #[inline]
    pub fn scale(self, factor: i32) -> Self {
        Point::new(self.x * factor, self.y * factor)
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut parts = str.split(',');
        let x = parts.next().ok_or(Err)?.parse::<i32>()?;
        let y = parts.next().ok_or(Err)?.parse::<i32>()?;
        Ok(Self::new(x, y))
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub fn parse_directions(input: &[u8]) -> Vec<Point> {
    input.iter().filter_map(|&c| parse_direction(c)).collect()
}

#[inline]
#[must_use]
pub const fn parse_direction(c: u8) -> Option<Point> {
    match c {
        b'^' => Some(UP),
        b'v' => Some(DOWN),
        b'>' => Some(RIGHT),
        b'<' => Some(LEFT),
        _ => None,
    }
}
