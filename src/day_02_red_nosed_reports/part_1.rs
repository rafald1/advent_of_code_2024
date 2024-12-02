use std::cmp::Ordering;

fn process_data(file_path: &str) -> Vec<Vec<u8>> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<u8>().expect("Should parse the number"))
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &[u8]) -> bool {
    let mut has_ascending_order = false;
    let mut has_descending_order = false;

    for pair in report.windows(2) {
        match pair[0].cmp(&pair[1]) {
            Ordering::Less => has_ascending_order = true,
            Ordering::Greater => has_descending_order = true,
            Ordering::Equal => return false,
        }

        if pair[0].abs_diff(pair[1]) > 3 {
            return false;
        }
    }

    if has_ascending_order && has_descending_order {
        return false;
    }

    true
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let reports: Vec<Vec<u8>> = process_data("./input/02.txt");
    let result: usize = reports
        .into_iter()
        .filter(|report| is_report_safe(report))
        .count();
    Ok(format!("Day 2 Red-Nosed Reports (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_02_red_nosed_reports::part_1::*;

    #[test]
    fn solve_with_test_data() -> Result<(), Box<dyn std::error::Error>> {
        let reports: Vec<Vec<u8>> = process_data("./test_input/02.txt");
        let result: u64 = reports
            .into_iter()
            .map(|report| is_report_safe(&report) as u64)
            .sum();
        assert_eq!(result, 2);
        Ok(())
    }
}
