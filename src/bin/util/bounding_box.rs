// Implementation of BoundingBox to work with the point module.

use crate::bin::util::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl BoundingBox {
    #[must_use]
    #[inline]
    pub const fn new(top_left: Point, bottom_right: Point) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    #[must_use]
    #[inline]
    pub const fn with_dimensions(width: i32, height: i32) -> Self {
        Self {
            top_left: Point::new(0, 0),
            bottom_right: Point::new(width, height),
        }
    }

    // Returns weather the point is within the bounding box.
    // The top left corner is inclusive, the bottom right corner is exclusive.
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.top_left.x
            && point.x < self.bottom_right.x
            && point.y >= self.top_left.y
            && point.y < self.bottom_right.y
    }
}
