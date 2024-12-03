advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // using regex as fast solution, for performance switch to carry ahead parsing.
    let mul_re = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut sum = 0;
    for mul in mul_re.captures_iter(input) {
        let mut nums = mul[0]
            .split(['(', ',', ')'])
            .filter_map(|n| n.parse::<u32>().ok());
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        sum += a * b;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // using regex as fast solution, for performance switch to carry ahead parsing.
    let re = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut active = true;

    for command in re.captures_iter(input) {
        match &command[0] {
            "do()" => active = true,
            "don't()" => active = false,
            _ => {
                if active {
                    let mut nums = command[0]
                        .split(['(', ',', ')'])
                        .filter_map(|n| n.parse::<u32>().ok());
                    let a = nums.next().unwrap();
                    let b = nums.next().unwrap();
                    sum += a * b;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
