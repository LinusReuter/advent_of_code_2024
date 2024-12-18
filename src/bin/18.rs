use advent_of_code::bin::util::grid::Grid;
use advent_of_code::bin::util::point::{
    parse_direction, Point, DIRECTIONS_ORTHOGONAL, DOWN, LEFT, RIGHT, UP,
};
use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(18);

fn bfs(grid: &mut Grid<u8>, start: Point, end: Point) -> Option<u64> {
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));

    while let Some((pos, steps)) = queue.pop_front() {
        if pos == end {
            return Some(steps);
        }
        for dir in DIRECTIONS_ORTHOGONAL {
            let new_pos = pos + dir;
            if let Some(b'.') = grid.get(new_pos) {
                grid[new_pos] = b'X';
                queue.push_back((new_pos, steps + 1));
            }
        }
    }

    None
}

fn highlight_shortest_path(
    grid: &mut Grid<u8>,
    visited: &mut Grid<bool>,
    on_shortest_route: &mut Grid<bool>,
    start: Point,
    end: Point,
) -> bool {
    let mut queue = VecDeque::new();

    // reset
    on_shortest_route.reset(false);
    visited.reset(false);

    queue.push_back((start, 0));

    let mut found = false;

    while let Some((pos, steps)) = queue.pop_front() {
        if pos == end {
            found = true;
            break;
        }

        for dir in DIRECTIONS_ORTHOGONAL {
            let new_pos = pos + dir;
            // Check if the position is valid and not visited
            if let Some(val) = grid.get(new_pos) {
                if *val != b'#' && !visited[new_pos] {
                    visited[new_pos] = true; // Mark as visited before adding to queue
                    match dir {
                        UP => grid[new_pos] = b'v',
                        DOWN => grid[new_pos] = b'^',
                        LEFT => grid[new_pos] = b'>',
                        RIGHT => grid[new_pos] = b'<',
                        _ => unreachable!(),
                    }
                    queue.push_back((new_pos, steps + 1));
                }
            }
        }
    }

    if !found {
        return false;
    }

    let mut pos = end;
    while pos != start {
        if let Some(dir) = parse_direction(grid[pos]) {
            on_shortest_route[pos] = true;
            pos += dir;
        } else {
            break;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u64> {
    // let mut grid = Grid::new(7, 7, b'.'); // for the example
    let mut grid = Grid::new(71, 71, b'.');

    // let first_x_elements = 12; // for the example
    let first_x_elements = 1024; // for the input

    for line in input.lines().take(first_x_elements) {
        let pos = Point::from_str(line).unwrap();
        grid[pos] = b'#';
    }

    let start = Point::new(0, 0);
    let end = Point::new(grid.width() - 1, grid.height() - 1);
    bfs(&mut grid, start, end)
}

pub fn part_two(input: &str) -> Option<&str> {
    // let mut grid = Grid::new(7, 7, b'.'); // for the example
    let mut grid = Grid::new(71, 71, b'.');

    // let first_x_elements = 12; // for the example
    let first_x_elements = 1024; // for the input

    for line in input.lines().take(first_x_elements) {
        let pos = Point::from_str(line).unwrap();
        grid[pos] = b'#';
    }

    let start = Point::new(0, 0);
    let end = Point::new(grid.width() - 1, grid.height() - 1);

    let mut on_shortest_route = Grid::new(grid.width(), grid.height(), false);
    let mut visited = Grid::new(grid.width(), grid.height(), false);

    highlight_shortest_path(&mut grid, &mut on_shortest_route, &mut visited, start, end);
    for line in input.lines().skip(first_x_elements) {
        let pos = Point::from_str(line).unwrap();
        grid[pos] = b'#';
        if !on_shortest_route[pos] {
            continue;
        }
        if !highlight_shortest_path(&mut grid, &mut on_shortest_route, &mut visited, start, end) {
            return Some(line);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    /*use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&binding);
        assert_eq!(result, Some("6,1"));
    }*/
}
