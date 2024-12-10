use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

struct HeightInfo1 {
    reachable_mountain_caps: HashSet<(i8, i8)>,
}

impl HeightInfo1 {
    fn new() -> Self {
        HeightInfo1 {
            reachable_mountain_caps: HashSet::new(),
        }
    }
}

struct HeightInfo2 {
    reachable_mountain_caps: u8,
}

impl HeightInfo2 {
    fn new() -> Self {
        HeightInfo2 {
            reachable_mountain_caps: 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut height_levels: Vec<HashMap<(i8, i8), HeightInfo1>> =
        (0..10).map(|_| HashMap::new()).collect();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap() as u8;
            let height_info = HeightInfo1::new();
            height_levels[height as usize].insert((x as i8, y as i8), height_info);
        }
    }

    for (pos, height_info) in height_levels[9].iter_mut() {
        height_info.reachable_mountain_caps.insert(*pos);
    }

    for i in (1..height_levels.len()).rev() {
        let (before, after) = height_levels.split_at_mut(i);
        let upper_level = &after[0];
        let pre_level = &mut before[i - 1];
        for ((x, y), height_info) in upper_level.iter() {
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                if let Some(neighbour) = pre_level.get_mut(&(nx, ny)) {
                    neighbour
                        .reachable_mountain_caps
                        .extend(height_info.reachable_mountain_caps.iter());
                }
            }
        }
    }

    let result: usize = height_levels[0]
        .values()
        .map(|height_info| height_info.reachable_mountain_caps.len())
        .sum();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut height_levels: Vec<HashMap<(i8, i8), HeightInfo2>> =
        (0..10).map(|_| HashMap::new()).collect();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap() as u8;
            let height_info = HeightInfo2::new();
            height_levels[height as usize].insert((x as i8, y as i8), height_info);
        }
    }

    for (_, height_info) in height_levels[9].iter_mut() {
        height_info.reachable_mountain_caps = 1;
    }

    for i in (1..height_levels.len()).rev() {
        let (before, after) = height_levels.split_at_mut(i);
        let upper_level = &after[0];
        let pre_level = &mut before[i - 1];
        for ((x, y), height_info) in upper_level.iter() {
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                if let Some(neighbour) = pre_level.get_mut(&(nx, ny)) {
                    neighbour.reachable_mountain_caps += height_info.reachable_mountain_caps;
                }
            }
        }
    }

    let result = height_levels[0]
        .values()
        .map(|height_info| height_info.reachable_mountain_caps as u32)
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
