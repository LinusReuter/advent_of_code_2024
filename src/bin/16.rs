use advent_of_code::bin::util::grid::Grid;
use advent_of_code::bin::util::point::{Point, RIGHT};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

advent_of_code::solution!(16);

struct State {
    position: Point,
    direction: Point,
    cost: u32,
}

impl Eq for State {}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    // Reverse ordering so that the smallest element is at the top of the heap
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn bfs(grid: &Grid<u8>, start: Point, goal: Point) -> Option<u64> {
    let mut heap = BinaryHeap::new();
    let mut visited = Grid::new(grid.width(), grid.height(), false);
    heap.push(State {
        position: start,
        direction: RIGHT,
        cost: 0,
    });

    while let Some(State {
        position,
        direction,
        cost,
    }) = heap.pop()
    {
        if position == goal {
            return Some(cost as u64);
        }
        if visited[position] {
            continue;
        }
        visited[position] = true;

        for (direction, price) in [
            (direction, 1),
            (direction.clockwise(), 1001),
            (direction.counter_clockwise(), 1001),
        ] {
            let next = position + direction;
            if grid[next] == b'#' {
                continue;
            }
            heap.push(State {
                position: next,
                direction,
                cost: cost + price,
            });
        }
    }
    None
}

fn bfs_cost_grid(grid: &Grid<u8>, start: Point, goal: Point) -> Grid<u32> {
    let mut cost_grid = Grid::new(grid.width(), grid.height(), 0);
    let mut heap = BinaryHeap::new();
    let mut visited = Grid::new(grid.width(), grid.height(), false);
    heap.push(State {
        position: start,
        direction: RIGHT,
        cost: 0,
    });

    while let Some(State {
        position,
        direction,
        cost,
    }) = heap.pop()
    {
        if visited[position] {
            continue;
        }

        cost_grid[position] = cost;
        if position == goal && direction != RIGHT {
            cost_grid[position] = cost + 1000;
        }

        visited[position] = true;

        for (direction, price) in [
            (direction, 1),
            (direction.clockwise(), 1001),
            (direction.counter_clockwise(), 1001),
        ] {
            let next = position + direction;

            if grid[next] == b'#' {
                continue;
            }
            heap.push(State {
                position: next,
                direction,
                cost: cost + price,
            });
        }
    }
    cost_grid
}

fn find_points_on_shortest_paths(grid: &Grid<u8>, start: Point, goal: Point) -> Option<u64> {
    let forward_costs = bfs_cost_grid(grid, start, goal);
    let backward_costs = bfs_cost_grid(grid, goal, start);
    let shortest_path_cost = forward_costs[goal];
    let mut result = 0;

    for combined_cost in forward_costs
        .iter()
        .zip(backward_costs.iter())
        .map(|(a, b)| *a + *b)
    {
        if combined_cost == shortest_path_cost || combined_cost == shortest_path_cost - 1000 {
            result += 1;
        }
    }
    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::parse_ascii(input);
    let start = Point::new(1, grid.height() - 2);
    let goal = Point::new(grid.width() - 2, 1);

    bfs(&grid, start, goal)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::parse_ascii(input);
    let start = Point::new(1, grid.height() - 2);
    let goal = Point::new(grid.width() - 2, 1);

    find_points_on_shortest_paths(&grid, start, goal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
