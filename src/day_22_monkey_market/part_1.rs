fn process_data(file_path: &str) -> Vec<u64> {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");

    file_content
        .lines()
        .map(|line| line.parse().expect("Should be valid u64 number"))
        .collect()
}

fn generate_secret_number(num: u64, n: u32) -> u64 {
    let mut secret_number = num;

    for _ in 0..n {
        secret_number ^= (secret_number * 64) % 16777216;
        secret_number ^= secret_number / 32;
        secret_number ^= (secret_number * 2048) % 16777216;
    }

    secret_number
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let secret_numbers: Vec<u64> = process_data("./input/22.txt");
    let result: u64 = secret_numbers
        .iter()
        .map(|&secret_number| generate_secret_number(secret_number, 2000))
        .sum();
    Ok(format!("Day 22 Monkey Market (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_22_monkey_market::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let secret_numbers = process_data("./test_input/22_01.txt");
        let result: u64 = secret_numbers
            .iter()
            .map(|&secret_number| generate_secret_number(secret_number, 2000))
            .sum();
        assert_eq!(result, 37327623);
    }
}
