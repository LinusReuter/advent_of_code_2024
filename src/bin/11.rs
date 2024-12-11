use std::collections::HashMap;
use std::str::FromStr;

advent_of_code::solution!(11);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    engraving: u64,
    blinks_left: usize,
}

fn count_stones_after_blinks(
    engraving: u64,
    blinks: usize,
    cache: &mut HashMap<CacheKey, usize>,
) -> usize {
    let key = CacheKey {
        engraving,
        blinks_left: blinks,
    };

    if let Some(&count) = cache.get(&key) {
        return count;
    }

    if blinks == 0 {
        cache.insert(key, 1);
        return 1;
    }

    let engraving_str = engraving.to_string();

    // Rule 1: Replace 0 with 1
    if engraving == 0 {
        return count_stones_after_blinks(1, blinks - 1, cache);
    }

    // Rule 2: Split stones with even number of digits
    if engraving_str.len() % 2 == 0 {
        let mid = engraving_str.len() / 2;
        let left = u64::from_str(&engraving_str[..mid]).unwrap_or(0);
        let right = u64::from_str(&engraving_str[mid..]).unwrap_or(0);

        // Recursively count stones for left and right halves
        let count = count_stones_after_blinks(left, blinks - 1, cache)
            + count_stones_after_blinks(right, blinks - 1, cache);
        cache.insert(key, count);
        return count;
    }

    // Rule 3: Multiply by 2024
    let count = count_stones_after_blinks(engraving * 2024, blinks - 1, cache);
    cache.insert(key, count);
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let total_stones = input
        .split_whitespace()
        .map(|num| num.parse().unwrap_or(0))
        .map(|engraving| count_stones_after_blinks(engraving, 25, &mut HashMap::new()))
        .sum::<usize>();

    Some(total_stones)
}

pub fn part_two(input: &str) -> Option<usize> {
    let total_stones = input
        .split_whitespace()
        .map(|num| num.parse().unwrap_or(0))
        .map(|engraving| count_stones_after_blinks(engraving, 75, &mut HashMap::new()))
        .sum::<usize>();

    Some(total_stones)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
