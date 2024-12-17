fn process_data(file_path: &str) -> ([u64; 3], Vec<u8>) {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");
    let mut lines = file_content.lines();

    let registers = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (_, number) = line
                .split_once(": ")
                .expect("Line should contain \": \" once");
            number.parse().expect("Should be valid u64 number")
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("Should be three u64 numbers");

    let program = lines
        .by_ref()
        .flat_map(|line| {
            let (_, numbers) = line
                .split_once(": ")
                .expect("Line should contain \": \" once");
            numbers
                .split(',')
                .map(|number| number.parse().expect("Should be valid u8 number"))
        })
        .collect();

    (registers, program)
}

fn get_combo_operand([a, b, c]: [u64; 3], operand: u8) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid operand value: {}", operand),
    }
}

fn execute_program([a, b, c]: [u64; 3], program: &[u8]) -> Vec<u8> {
    let [mut a, mut b, mut c] = [a, b, c];
    let mut instruction_pointer: usize = 0;
    let mut output: Vec<u8> = Vec::new();

    loop {
        if instruction_pointer >= program.len() - 1 {
            break;
        }

        let instruction = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];

        match instruction {
            0 => a >>= get_combo_operand([a, b, c], operand),
            1 => b ^= operand as u64,
            2 => b = get_combo_operand([a, b, c], operand) % 8,
            3 => {
                if a > 0 {
                    instruction_pointer = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push((get_combo_operand([a, b, c], operand) % 8) as u8),
            6 => b = a >> get_combo_operand([a, b, c], operand),
            7 => c = a >> get_combo_operand([a, b, c], operand),
            _ => panic!("Invalid instruction number: {}", instruction),
        }

        instruction_pointer += 2;
    }

    output
}

fn find_a_register(registers: &[u64; 3], program: &[u8]) -> Vec<u64> {
    let [_, b, c]: [u64; 3] = *registers;
    let mut possible_a_registers = Vec::from([0]);

    for i in 1..=program.len() {
        let (_, expected_output) = program.split_at(program.len() - i);
        let mut next_possible_a_registers = Vec::new();

        for possible_a_register in possible_a_registers {
            for next_part_of_a in 0..=7 {
                let created_a_register = possible_a_register << 3 | next_part_of_a;
                let output: Vec<u8> = execute_program([created_a_register, b, c], program);

                if output == expected_output {
                    next_possible_a_registers.push(created_a_register);
                }
            }
        }

        possible_a_registers = next_possible_a_registers;
    }

    possible_a_registers
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let (registers, program): ([u64; 3], Vec<u8>) = process_data("./input/17.txt");
    let result = find_a_register(&registers, &program);
    Ok(format!(
        "Day 17 Chronospatial Computer (Part 2): {:?}.",
        result
    ))
}

#[cfg(test)]
mod tests {
    use crate::day_17_chronospatial_computer::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let (registers, program) = process_data("./test_input/17_02.txt");
        let result = find_a_register(&registers, &program);
        assert_eq!(result[0], 117440_u64);
    }
}
