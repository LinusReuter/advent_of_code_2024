use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut unique_antinode_positions: HashSet<(u32, u32)> = HashSet::new();
    let dimensions: (u32, u32) = (
        input.lines().next()?.len() as u32,
        input.lines().count() as u32,
    );
    let mut antennas: HashMap<char, Vec<(u32, u32)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_alphanumeric() {
                antennas
                    .entry(c)
                    .or_default()
                    .push((x as u32, y as u32));
            }
        }
    }

    for (_, positions) in antennas.iter() {
        // iterate over all pairs of positions, excluding the same position
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;
                // find the two points where the one is twice as far from the first as the second
                let x3 = x2 as i32 + dx;
                let y3 = y2 as i32 + dy;
                let x4 = x1 as i32 - dx;
                let y4 = y1 as i32 - dy;
                if x3 >= 0 && x3 < dimensions.0 as i32 && y3 >= 0 && y3 < dimensions.1 as i32 {
                    unique_antinode_positions.insert((x3 as u32, y3 as u32));
                }
                if x4 >= 0 && x4 < dimensions.0 as i32 && y4 >= 0 && y4 < dimensions.1 as i32 {
                    unique_antinode_positions.insert((x4 as u32, y4 as u32));
                }
            }
        }
    }

    Some(unique_antinode_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut unique_antinode_positions: HashSet<(u32, u32)> = HashSet::new();
    let dimensions: (u32, u32) = (
        input.lines().next()?.len() as u32,
        input.lines().count() as u32,
    );
    let mut antennas: HashMap<char, Vec<(u32, u32)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_alphanumeric() {
                antennas
                    .entry(c)
                    .or_default()
                    .push((x as u32, y as u32));
            }
        }
    }

    for (_, positions) in antennas.iter() {
        // iterate over all pairs of positions, excluding the same position
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;
                // find all points in dimensions that are inline with the two points including the two points
                let mut x = x2 as i32;
                let mut y = y2 as i32;
                while x >= 0 && x < dimensions.0 as i32 && y >= 0 && y < dimensions.1 as i32 {
                    unique_antinode_positions.insert((x as u32, y as u32));
                    x += dx;
                    y += dy;
                }
                x = x1 as i32;
                y = y1 as i32;
                while x >= 0 && x < dimensions.0 as i32 && y >= 0 && y < dimensions.1 as i32 {
                    unique_antinode_positions.insert((x as u32, y as u32));
                    x -= dx;
                    y -= dy;
                }
            }
        }
    }

    Some(unique_antinode_positions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
