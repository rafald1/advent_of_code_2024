use std::collections::HashMap;

fn process_data(file_path: &str) -> HashMap<u64, u64> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .split_whitespace()
        .map(|num| num.parse::<u64>().expect("Should be a valid u64 number"))
        .fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_default() += 1;
            acc
        })
}

fn digit_count(number: u64) -> u32 {
    number.checked_ilog10().unwrap_or(0) + 1
}

fn split_in_half(number: u64, digit_count: u32) -> [u64; 2] {
    let divisor = 10u64.pow(digit_count / 2);
    [number / divisor, number % divisor]
}

fn blink_n_times(stones: HashMap<u64, u64>, n: u32) -> u64 {
    let mut current_stones = stones;

    for _ in 0..n {
        let mut next_stones: HashMap<u64, u64> = HashMap::new();

        for (&stone_number, &occurrence_count) in &current_stones {
            let digit_count = digit_count(stone_number);

            match stone_number {
                0 => *next_stones.entry(1).or_default() += occurrence_count,
                num if digit_count % 2 == 0 => split_in_half(num, digit_count)
                    .iter()
                    .for_each(|&half| *next_stones.entry(half).or_default() += occurrence_count),
                num => *next_stones.entry(num * 2024).or_default() += occurrence_count,
            };
        }

        current_stones = next_stones;
    }

    current_stones.values().sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let stones: HashMap<u64, u64> = process_data("./input/11.txt");
    let result: u64 = blink_n_times(stones, 75);
    Ok(format!("Day 11 Plutonian Pebbles (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_11_plutonian_pebbles::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let stones = process_data("./test_input/11.txt");
        let result = blink_n_times(stones, 25);
        assert_eq!(result, 55312);
    }
}
