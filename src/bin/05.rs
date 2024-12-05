use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    // input starts with 1 rule per line in the form xx|yy where xx and yy are two diget numbers
    // followed challenge input is a list of numbers separated by a comma per line

    let mut rules: Vec<Vec<u8>> = vec![Vec::new(); 100];
    let mut challenges = Vec::new();
    let mut is_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            is_rules = false;
            continue;
        }
        if is_rules {
            let rule: Vec<u8> = line.split('|').map(|x| x.parse().unwrap()).collect();
            rules[rule[0] as usize].push(rule[1]);
        } else {
            let challenge: Vec<u8> = line.split(',').map(|x| x.parse().unwrap()).collect();
            challenges.push(challenge);
        }
    }

    let mut puzzle_solution = 0;
    for challenge in challenges {
        let mut valid = true;
        let mut seen_bitmap: u128 = 0;
        'challenge: for &number in &challenge {
            seen_bitmap |= 1 << number;
            for &rule in &rules[number as usize] {
                if (seen_bitmap & (1 << rule)) != 0 {
                    valid = false;
                    break 'challenge;
                }
            }
        }
        if valid {
            let mid = challenge.len() / 2;
            puzzle_solution += challenge[mid] as u32;
        }
    }

    Some(puzzle_solution)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules: Vec<Vec<u8>> = vec![Vec::new(); 100];
    let mut rules_inverse: Vec<Vec<u8>> = vec![Vec::new(); 100];
    let mut challenges = Vec::new();
    let mut is_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            is_rules = false;
            continue;
        }
        if is_rules {
            let rule: Vec<u8> = line.split('|').map(|x| x.parse().unwrap()).collect();
            rules[rule[0] as usize].push(rule[1]);
            rules_inverse[rule[1] as usize].push(rule[0]);
        } else {
            let challenge: Vec<u8> = line.split(',').map(|x| x.parse().unwrap()).collect();
            challenges.push(challenge);
        }
    }

    let mut puzzle_solution = 0;
    for challenge in challenges {
        let mut valid = true;
        let mut seen_bitmap: u128 = 0;
        'challenge: for &number in &challenge {
            seen_bitmap |= 1 << number;
            for &rule in &rules[number as usize] {
                if (seen_bitmap & (1 << rule)) != 0 {
                    valid = false;
                    break 'challenge;
                }
            }
        }
        if !valid {
            let mut to_insert: HashSet<u8> = HashSet::new();
            for &number in &challenge {
                to_insert.insert(number);
            }
            let mut new_order = Vec::new();
            while !to_insert.is_empty() {
                let mut element = *to_insert.iter().next().unwrap();
                while !rules_inverse[element as usize].is_empty() {
                    let mut found = false;
                    for &rule in &rules_inverse[element as usize] {
                        if to_insert.contains(&rule) {
                            element = rule;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        break;
                    }
                }
                new_order.push(element);
                to_insert.remove(&element);
            }
            let mid = new_order.len() / 2;
            puzzle_solution += new_order[mid] as u32;
        }
    }

    Some(puzzle_solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
