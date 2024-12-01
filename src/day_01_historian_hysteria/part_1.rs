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

fn find_total_distance_between_locations(left_locations: &[u32], right_locations: &[u32]) -> u64 {
    left_locations
        .iter()
        .zip(right_locations.iter())
        .fold(0_u64, |acc, (&left, &right)| {
            acc + left.abs_diff(right) as u64
        })
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let (mut left_locations, mut right_locations) = process_data("./input/01.txt")?;
    left_locations.sort_unstable();
    right_locations.sort_unstable();
    let result = find_total_distance_between_locations(&left_locations, &right_locations);
    Ok(format!("Day 1 Historian Hysteria (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_01_historian_hysteria::part_1::*;

    #[test]
    fn solve_with_test_data() -> Result<(), Box<dyn std::error::Error>> {
        let (mut left_locations, mut right_locations) = process_data("./test_input/01.txt")?;
        left_locations.sort_unstable();
        right_locations.sort_unstable();
        let result = find_total_distance_between_locations(&left_locations, &right_locations);
        assert_eq!(result, 11);
        Ok(())
    }
}
