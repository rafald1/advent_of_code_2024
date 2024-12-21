use std::collections::{HashMap, VecDeque};

fn process_data(file_path: &str) -> Vec<String> {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");

    file_content.lines().map(|line| line.to_owned()).collect()
}

fn get_x_y_key_positions(keypad: &[&str]) -> HashMap<char, (u8, u8)> {
    let mut key_positions: HashMap<char, (u8, u8)> = HashMap::new();

    for (y, row) in keypad.iter().enumerate() {
        for (x, key) in row.chars().enumerate() {
            if key != ' ' {
                key_positions.insert(key, (x as u8, y as u8));
            }
        }
    }

    key_positions
}

fn get_valid_neighbours(keypad: &[&str], (x, y): (u8, u8)) -> Vec<((u8, u8), char)> {
    [
        ((x + 1, y), '>'),
        ((x, y + 1), 'v'),
        ((x.wrapping_sub(1), y), '<'),
        ((x, y.wrapping_sub(1)), '^'),
    ]
    .into_iter()
    .filter_map(|((nx, ny), direction)| {
        keypad
            .get(ny as usize)
            .and_then(|row| row.chars().nth(nx as usize))
            .filter(|&key| key != ' ')
            .map(|_| ((nx, ny), direction))
    })
    .collect()
}

fn find_minimal_possible_paths(keypad: &[&str], start: (u8, u8), end: (u8, u8)) -> Vec<String> {
    let mut possible_paths: Vec<String> = Vec::new();
    let mut queue: VecDeque<((u8, u8), String)> = VecDeque::from([(start, String::new())]);
    let mut minimal_path_length: u8 = u8::MAX;

    while let Some(((x, y), sequence)) = queue.pop_front() {
        if minimal_path_length < sequence.len() as u8 + 1 {
            return possible_paths;
        }

        for ((nx, ny), direction) in get_valid_neighbours(keypad, (x, y)) {
            let mut next_sequence = sequence.clone();
            next_sequence.push(direction);

            if (nx, ny) == end {
                minimal_path_length = next_sequence.len() as u8;
                next_sequence.push('A');
                possible_paths.push(next_sequence);
            } else {
                queue.push_back(((nx, ny), next_sequence));
            }
        }
    }

    possible_paths
}

fn get_possible_sequences_between_two_keys(keypad: &[&str]) -> HashMap<(char, char), Vec<String>> {
    let key_positions = get_x_y_key_positions(keypad);
    let mut sequences_from_to = HashMap::new();

    for (&key1, &from) in &key_positions {
        for (&key2, &to) in &key_positions {
            let paths = if key1 == key2 {
                vec![String::from('A')]
            } else {
                find_minimal_possible_paths(keypad, from, to)
            };
            sequences_from_to.insert((key1, key2), paths);
        }
    }

    sequences_from_to
}

fn create_from_to_pairs(sequence: &str) -> Vec<(char, char)> {
    format!("A{}", sequence)
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|pair| (pair[0], pair[1]))
        .collect()
}

fn create_all_possible_sequences(
    from_to_pairs: &[(char, char)],
    sequences: &HashMap<(char, char), Vec<String>>,
) -> Vec<String> {
    let all_possible_paths: Vec<_> = from_to_pairs
        .iter()
        .filter_map(|pair| sequences.get(pair))
        .cloned()
        .collect();

    fn create(
        paths: &[Vec<String>],
        index: usize,
        current_sequence: &mut String,
        all_possible_sequences: &mut Vec<String>,
    ) {
        if index == paths.len() {
            all_possible_sequences.push(current_sequence.clone());
            return;
        }

        for path in &paths[index] {
            let original_length = current_sequence.len();
            current_sequence.push_str(path);
            create(paths, index + 1, current_sequence, all_possible_sequences);
            current_sequence.truncate(original_length);
        }
    }

    let mut all_possible_sequences: Vec<String> = Vec::new();
    create(
        &all_possible_paths,
        0,
        &mut String::new(),
        &mut all_possible_sequences,
    );

    all_possible_sequences
}

fn calculate_code_complexities_sum(keypad_inputs: &[String]) -> usize {
    let numeric_keypad: [&str; 4] = ["789", "456", "123", " 0A"];
    let sequences_between_numeric_keys: HashMap<(char, char), Vec<String>> =
        get_possible_sequences_between_two_keys(&numeric_keypad);
    let directional_keypad: [&str; 2] = [" ^A", "<v>"];
    let sequences_between_directional_keys: HashMap<(char, char), Vec<String>> =
        get_possible_sequences_between_two_keys(&directional_keypad);

    keypad_inputs
        .iter()
        .map(|keypad_input| {
            let input_pairs: Vec<(char, char)> = create_from_to_pairs(keypad_input);
            let mut possible_sequences: Vec<String> =
                create_all_possible_sequences(&input_pairs, &sequences_between_numeric_keys);

            for _ in 0..2 {
                let mut next_possible_sequences: Vec<String> = Vec::new();

                for sequence in &possible_sequences {
                    let input_pairs: Vec<(char, char)> = create_from_to_pairs(sequence);
                    let new_sequences: Vec<String> = create_all_possible_sequences(
                        &input_pairs,
                        &sequences_between_directional_keys,
                    );
                    next_possible_sequences.extend(new_sequences);
                }

                let min_length = next_possible_sequences
                    .iter()
                    .map(String::len)
                    .min()
                    .unwrap_or(0);
                possible_sequences = next_possible_sequences
                    .into_iter()
                    .filter(|seq| seq.len() == min_length)
                    .collect();
            }

            let numeric_part: usize = keypad_input[..keypad_input.len() - 1]
                .parse()
                .expect("Should contain number");
            possible_sequences[0].len() * numeric_part
        })
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let keypad_inputs: Vec<String> = process_data("./input/21.txt");
    let result: usize = calculate_code_complexities_sum(&keypad_inputs);
    Ok(format!("Day 21 Keypad Conundrum (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_21_keypad_conundrum::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let keypad_inputs = process_data("./test_input/21.txt");
        let result = calculate_code_complexities_sum(&keypad_inputs);
        assert_eq!(result, 126384);
    }
}
