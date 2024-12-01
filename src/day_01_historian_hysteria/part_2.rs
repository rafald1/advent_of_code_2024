use std::collections::HashMap;

fn process_data(file_path: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    let mut left_locations: Vec<u32> = Vec::new();
    let mut right_locations: Vec<u32> = Vec::new();

    for line in file_content.lines() {
        let (left, right) = line
            .split_once(' ')
            .expect("Line should contain two numbers separated by at least one space");

        let left: u32 = left.trim().parse().expect("Should parse the left number");
        let right: u32 = right.trim().parse().expect("Should parse the right number");

        left_locations.push(left);
        right_locations.push(right);
    }

    Ok((left_locations, right_locations))
}

fn calculate_similarity_score(left_locations: &[u32], right_locations: &[u32]) -> u64 {
    let mut right_location_frequency: HashMap<u32, u64> = HashMap::new();

    for &right_location in right_locations {
        *right_location_frequency.entry(right_location).or_insert(0) += 1;
    }

    left_locations
        .iter()
        .map(|&left_location| {
            *right_location_frequency
                .get(&left_location)
                .unwrap_or(&0_u64)
                * left_location as u64
        })
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let (left_locations, right_locations) = process_data("./input/01.txt")?;
    let result = calculate_similarity_score(&left_locations, &right_locations);
    Ok(format!("Day 1 Historian Hysteria (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_01_historian_hysteria::part_2::*;

    #[test]
    fn solve_with_test_data() -> Result<(), Box<dyn std::error::Error>> {
        let (left_locations, right_locations) = process_data("./test_input/01.txt")?;
        let result = calculate_similarity_score(&left_locations, &right_locations);
        assert_eq!(result, 31);
        Ok(())
    }
}
