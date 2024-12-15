use advent_of_code::bin::util::grid::Grid;
use advent_of_code::bin::util::point::{parse_directions, Point, DOWN, LEFT, RIGHT, UP};

advent_of_code::solution!(15);

fn parse(input: &str) -> (Grid<u8>, Vec<Point>) {
    let mut parts = input.split("\n\n");
    let grid = Grid::parse_ascii(parts.next().unwrap());
    let directions = parse_directions(parts.next().unwrap().as_bytes());
    (grid, directions)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut grid, directions) = parse(input);
    let mut position = grid.find(b'@').unwrap();

    for direction in directions {
        push_single(&mut grid, &mut position, direction);
    }

    let mut result = 0;
    for (point, &value) in grid.iter() {
        if value == b'O' {
            result += (point.x + point.y * 100) as u64;
        }
    }
    Some(result)
}

fn push_single(grid: &mut Grid<u8>, position: &mut Point, direction: Point) {
    let next = *position + direction;
    // find next b'.' or b'#' in the direction
    let mut limit = (1..)
        .map(|i| *position + direction * i)
        .find(|&p| grid[p] == b'#' || grid[p] == b'.')
        .unwrap();

    if grid[limit] == b'#' {
        return;
    }
    // move all elements including the position to excluding the limit one step in the direction
    loop {
        grid.swap(limit, limit - direction);
        limit -= direction;
        if limit == *position {
            break;
        }
    }

    *position = next;
}

fn push_wide(
    grid: &mut Grid<u8>,
    position: &mut Point,
    direction: Point,
    to_move: &mut Vec<Point>,
    seen: &mut Grid<u16>,
    seen_id: u16,
) {
    to_move.clear();
    let mut index = 0;
    to_move.push(*position);

    while index < to_move.len() {
        let current = to_move[index];
        let next = current + direction;
        index += 1;

        let partner = match grid[next] {
            b'#' => return,
            b'[' => RIGHT,
            b']' => LEFT,
            _ => continue,
        };

        if seen[next] != seen_id {
            seen[next] = seen_id;
            to_move.push(next);
        }
        let next_partner = next + partner;
        if seen[next_partner] != seen_id {
            seen[next_partner] = seen_id;
            to_move.push(next_partner);
        }
    }

    for &point in to_move.iter().rev() {
        grid.swap(point, point + direction);
    }

    *position += direction;
}

fn widen_grid(grid: &Grid<u8>) -> Grid<u8> {
    let mut new_grid = Grid::new(grid.width() * 2, grid.height(), b'.');
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let (l, r) = match grid[Point::new(x, y)] {
                b'#' => (b'#', b'#'),
                b'@' => (b'@', b'.'),
                b'O' => (b'[', b']'),
                _ => continue,
            };
            new_grid[Point::new(2 * x, y)] = l;
            new_grid[Point::new(2 * x + 1, y)] = r;
        }
    }
    new_grid
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut grid, directions) = parse(input);
    grid = widen_grid(&grid);
    let mut position = grid.find(b'@').unwrap();

    let mut boxes_to_move: Vec<Point> = Vec::new();
    let mut seen = Grid::new(grid.width(), grid.height(), u16::MAX);

    for (seen_id, direction) in directions.into_iter().enumerate() {
        match direction {
            UP | DOWN => push_wide(
                &mut grid,
                &mut position,
                direction,
                &mut boxes_to_move,
                &mut seen,
                seen_id as u16,
            ),
            LEFT | RIGHT => push_single(&mut grid, &mut position, direction),
            _ => unreachable!(),
        }
    }

    let mut result = 0;
    for (point, &value) in grid.iter() {
        if value == b'[' {
            result += (point.x + point.y * 100) as u64;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
