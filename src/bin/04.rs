advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    // word puzzle, find horizontal, vertical and diagonal occurrences of 'XMAS' both forwards and backwards.

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut words = 0;
    // horizontal
    for row in &grid {
        for i in 0..row.len() {
            if i + 4 <= row.len() {
                let word: String = row[i..i + 4].iter().collect();
                if word == "XMAS" || word == "SAMX" {
                    words += 1;
                }
            }
        }
    }
    // vertical
    for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            if j + 4 <= grid.len() {
                let word: String = (0..4).map(|k| grid[j + k][i]).collect();
                if word == "XMAS" || word == "SAMX" {
                    words += 1;
                }
            }
        }
    }
    // diagonal
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if i + 4 <= grid.len() && j + 4 <= grid[0].len() {
                let word: String = (0..4).map(|k| grid[i + k][j + k]).collect();
                if word == "XMAS" || word == "SAMX" {
                    words += 1;
                }
            }
            if i + 4 <= grid.len() && j >= 3 {
                let word: String = (0..4).map(|k| grid[i + k][j - k]).collect();
                if word == "XMAS" || word == "SAMX" {
                    words += 1;
                }
            }
        }
    }

    Some(words)
}

pub fn part_two(input: &str) -> Option<u32> {
    // find patterns in the input:
    // M*S    M*M    S*M    S*S
    // *A* or *A* or *A* or *A* where * is any character
    // M*S    S*S    S*M    M*M

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut words = 0;
    for i in 0..grid.len() - 2 {
        for j in 0..grid[0].len() - 2 {
            if grid[i + 1][j + 1] == 'A'
                && ((grid[i][j] == 'M' && grid[i + 2][j + 2] == 'S')
                    || (grid[i][j] == 'S' && grid[i + 2][j + 2] == 'M'))
                && ((grid[i][j + 2] == 'M' && grid[i + 2][j] == 'S')
                    || (grid[i][j + 2] == 'S' && grid[i + 2][j] == 'M'))
            {
                words += 1;
            }
        }
    }

    Some(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
