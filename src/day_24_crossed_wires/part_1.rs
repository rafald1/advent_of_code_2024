use std::collections::{HashMap, VecDeque};

fn process_data(file_path: &str) -> (HashMap<String, bool>, VecDeque<[String; 4]>) {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");
    let mut lines = file_content.lines();

    let wire_values: HashMap<String, bool> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.split_once(": "))
        .map(|(key, value)| {
            (
                key.to_owned(),
                value.parse::<u8>().expect("Should be 0 or 1") != 0,
            )
        })
        .collect();

    let gates: VecDeque<[String; 4]> = lines
        .map(|line| {
            line.replace("-> ", "")
                .split(' ')
                .map(|part| part.to_owned())
                .collect::<Vec<String>>()
                .try_into()
                .expect("Should contain four String values")
        })
        .collect();

    (wire_values, gates)
}

fn evaluate(input1: bool, operation: &str, input2: bool) -> bool {
    match operation {
        "AND" => input1 & input2,
        "OR" => input1 | input2,
        "XOR" => input1 ^ input2,
        _ => unreachable!(),
    }
}

fn get_values_for_all_wires(
    wire_values: &mut HashMap<String, bool>,
    gates: &mut VecDeque<[String; 4]>,
) {
    while let Some([input1, operation, input2, output]) = gates.pop_front() {
        if let Some(&input1) = wire_values.get(&input1) {
            if let Some(&input2) = wire_values.get(&input2) {
                wire_values.insert(output, evaluate(input1, &operation, input2));
                continue;
            }
        }

        gates.push_back([input1, operation, input2, output]);
    }
}

fn get_decimal_number_output_on_z_wires(known_values: &HashMap<String, bool>) -> u64 {
    known_values
        .iter()
        .filter_map(|(key, value)| {
            key.strip_prefix('z').and_then(|number| {
                number
                    .parse::<u32>()
                    .ok()
                    .map(|exponent| 2_u64.pow(exponent) * *value as u64)
            })
        })
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let (mut wire_values, mut gates): (HashMap<String, bool>, VecDeque<[String; 4]>) =
        process_data("./input/24.txt");
    get_values_for_all_wires(&mut wire_values, &mut gates);
    let result: u64 = get_decimal_number_output_on_z_wires(&wire_values);
    Ok(format!("Day 24 Crossed Wires (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_24_crossed_wires::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let (mut wire_values, mut gates) = process_data("./test_input/24_01.txt");
        get_values_for_all_wires(&mut wire_values, &mut gates);
        let result = get_decimal_number_output_on_z_wires(&wire_values);
        assert_eq!(result, 4);

        let (mut wire_values, mut gates) = process_data("./test_input/24_02.txt");
        get_values_for_all_wires(&mut wire_values, &mut gates);
        let result = get_decimal_number_output_on_z_wires(&wire_values);
        assert_eq!(result, 2024);
    }
}
