fn process_data(file_path: &str) -> ([u32; 3], Vec<u8>) {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");
    let mut lines = file_content.lines();

    let registers = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (_, number) = line
                .split_once(": ")
                .expect("Line should contain \": \" once");
            number.parse().expect("Should be valid u32 number")
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("Should be three u32 numbers");

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

fn get_combo_operand([a, b, c]: [u32; 3], operand: u8) -> u32 {
    match operand {
        0..=3 => operand as u32,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid operand value: {}", operand),
    }
}

fn execute_program(registers: &[u32; 3], program: &[u8]) -> String {
    let [mut a, mut b, mut c]: [u32; 3] = *registers;
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
            1 => b ^= operand as u32,
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
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let (registers, program): ([u32; 3], Vec<u8>) = process_data("./input/17.txt");
    let result: String = execute_program(&registers, &program);
    Ok(format!(
        "Day 17 Chronospatial Computer (Part 1): {}.",
        result
    ))
}

#[cfg(test)]
mod tests {
    use crate::day_17_chronospatial_computer::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let (registers, program) = process_data("./test_input/17_01.txt");
        let result = execute_program(&registers, &program);
        assert_eq!(&result, "4,6,3,5,6,3,5,2,1,0");
    }
}
