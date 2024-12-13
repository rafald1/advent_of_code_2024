fn process_data(file_path: &str) -> Vec<[[u32; 2]; 3]> {
    let parse_line = |line: &&str| -> [u32; 2] {
        line.replace("=", "+")
            .split_once('+')
            .and_then(|(_, numbers)| numbers.split_once(", Y+"))
            .map(|(num_1, num_2)| {
                [
                    num_1.parse().expect("Should be a valid u32 number"),
                    num_2.parse().expect("Should be a valid u32 number"),
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
                .collect::<Vec<[u32; 2]>>()
                .try_into()
                .expect("Each chunk should contain exactly 3 entries of [u32; 2]")
        })
        .collect()
}

fn calculate_number_of_used_tokens(button_behaviors_and_prize_locations: &[[[u32; 2]; 3]]) -> u32 {
    let mut tokens_used: u32 = 0;

    for [[x1, y1], [x2, y2], [x_result, y_result]] in button_behaviors_and_prize_locations {
        for pressed_button_a_count in 0..=100 {
            if let Some(numerator) = x_result.checked_sub(pressed_button_a_count * x1) {
                if numerator % *x2 == 0 {
                    let pressed_button_b_count = numerator / *x2;

                    if pressed_button_b_count <= 100
                        && pressed_button_a_count * y1 + pressed_button_b_count * y2 == *y_result
                    {
                        tokens_used += 3 * pressed_button_a_count + pressed_button_b_count;
                    }
                }
            }
        }
    }

    tokens_used
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let button_behaviors_and_prize_locations: Vec<[[u32; 2]; 3]> = process_data("./input/13.txt");
    let result: u32 = calculate_number_of_used_tokens(&button_behaviors_and_prize_locations);
    Ok(format!("Day 13 Claw Contraption (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_13_claw_contraption::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let button_behaviors_and_prize_locations = process_data("./test_input/13.txt");
        let result = calculate_number_of_used_tokens(&button_behaviors_and_prize_locations);
        assert_eq!(result, 480);
    }
}
