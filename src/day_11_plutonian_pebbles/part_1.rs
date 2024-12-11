fn process_data(file_path: &str) -> Vec<u64> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .split_whitespace()
        .map(|num| num.parse().expect("Should be a valid u64 number"))
        .collect()
}

fn digit_count(number: u64) -> u32 {
    number.checked_ilog10().unwrap_or(0) + 1
}

fn split_in_half(number: u64, digit_count: u32) -> [u64; 2] {
    let divisor = 10u64.pow(digit_count / 2);
    [number / divisor, number % divisor]
}

fn blink_n_times(stones: Vec<u64>, n: u32) -> usize {
    let mut current_stones: Vec<u64> = stones;

    for _ in 0..n {
        let mut next_stones: Vec<u64> = Vec::new();

        while let Some(stone_number) = current_stones.pop() {
            let digit_count = digit_count(stone_number);

            match stone_number {
                0 => next_stones.push(1),
                num if digit_count % 2 == 0 => next_stones.extend(split_in_half(num, digit_count)),
                num => next_stones.push(num * 2024),
            }
        }

        current_stones.append(&mut next_stones);
    }

    current_stones.len()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let stones: Vec<u64> = process_data("./input/11.txt");
    let result: usize = blink_n_times(stones, 25);
    Ok(format!("Day 11 Plutonian Pebbles (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_11_plutonian_pebbles::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let stones = process_data("./test_input/11.txt");
        let result = blink_n_times(stones, 25);
        assert_eq!(result, 55312);
    }
}
