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

fn count_possible_ways_to_complete_design<'a>(
    design: &'a str,
    patterns: &[String],
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&counter) = memo.get(design) {
        return counter;
    }

    if design.is_empty() {
        return 1;
    }

    let counter = patterns
        .iter()
        .filter(|&pattern| design.starts_with(pattern))
        .map(|pattern| {
            count_possible_ways_to_complete_design(&design[pattern.len()..], patterns, memo)
        })
        .sum();

    memo.insert(design, counter);
    counter
}

fn count_possible_ways_to_complete_designs(designs: &[String], patterns: &[String]) -> usize {
    designs
        .iter()
        .map(|design| count_possible_ways_to_complete_design(design, patterns, &mut HashMap::new()))
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let (patterns, designs): (Vec<String>, Vec<String>) = process_data("./input/19.txt");
    let result: usize = count_possible_ways_to_complete_designs(&designs, &patterns);
    Ok(format!("Day 19 Linen Layout (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_19_linen_layout::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let (patterns, designs) = process_data("./test_input/19.txt");
        let result = count_possible_ways_to_complete_designs(&designs, &patterns);
        assert_eq!(result, 16);
    }
}
