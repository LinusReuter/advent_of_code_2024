advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let operators: [fn(u64, u64) -> u64; 2] = [|a, b| a + b, |a, b| a * b];

    Some(process_parts_with_operators(input, &operators))
}

pub fn part_two(input: &str) -> Option<u64> {
    let operators: [fn(u64, u64) -> u64; 3] = [
        |a, b| a + b,
        |a, b| a * b,
        |a, b| {
            let mut b_digits = b;
            let mut multiplier = 1;
            while b_digits > 0 {
                b_digits /= 10;
                multiplier *= 10;
            }
            a * multiplier + b
        },
    ];

    Some(process_parts_with_operators(input, &operators))
}

fn process_parts_with_operators(input: &str, operators: &[fn(u64, u64) -> u64]) -> u64 {
    let mut solution_sum = 0;

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let result_str = parts[0].strip_suffix(":").unwrap();
        let result = result_str.parse::<u64>().ok().unwrap();

        let numbers = parts
            .iter()
            .filter_map(|part| part.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        if numbers.len() == 1 {
            if numbers[0] == result {
                solution_sum += result;
            }
            continue;
        }

        // Recursive backtracking solver
        if solve(&numbers, result, operators) {
            solution_sum += result;
        }
    }

    solution_sum
}

// Recursive backtracking solver
fn solve(numbers: &[u64], target: u64, operators: &[fn(u64, u64) -> u64]) -> bool {
    fn backtrack(
        numbers: &[u64],
        target: u64,
        operators: &[fn(u64, u64) -> u64],
        current_index: usize,
        current_value: u64,
    ) -> bool {
        if current_index == numbers.len() {
            return current_value == target;
        }

        for &op in operators.iter() {
            let next_value = op(current_value, numbers[current_index]);

            // early fail if the next value is greater than the target
            if next_value > target {
                continue;
            }

            if backtrack(numbers, target, operators, current_index + 1, next_value) {
                return true;
            }
        }

        false
    }

    backtrack(numbers, target, operators, 1, numbers[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
