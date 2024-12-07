fn process_data(file_path: &str) -> Vec<Vec<u64>> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .map(|line| {
            line.replace(":", "")
                .split(" ")
                .map(|num| num.parse().expect("Should be a valid u32 number"))
                .collect()
        })
        .collect()
}

fn combine(a: u64, b: u64) -> u64 {
    let mut multiplier = 10;

    while multiplier <= b {
        multiplier *= 10;
    }

    a * multiplier + b
}

fn is_equation_valid(equation: &[u64], current_value: u64, index: usize) -> bool {
    let expected_result = equation[0];

    match equation.get(index + 1) {
        None => {
            if current_value == expected_result {
                return true;
            }

            false
        }
        Some(next_value) => {
            if current_value > expected_result {
                return false;
            }

            is_equation_valid(equation, current_value + next_value, index + 1)
                || is_equation_valid(equation, current_value * next_value, index + 1)
                || is_equation_valid(equation, combine(current_value, *next_value), index + 1)
        }
    }
}

fn sum_valid_operations(equations: &[Vec<u64>]) -> u64 {
    equations
        .iter()
        .filter(|equation| is_equation_valid(equation, equation[1], 1))
        .map(|equation| equation[0])
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let equations = process_data("./input/07.txt");
    let result: u64 = sum_valid_operations(&equations);
    Ok(format!("Day 7 Bridge Repair (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_07_bridge_repair::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let equations = process_data("./test_input/07.txt");
        let result: u64 = sum_valid_operations(&equations);
        assert_eq!(result, 11387);
    }
}
