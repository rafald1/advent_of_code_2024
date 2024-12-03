fn get_data(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Should open a file")
}

fn find_valid_multiplication_operations(corrupted_data: &str) -> Vec<(u32, u32)> {
    let mut multiplicands_and_multipliers: Vec<(u32, u32)> = Vec::new();
    let mut corrupted_data = corrupted_data;

    while let Some((_, corrupted_data_after_mul)) = corrupted_data.split_once("mul(") {
        corrupted_data = corrupted_data_after_mul;

        if let Some((numbers, _)) = corrupted_data.split_once(")") {
            if let Some((multiplicand, multiplier)) = numbers.split_once(",") {
                if let (Ok(multiplicand), Ok(multiplier)) =
                    (multiplicand.parse::<u32>(), multiplier.parse::<u32>())
                {
                    multiplicands_and_multipliers.push((multiplicand, multiplier));
                }
            }
        }
    }

    multiplicands_and_multipliers
}

fn process_multiplication_operations(multiplication_operations: &[(u32, u32)]) -> u64 {
    multiplication_operations
        .iter()
        .fold(0_u64, |acc, (multiplicand, multiplier)| {
            acc + (multiplicand * multiplier) as u64
        })
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let corrupted_data: String = get_data("./input/03.txt");
    let multiplications: Vec<(u32, u32)> = find_valid_multiplication_operations(&corrupted_data);
    let result: u64 = process_multiplication_operations(&multiplications);
    Ok(format!("Day 3 Mull It Over (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_03_mull_it_over::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let corrupted_data = get_data("./test_input/03_01.txt");
        let multiplications = find_valid_multiplication_operations(&corrupted_data);
        let result = process_multiplication_operations(&multiplications);
        assert_eq!(result, 161);
    }
}
