use std::cmp;

advent_of_code::solution!(2);

fn is_safe(report: &[u32]) -> bool {
    report
        .windows(2)
        .map(|window| (window[0], window[1]))
        .try_fold(None, |order: Option<cmp::Ordering>, (a, b)| {
            let diff = (a as i32 - b as i32).abs();
            if !(1..=3).contains(&diff) {
                return None;
            }

            match order {
                None => Some(Option::from(a.cmp(&b))),
                Some(last_order) => {
                    if last_order != a.cmp(&b) {
                        return None;
                    }
                    Some(Option::from(last_order))
                }
            }
        })
        .is_some()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let report: Vec<u32> = line
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();
                is_safe(&report)
            })
            .filter(|&is_safe| is_safe)
            .count() as u32,
    )
}

fn is_safe_with_skip(report: &[u32]) -> bool {
    if is_safe(report) {
        return true;
    }

    // on unsafe reports, find the indexes of the elements that are not safe and try to skip them

    let diffs: Vec<i32> = report
        .windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)
        .collect();

    // find not safe indexes
    // first find if list is ascending or descending in majority by viewing up to 3 elements
    let is_increasing = diffs.iter().take(3).filter(|&&x| x > 0).count() > (diffs.len().min(3) / 2);

    let not_safe_indexes: Vec<usize> = diffs
        .iter()
        .enumerate()
        .filter_map(|(i, &diff)| {
            if is_increasing && !(1..=3).contains(&diff)
                || !is_increasing && !(-3..=-1).contains(&diff)
            {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    // Early return conditions
    if not_safe_indexes.len() > 2 {
        return false;
    }
    if not_safe_indexes.len() == 2 && not_safe_indexes[1] - not_safe_indexes[0] > 1 {
        return false;
    }

    for index in not_safe_indexes {
        // Create two sub-reports by skipping one element at a time:
        // - One skips the current element (at `index`).
        // - One skips the next element (at `index + 1`).
        let mut sub_report1 = report.to_vec();
        let mut sub_report2 = sub_report1.clone();

        sub_report1.remove(index);
        if index + 1 < sub_report2.len() {
            sub_report2.remove(index + 1);
        }

        // Step 4: Check if any of the sub-reports are safe
        if is_safe(&sub_report1) || is_safe(&sub_report2) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_count = input
        .lines()
        .filter_map(|line| {
            let report = line
                .split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok())
                .collect::<Vec<u32>>();
            if is_safe_with_skip(&report) {
                Some(())
            } else {
                None
            }
        })
        .count();
    Some(safe_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_is_safe_with_skip() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_safe_with_skip(&report)); // Safe without skips

        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_safe_with_skip(&report)); // Unsafe, no valid skip

        let report = vec![1, 3, 2, 4, 5];
        assert!(is_safe_with_skip(&report)); // Safe by skipping 2

        let report = vec![8, 6, 4, 4, 1];
        assert!(is_safe_with_skip(&report)); // Safe by skipping one 4

        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_safe_with_skip(&report)); // Unsafe, no valid skip

        let report = vec![1, 3, 6, 7, 9];
        assert!(is_safe_with_skip(&report)); // Safe without skips

        // Minimal inputs
        assert!(is_safe_with_skip(&[])); // Empty report
        assert!(is_safe_with_skip(&[5])); // Single element

        // Boundary values
        assert!(is_safe_with_skip(&[1, 4])); // Diff exactly 3
        assert!(!is_safe_with_skip(&[1, 5, 9])); // Diff above threshold (4), no valid skip
        assert!(!is_safe_with_skip(&[2, 2, 2])); // Repeated elements (monotonic)

        // Complex skips
        assert!(is_safe_with_skip(&[1, 2, 7, 3, 4])); // Skip 7 to maintain rules
        assert!(is_safe_with_skip(&[9, 2, 3, 5, 6])); // Skip 9 to maintain rules
        assert!(is_safe_with_skip(&[1, 2, 3, 6, 4, 6])); // Skip first 6 to maintain rules
        assert!(is_safe_with_skip(&[8, 6, 5, 7, 3, 1])); // Skip 7 to maintain rules
        assert!(!is_safe_with_skip(&[1, 22, 3, 4, 55, 6])); // Two skips required
        assert!(!is_safe_with_skip(&[1, 22, 33, 4, 5, 6, 7])); // Two skips required

        // Monotonicity violations
        assert!(is_safe_with_skip(&[3, 4, 6, 2, 7])); // Skip 2 to maintain monotonicity
        assert!(!is_safe_with_skip(&[10, 2, 3, 9, 4, 1])); // Multiple monotonicity violations

        // Alternating patterns
        assert!(!is_safe_with_skip(&[1, 3, 2, 4, 3, 5])); // Zig-zag pattern, invalid
    }
}
