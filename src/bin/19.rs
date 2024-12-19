advent_of_code::solution!(19);
pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let patterns_line = lines.next()?.trim();
    let mut patterns: Vec<&[u8]> = patterns_line
        .split(',')
        .map(|s| {
            let s = s.trim();
            s.as_bytes()
        })
        .collect();

    patterns.sort_unstable_by_key(|a| a.len());

    let _ = lines.next()?;

    let designs: Vec<&[u8]> = lines.map(|line| line.trim().as_bytes()).collect();

    // Function to check if a design can be constructed
    fn can_construct(design: &[u8], patterns: &[&[u8]]) -> bool {
        let n = design.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 1..=n {
            for &pattern in patterns {
                let len = pattern.len();
                if len > i {
                    break;
                }
                if &design[i - len..i] == pattern && dp[i - len] {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[n]
    }

    let count = designs
        .iter()
        .filter(|&&d| can_construct(d, &patterns))
        .count() as u64;

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let patterns_line = lines.next()?.trim();
    let mut patterns: Vec<&[u8]> = patterns_line
        .split(',')
        .map(|s| {
            let s = s.trim();
            s.as_bytes()
        })
        .collect();

    patterns.sort_unstable_by_key(|a| a.len());

    let _ = lines.next()?;

    let designs: Vec<&[u8]> = lines.map(|line| line.trim().as_bytes()).collect();

    // Function to count the number of pattern combinations to construct a design
    fn can_construct(design: &[u8], patterns: &[&[u8]]) -> u64 {
        let n = design.len();
        let mut dp = vec![0u64; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for &pattern in patterns {
                let len = pattern.len();
                if len > i {
                    break;
                }
                if &design[i - len..i] == pattern {
                    dp[i] += dp[i - len];
                }
            }
        }

        dp[n]
    }

    let count = designs.iter().map(|&d| can_construct(d, &patterns)).sum();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
