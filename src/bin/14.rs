use advent_of_code::bin::util::bounding_box::BoundingBox;
use advent_of_code::bin::util::point::Point;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u64> {
    let mut robots: Vec<Point> = Vec::new();
    let mut velocities: Vec<Point> = Vec::new();
    // let dimensions: Point = Point::new(11, 7); // for the example input
    let dimensions: Point = Point::new(101, 103); // for the puzzle input
    let simulation_steps = 100;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let position: Vec<i32> = parts[0]
            .strip_prefix("p=")
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        let velocity: Vec<i32> = parts[1]
            .strip_prefix("v=")
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        robots.push(Point::new(position[0], position[1]));
        velocities.push(Point::new(velocity[0], velocity[1]));
    }

    for (robot, velocity) in robots.iter_mut().zip(velocities.iter()) {
        *robot += velocity.scale(simulation_steps);
        // wrap around the dimensions of the grid
        *robot += dimensions.scale(simulation_steps);
        robot.x %= dimensions.x;
        robot.y %= dimensions.y;
    }

    // bounding points for quadrants. Exclude the middle on the always uneven dimensions.
    let x_mid = dimensions.x / 2;
    let y_mid = dimensions.y / 2;

    let top_left = BoundingBox::new(Point::new(0, 0), Point::new(x_mid, y_mid));
    let top_right = BoundingBox::new(Point::new(x_mid + 1, 0), Point::new(dimensions.x, y_mid));
    let bottom_left = BoundingBox::new(Point::new(0, y_mid + 1), Point::new(x_mid, dimensions.y));
    let bottom_right = BoundingBox::new(
        Point::new(x_mid + 1, y_mid + 1),
        Point::new(dimensions.x, dimensions.y),
    );

    let mut top_left_count = 0;
    let mut top_right_count = 0;
    let mut bottom_left_count = 0;
    let mut bottom_right_count = 0;

    for robot in robots {
        if top_left.contains(robot) {
            top_left_count += 1;
        } else if top_right.contains(robot) {
            top_right_count += 1;
        } else if bottom_left.contains(robot) {
            bottom_left_count += 1;
        } else if bottom_right.contains(robot) {
            bottom_right_count += 1;
        }
    }

    Some(top_left_count * top_right_count * bottom_left_count * bottom_right_count)
}

fn robot_unique_positions(robots: &[Point]) -> u32 {
    let mut unique_positions_set = std::collections::HashSet::new();
    for robot in robots {
        unique_positions_set.insert(robot);
    }
    unique_positions_set.len() as u32
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut robots: Vec<Point> = Vec::new();
    let mut velocities: Vec<Point> = Vec::new();
    // let dimensions: Point = Point::new(11, 7); // for the example input
    let dimensions: Point = Point::new(101, 103); // for the puzzle input

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let position: Vec<i32> = parts[0]
            .strip_prefix("p=")
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        let velocity: Vec<i32> = parts[1]
            .strip_prefix("v=")
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        robots.push(Point::new(position[0], position[1]));
        velocities.push(Point::new(velocity[0], velocity[1]));
    }

    let mut seconds = 0;
    loop {
        seconds += 1;
        for (robot, velocity) in robots.iter_mut().zip(velocities.iter()) {
            *robot += *velocity;
            *robot += dimensions;
            robot.x %= dimensions.x;
            robot.y %= dimensions.y;
        }

        let unique_positions = robot_unique_positions(&robots);
        if unique_positions == robots.len() as u32 {
            return Some(seconds);
        }
    }
}

#[cfg(test)]
mod tests {
    /* use super::*;

    #[test] // when testing, change dimensions to 11, 7
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }*/
}
