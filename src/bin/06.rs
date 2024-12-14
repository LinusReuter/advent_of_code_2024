use advent_of_code::bin::util::grid::Grid;
use advent_of_code::bin::util::point::UP;
use std::cmp::PartialEq;

advent_of_code::solution!(6);

const EMPTY: u8 = b'.';
const BLOCKED: u8 = b'#';
const START: u8 = b'^';
const VISITED: u8 = b'X';

#[derive(Clone)]
enum PositionStatus {
    Empty,
    Blocked,
    Visited,
}

impl PartialEq for PositionStatus {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (PositionStatus::Empty, PositionStatus::Empty)
                | (PositionStatus::Blocked, PositionStatus::Blocked)
                | (PositionStatus::Visited, PositionStatus::Visited)
        )
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Up)
                | (Direction::Down, Direction::Down)
                | (Direction::Left, Direction::Left)
                | (Direction::Right, Direction::Right)
        )
    }
}

fn puzzle_has_loop(
    grid: &mut [Vec<(PositionStatus, Direction)>],
    current: (usize, usize),
    direction: Direction,
    step_limit: u32,
) -> bool {
    let mut current = current;
    let mut direction = direction;
    let mut steps = 0;

    loop {
        let current_pos = grid[current.0][current.1].clone();
        if current_pos == (PositionStatus::Visited, direction) {
            return true;
        }

        let next_pos = match direction {
            Direction::Up => {
                if current.0 == 0 {
                    break;
                }
                (current.0 - 1, current.1)
            }
            Direction::Down => {
                if current.0 == grid.len() - 1 {
                    break;
                }
                (current.0 + 1, current.1)
            }
            Direction::Left => {
                if current.1 == 0 {
                    break;
                }
                (current.0, current.1 - 1)
            }
            Direction::Right => {
                if current.1 == grid[0].len() - 1 {
                    break;
                }
                (current.0, current.1 + 1)
            }
        };

        match grid[next_pos.0][next_pos.1] {
            (PositionStatus::Empty, _) | (PositionStatus::Visited, _) => {
                grid[current.0][current.1] = (PositionStatus::Visited, direction);
                current = next_pos;
                steps += 1;
            }
            (PositionStatus::Blocked, _) => {
                // Turn right
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
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
    let puzzle_dimensions = (input.lines().count(), input.lines().next()?.len());
    let mut grid = vec![
        vec![(PositionStatus::Empty, Direction::Up); puzzle_dimensions.1];
        puzzle_dimensions.0
    ];
    let mut current = (0, 0);
    let direction = Direction::Up;
    let limit = puzzle_dimensions.0 as u32 * puzzle_dimensions.1 as u32;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => grid[y][x] = (PositionStatus::Empty, Direction::Up),
            '#' => grid[y][x] = (PositionStatus::Blocked, Direction::Up),
            '^' => {
                grid[y][x] = (PositionStatus::Empty, Direction::Up);
                current = (y, x);
            }
            _ => panic!("Invalid character in input"),
        });
    });

    let mut possible = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == (PositionStatus::Empty, Direction::Up) {
                let mut grid = grid.clone();
                grid[i][j] = (PositionStatus::Blocked, Direction::Up);
                if puzzle_has_loop(&mut grid, current, direction, limit) {
                    possible += 1;
                }
            }
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
