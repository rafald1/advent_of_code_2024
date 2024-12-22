use std::collections::{HashMap, HashSet};

fn process_data(file_path: &str) -> Vec<u64> {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");

    file_content
        .lines()
        .map(|line| line.parse().expect("Should be valid u64 number"))
        .collect()
}

fn modify_secret_number(secret_number: &mut u64) {
    *secret_number ^= (*secret_number * 64) % 16777216;
    *secret_number ^= *secret_number / 32;
    *secret_number ^= (*secret_number * 2048) % 16777216;
}

fn find_sequence_with_most_bananas(secret_numbers: &mut [u64]) -> u32 {
    let mut sequence_total_bananas: HashMap<Vec<i8>, u32> = HashMap::new();

    for secret_number in secret_numbers.iter_mut() {
        let mut buyer_prices: Vec<i8> = vec![(*secret_number % 10) as i8];

        for _ in 0..2000 {
            modify_secret_number(secret_number);
            buyer_prices.push((*secret_number % 10) as i8);
        }

        let mut seen: HashSet<Vec<i8>> = HashSet::new();

        for five_prices in buyer_prices.windows(5) {
            let price_diff: Vec<i8> = five_prices
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect();

            if seen.insert(price_diff.clone()) {
                let sell_price: u32 = *five_prices.last().expect("Should contain 5 values") as u32;
                sequence_total_bananas
                    .entry(price_diff)
                    .and_modify(|total| *total += sell_price)
                    .or_insert(sell_price);
            }
        }
    }

    *sequence_total_bananas.values().max().unwrap_or(&0)
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let mut secret_numbers: Vec<u64> = process_data("./input/22.txt");
    let result: u32 = find_sequence_with_most_bananas(&mut secret_numbers);
    Ok(format!("Day 22 Monkey Market (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_22_monkey_market::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let mut secret_numbers = process_data("./test_input/22_02.txt");
        let result = find_sequence_with_most_bananas(&mut secret_numbers);
        assert_eq!(result, 23);
    }
}
