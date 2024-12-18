use advent_of_code::bin::util::grid::Grid;
use advent_of_code::bin::util::point::{Point, DOWN, RIGHT, UP};

advent_of_code::solution!(6);

const EMPTY: u8 = b'.';
const BLOCKED: u8 = b'#';
const START: u8 = b'^';
const VISITED: u8 = b'X';
const VISITED_UP: u8 = b'1';
const VISITED_RIGHT: u8 = b'2';
const VISITED_DOWN: u8 = b'3';
const VISITED_LEFT: u8 = b'4';

fn has_loop(
    grid: &mut Grid<u8>,
    mut current: Point,
    mut direction: Point,
    step_limit: u32,
) -> bool {
    let mut steps = 0;

    loop {
        let visited_mark = if direction == UP {
            VISITED_UP
        } else if direction == RIGHT {
            VISITED_RIGHT
        } else if direction == DOWN {
            VISITED_DOWN
        } else {
            VISITED_LEFT
        };

        if grid[current] == visited_mark {
            return true;
        }

        let next_pos = current + direction;
        if !grid.contains(next_pos) {
            break;
        }

        match grid[next_pos] {
            EMPTY | VISITED_UP | VISITED_RIGHT | VISITED_DOWN | VISITED_LEFT => {
                grid[current] = visited_mark;
                current = next_pos;
                steps += 1;
            }
            BLOCKED | START => {
                direction = direction.clockwise();
            }
            _ => break,
        }

        if steps >= step_limit {
            break;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::parse_ascii(input);
    let mut direction = UP;
    let mut current = grid.find(START)?;
    let mut result = 1;

    grid[current] = VISITED;
    while grid.contains(current + direction) {
        if grid[current + direction] == BLOCKED {
            direction = direction.clockwise();
            continue;
        }
        current += direction;
        if grid[current] == EMPTY {
            grid[current] = VISITED;
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse_ascii(input);
    let mut setup_grid = grid.clone();
    let start = grid.find(START)?;
    let mut possible = 0;
    let limit = (grid.width() * grid.height()) as u32;

    let mut path_positions = Vec::new();
    let mut current = start;
    let mut direction = UP;

    setup_grid[current] = VISITED;
    path_positions.push(current);

    while setup_grid.contains(current + direction) {
        if setup_grid[current + direction] == BLOCKED {
            direction = direction.clockwise();
            continue;
        }
        current += direction;
        if setup_grid[current] == EMPTY {
            setup_grid[current] = VISITED;
            path_positions.push(current);
        }
    }

    for &point in &path_positions {
        let mut test_grid = grid.clone();
        test_grid[point] = BLOCKED;
        if has_loop(&mut test_grid, start, UP, limit) {
            possible += 1;
        }
    }

    Some(possible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
