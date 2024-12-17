advent_of_code::solution!(17);

#[derive(Debug)]
struct Computer {
    registers: [i64; 3], // [A, B, C]
    instruction_pointer: usize,
    instructions: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    #[inline(always)]
    fn new(registers: [i64; 3], instructions: Vec<u8>) -> Self {
        Self {
            registers,
            instruction_pointer: 0,
            instructions,
            output: Vec::with_capacity(16),
        }
    }

    #[inline(always)]
    fn get_combo_value(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4 => self.registers[0], // A
            5 => self.registers[1], // B
            6 => self.registers[2], // C
            _ => panic!("Invalid combo operand"),
        }
    }

    #[inline(always)]
    fn adv(&mut self, operand: u8) {
        let power = 1 << self.get_combo_value(operand);
        self.registers[0] /= power;
    }

    #[inline(always)]
    fn bxl(&mut self, operand: u8) {
        self.registers[1] ^= operand as i64;
    }

    #[inline(always)]
    fn bst(&mut self, operand: u8) {
        self.registers[1] = self.get_combo_value(operand) & 7;
    }

    #[inline(always)]
    fn jnz(&mut self, operand: u8) {
        if self.registers[0] != 0 {
            self.instruction_pointer = operand as usize;
        } else {
            self.instruction_pointer += 2;
        }
    }

    #[inline(always)]
    fn bxc(&mut self, _operand: u8) {
        self.registers[1] ^= self.registers[2];
    }

    #[inline(always)]
    fn out(&mut self, operand: u8) {
        self.output.push((self.get_combo_value(operand) & 7) as u8);
    }

    #[inline(always)]
    fn bdv(&mut self, operand: u8) {
        let power = 1 << self.get_combo_value(operand);
        self.registers[1] = self.registers[0] / power;
    }

    #[inline(always)]
    fn cdv(&mut self, operand: u8) {
        let power = 1 << self.get_combo_value(operand);
        self.registers[2] = self.registers[0] / power;
    }

    fn run(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            let opcode = self.instructions[self.instruction_pointer];
            let operand = self.instructions[self.instruction_pointer + 1];

            match opcode {
                0 => {
                    self.adv(operand);
                    self.instruction_pointer += 2;
                }
                1 => {
                    self.bxl(operand);
                    self.instruction_pointer += 2;
                }
                2 => {
                    self.bst(operand);
                    self.instruction_pointer += 2;
                }
                3 => self.jnz(operand),
                4 => {
                    self.bxc(operand);
                    self.instruction_pointer += 2;
                }
                5 => {
                    self.out(operand);
                    self.instruction_pointer += 2;
                }
                6 => {
                    self.bdv(operand);
                    self.instruction_pointer += 2;
                }
                7 => {
                    self.cdv(operand);
                    self.instruction_pointer += 2;
                }
                _ => panic!("Invalid opcode"),
            }
        }
    }

    fn run_until_next_out(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            let opcode = self.instructions[self.instruction_pointer];
            let operand = self.instructions[self.instruction_pointer + 1];

            match opcode {
                0 => {
                    self.adv(operand);
                    self.instruction_pointer += 2;
                }
                1 => {
                    self.bxl(operand);
                    self.instruction_pointer += 2;
                }
                2 => {
                    self.bst(operand);
                    self.instruction_pointer += 2;
                }
                3 => {
                    self.jnz(operand);
                }
                4 => {
                    self.bxc(operand);
                    self.instruction_pointer += 2;
                }
                5 => {
                    self.out(operand);
                    self.instruction_pointer += 2;
                    break;
                }
                6 => {
                    self.bdv(operand);
                    self.instruction_pointer += 2;
                }
                7 => {
                    self.cdv(operand);
                    self.instruction_pointer += 2;
                }
                _ => panic!("Invalid opcode"),
            }
        }
    }

    /*
    // Helper to build part two
    fn analyze_program_structure(&self) {
        println!("\nProgram Structure Analysis:");
        println!("Total instructions: {}", self.instructions.len() / 2);

        // Find the jump instruction
        for i in (0..self.instructions.len()).step_by(2) {
            let opcode = self.instructions[i];
            let operand = self.instructions[i + 1];
            println!("Position {}: opcode={}, operand={}", i / 2, opcode, operand);

            if opcode == 3 {
                // jnz
                println!("Found jump at position {} to position {}", i / 2, operand);
            }
        }
    }

    fn analyze_a_transformations(&mut self) {
        println!("\nRegister A Transformations:");
        let initial_a = self.registers[0];
        let mut iteration = 0;
        let mut outputs = Vec::new();

        // Track A's value through one complete iteration
        while self.instruction_pointer < self.instructions.len() {
            let pos = self.instruction_pointer;
            let opcode = self.instructions[pos];
            let operand = self.instructions[pos + 1];

            let old_a = self.registers[0];

            match opcode {
                0 => {
                    // adv
                    self.adv(operand);
                    println!(
                        "Position {}: A divided by 2^{} ({} -> {})",
                        pos / 2,
                        self.get_combo_value(operand),
                        old_a,
                        self.registers[0]
                    );
                }
                5 => {
                    // out
                    self.out(operand);
                    outputs.push((self.get_combo_value(operand) & 7) as u8);
                    println!("Position {}: Output: {}", pos / 2, outputs.last().unwrap());
                }
                3 => {
                    // jnz
                    println!("Position {}: Jump check (A={})", pos / 2, self.registers[0]);
                    if self.registers[0] == 0 {
                        break;
                    }
                    iteration += 1;
                    println!("\nIteration {} complete", iteration);
                    if iteration > 1 {
                        break;
                    }
                }
                _ => {}
            }

            if opcode == 3 {
                if self.registers[0] != 0 {
                    self.instruction_pointer = operand as usize;
                } else {
                    self.instruction_pointer += 2;
                }
            } else {
                self.instruction_pointer += 2;
            }
        }

        println!("\nSummary:");
        println!("Initial A: {}", initial_a);
        println!("Final A: {}", self.registers[0]);
        println!("Outputs generated: {:?}", outputs);
    }
    */
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();

    // Parse registers
    let reg_a = lines
        .next()?
        .trim_start_matches("Register A: ")
        .parse::<i64>()
        .ok()?;
    let reg_b = lines
        .next()?
        .trim_start_matches("Register B: ")
        .parse::<i64>()
        .ok()?;
    let reg_c = lines
        .next()?
        .trim_start_matches("Register C: ")
        .parse::<i64>()
        .ok()?;

    // Skip empty line
    lines.next()?;

    // Parse program
    let program = lines
        .next()?
        .trim_start_matches("Program: ")
        .split(',')
        .map(|n| n.trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut computer = Computer::new([reg_a, reg_b, reg_c], program);
    computer.run();

    // Convert output to string
    Some(
        computer
            .output
            .iter()
            .map(|&n| n.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    // skip register a
    lines.next()?;
    let reg_b = lines
        .next()?
        .trim_start_matches("Register B: ")
        .parse::<i64>()
        .ok()?;
    let reg_c = lines
        .next()?
        .trim_start_matches("Register C: ")
        .parse::<i64>()
        .ok()?;

    lines.next()?;

    let program = lines
        .next()?
        .trim_start_matches("Program: ")
        .split(',')
        .map(|n| n.trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let expected_output = program.clone();

    let mut valid_a_values = vec![0];

    for &out_elem in expected_output.iter().rev() {
        let mut new_valid_a_values = Vec::new();
        for &a in valid_a_values.iter() {
            for n in 0..8 {
                let a = (a << 3) | n;
                let mut computer = Computer::new([a, reg_b, reg_c], program.clone());
                computer.run_until_next_out();

                if computer.output.first() == Some(&out_elem) {
                    new_valid_a_values.push(a);
                }
            }
        }
        valid_a_values = new_valid_a_values;
    }

    Some(*valid_a_values.iter().min().unwrap() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
