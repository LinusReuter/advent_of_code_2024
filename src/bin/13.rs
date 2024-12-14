advent_of_code::solution!(13);

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    goal_x: i64,
    goal_y: i64,
}

fn parse_input(input: &str) -> Vec<Machine> {
    // every machine has tree input lines separated by a blank line
    let mut machines = Vec::new();
    for machine in input.split("\n\n") {
        let mut lines = machine.lines();
        // Example:
        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        let mut a = lines.next().unwrap();
        a = a.trim_start_matches("Button A: ");
        let ax = a.split(", ").next().unwrap().trim_start_matches("X+");
        let ay = a.split(", ").last().unwrap().trim_start_matches("Y+");
        let mut b = lines.next().unwrap();
        b = b.trim_start_matches("Button B: ");
        let bx = b.split(", ").next().unwrap().trim_start_matches("X+");
        let by = b.split(", ").last().unwrap().trim_start_matches("Y+");
        let mut prize = lines.next().unwrap();
        prize = prize.trim_start_matches("Prize: ");
        let goal_x = prize.split(", ").next().unwrap().trim_start_matches("X=");
        let goal_y = prize.split(", ").last().unwrap().trim_start_matches("Y=");

        machines.push(Machine {
            ax: ax.parse().unwrap(),
            ay: ay.parse().unwrap(),
            bx: bx.parse().unwrap(),
            by: by.parse().unwrap(),
            goal_x: goal_x.parse().unwrap(),
            goal_y: goal_y.parse().unwrap(),
        });
    }
    machines
}

fn play(mut machine: Machine, part_2: bool) -> i64 {
    if part_2 {
        machine.goal_x += 10000000000000;
        machine.goal_y += 10000000000000;
    }

    // no solution if determinant is zero
    let det = machine.ax * machine.by - machine.ay * machine.bx;
    if det == 0 {
        return 0;
    }

    // solve for Integer Solution of a and b
    let mut a = machine.goal_x * machine.by - machine.goal_y * machine.bx;
    let mut b = machine.ax * machine.goal_y - machine.ay * machine.goal_x;
    if a % det != 0 || b % det != 0 {
        return 0;
    }
    a /= det;
    b /= det;

    if part_2 || (a <= 100 && b <= 100) {
        3 * a + b
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let mut result = 0;
    for machine in machines {
        result += play(machine, false);
    }
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let mut result = 0;
    for machine in machines {
        result += play(machine, true);
    }
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }
}
