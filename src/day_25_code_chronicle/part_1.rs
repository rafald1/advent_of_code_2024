fn process_data(file_path: &str) -> Vec<Vec<u8>> {
    std::fs::read_to_string(file_path)
        .expect("Should open a file")
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|chunk| {
            chunk
                .iter()
                .map(|line| {
                    line.chars()
                        .rev()
                        .enumerate()
                        .filter_map(|(i, char)| match char {
                            '#' => Some(2u8.pow(i as u32)),
                            '.' => None,
                            _ => panic!("Invalid input character: {}", char),
                        })
                        .sum()
                })
                .collect()
        })
        .collect()
}

fn count_keys_that_can_be_inserted_into_locks(locks_and_keys: Vec<Vec<u8>>) -> usize {
    let (locks, keys): (Vec<Vec<u8>>, Vec<Vec<u8>>) = locks_and_keys
        .into_iter()
        .partition(|lock_or_key| lock_or_key[0] == 0b11111);

    locks
        .iter()
        .flat_map(|lock| {
            keys.iter()
                .filter(|key| (1..=5).all(|i| !(lock[i] & key[i]) == 0b11111111))
        })
        .count()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let locks_and_keys: Vec<Vec<u8>> = process_data("./input/25.txt");
    let result: usize = count_keys_that_can_be_inserted_into_locks(locks_and_keys);
    Ok(format!("Day 25 Code Chronicle (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_25_code_chronicle::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let locks_and_keys = process_data("./test_input/25.txt");
        let result = count_keys_that_can_be_inserted_into_locks(locks_and_keys);
        assert_eq!(result, 3);
    }
}
