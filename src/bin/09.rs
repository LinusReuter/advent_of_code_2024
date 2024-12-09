use std::cmp::Reverse;
use std::collections::BinaryHeap;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let numbers: Vec<usize> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .collect();
    let sum: usize = numbers.iter().sum();
    let mut memory: Vec<Option<usize>> = vec![None; sum];
    let mut max_file_id = 0;
    let mut current_memory_position: usize = 0;
    for (i, number) in numbers.iter().enumerate() {
        if (i % 2) == 0 {
            // write file id to memory range
            for cell in memory
                .iter_mut()
                .skip(current_memory_position)
                .take(*number)
            {
                *cell = Some(max_file_id);
            }
            max_file_id += 1;
        }
        current_memory_position += *number;
    }

    // compaction: move right most full block to first free block in memory until no gaps are left
    // find last Some and move it to first None
    let mut last_file_block = memory.iter().rposition(|x| x.is_some()).unwrap();
    let mut first_free_block = memory.iter().position(|x| x.is_none()).unwrap();
    while last_file_block > first_free_block {
        memory[first_free_block] = memory[last_file_block];
        memory[last_file_block] = None;

        while last_file_block > 0 && memory[last_file_block - 1].is_none() {
            last_file_block -= 1;
        }
        last_file_block -= 1;

        while first_free_block < memory.len() && memory[first_free_block].is_some() {
            first_free_block += 1;
        }
    }

    let checksum = memory
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i * x.or_else(|| Some(0)).unwrap()));
    Some(checksum as u64)
}

pub fn part_two(input: &str) -> Option<usize> {
    let numbers: Vec<usize> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .collect();
    let sum: usize = numbers.iter().sum();
    let mut memory: Vec<Option<usize>> = vec![None; sum];
    let mut max_file_id = 0;
    let mut current_memory_position: usize = 0;
    let mut free_blocks_start_by_size = vec![BinaryHeap::new(); 10];
    let mut full_blocks: Vec<(usize, usize)> = vec![]; // pos / size
    for (i, number) in numbers.iter().enumerate() {
        if (i % 2) == 0 {
            // write file id to memory range
            for cell in memory
                .iter_mut()
                .skip(current_memory_position)
                .take(*number)
            {
                *cell = Some(max_file_id);
            }
            max_file_id += 1;
            full_blocks.push((current_memory_position, *number));
        } else {
            let heap = free_blocks_start_by_size.get_mut(*number).unwrap();
            heap.push(Reverse(current_memory_position));
        }
        current_memory_position += *number;
    }

    for (pos, size) in full_blocks.iter().rev() {
        // find the first free block of the same size or larger
        let mut free_pos = None;
        let mut free_pos_size = 0;
        for (i, heap) in free_blocks_start_by_size.iter().enumerate().skip(*size) {
            if let Some(Reverse(free_pos_candidate)) = heap.peek() {
                if *free_pos_candidate < free_pos.unwrap_or(usize::MAX)
                    && *free_pos_candidate < *pos
                {
                    free_pos = Some(*free_pos_candidate);
                    free_pos_size = i;
                }
            }
        }

        if let Some(free_pos) = free_pos {
            free_blocks_start_by_size[free_pos_size].pop();
            if free_pos_size > *size {
                free_blocks_start_by_size[free_pos_size - *size].push(Reverse(free_pos + *size));
            }

            for i in 0..*size {
                memory[free_pos + i] = memory[*pos + i];
                memory[*pos + i] = None;
            }
        }
    }

    let checksum = memory
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i * x.unwrap_or(0)));

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
