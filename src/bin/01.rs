advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in input.lines() {
        // split the line into two numbers and add them to the list
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }

    list1.sort();
    list2.sort();
    let sum: u32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).unsigned_abs())
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];

    for line in input.lines() {
        // split the line into two numbers and add them to the list
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }

    list1.sort();
    list2.sort();

    let mut score: u32 = 0;
    let mut l2_index = 0;
    let mut l2_index_tailing = 0;
    for l1 in list1.iter() {
        while l2_index_tailing < list2.len() && list2[l2_index_tailing] < *l1 {
            l2_index_tailing += 1;
        }
        if l2_index < l2_index_tailing {
            l2_index = l2_index_tailing;
        }
        while l2_index < list2.len() && list2[l2_index] <= *l1 {
            l2_index += 1;
        }
        score += (l2_index - l2_index_tailing) as u32 * l1;
    }

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}