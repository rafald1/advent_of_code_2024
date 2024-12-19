use std::collections::HashMap;

fn process_data(file_path: &str) -> (Vec<String>, Vec<String>) {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");
    let mut lines = file_content.lines();

    let patterns: Vec<String> = lines
        .next()
        .expect("Should contain values separated by commas")
        .split(", ")
        .map(String::from)
        .collect();

    let designs: Vec<String> = lines.skip(1).map(String::from).collect();

    (patterns, designs)
}

fn can_design_be_constructed<'a>(
    design: &'a str,
    patterns: &[String],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(&is_valid) = memo.get(design) {
        return is_valid;
    }

    if design.is_empty() {
        return true;
    }

    let is_valid = patterns.iter().any(|pattern| {
        design.starts_with(pattern)
            && can_design_be_constructed(&design[pattern.len()..], patterns, memo)
    });

    memo.insert(design, is_valid);
    is_valid
}

fn count_valid_designs(designs: &[String], patterns: &[String]) -> usize {
    designs
        .iter()
        .filter(|design| can_design_be_constructed(design, patterns, &mut HashMap::new()))
        .count()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let (patterns, designs): (Vec<String>, Vec<String>) = process_data("./input/19.txt");
    let result: usize = count_valid_designs(&designs, &patterns);
    Ok(format!("Day 19 Linen Layout (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_19_linen_layout::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let (patterns, designs) = process_data("./test_input/19.txt");
        let result = count_valid_designs(&designs, &patterns);
        assert_eq!(result, 6);
    }
}
