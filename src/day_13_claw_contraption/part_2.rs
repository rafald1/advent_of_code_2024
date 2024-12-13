fn process_data(file_path: &str) -> Vec<[[u64; 2]; 3]> {
    let parse_line = |line: &&str| -> [u64; 2] {
        line.replace("=", "+")
            .split_once('+')
            .and_then(|(_, numbers)| numbers.split_once(", Y+"))
            .map(|(num_1, num_2)| {
                [
                    num_1.parse().expect("Should be a valid u64 number"),
                    num_2.parse().expect("Should be a valid u64 number"),
                ]
            })
            .expect("Line should have the correct format")
    };

    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(parse_line) // Directly map `parse_line`, no need for extra closure
                .collect::<Vec<[u64; 2]>>()
                .try_into()
                .expect("Each chunk should contain exactly 3 entries of [u64; 2]")
        })
        .collect()
}

fn calculate_number_of_used_tokens(data: &[[[u64; 2]; 3]]) -> u64 {
    // a * x1 + b * x2 = result_x
    // a = (x_result - b * x2) / x1
    // a * y1 + b * y2 = y_result
    // ((x_result - b * x2) / x1) * y1 + b * y2 = y_result
    // b = (y_result * x1 - x_result * y1) / (x2 * y1 - x1 * y2)

    let mut tokens_used: u64 = 0;

    for [[x1, y1], [x2, y2], [x_result, y_result]] in data {
        // Ensure that x2 * y1 - x1 * y2 is not equal 0, as dividing by zero is undefined.
        if x2 * y1 == x1 * y2 {
            continue;
        }

        let x_result = 10000000000000 + x_result;
        let y_result = 10000000000000 + y_result;

        let numerator = (y_result * x1).abs_diff(x_result * y1);
        let denominator = (x2 * y1).abs_diff(x1 * y2);

        let b = match numerator % denominator == 0 {
            true => numerator / denominator,
            false => continue,
        };

        let a = (x_result - b * x2) / x1;

        tokens_used += 3 * a + b;
    }

    tokens_used
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let data: Vec<[[u64; 2]; 3]> = process_data("./input/13.txt");
    let result: u64 = calculate_number_of_used_tokens(&data);
    Ok(format!("Day 13 Claw Contraption (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_13_claw_contraption::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let button_behaviors_and_prize_locations = process_data("./test_input/13.txt");
        let result = calculate_number_of_used_tokens(&button_behaviors_and_prize_locations);
        assert_eq!(result, 875318608908);
    }
}
