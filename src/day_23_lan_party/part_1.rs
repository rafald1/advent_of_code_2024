use std::collections::{HashMap, HashSet};

fn process_data(file_path: &str) -> Vec<[String; 2]> {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");

    file_content
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(a, b)| [a.to_owned(), b.to_owned()])
        .collect()
}

fn create_connection_map(computer_pairs: &[[String; 2]]) -> HashMap<&str, HashSet<&str>> {
    let mut connection_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for [computer1, computer2] in computer_pairs {
        connection_map
            .entry(computer1)
            .or_default()
            .insert(computer2);
        connection_map
            .entry(computer2)
            .or_default()
            .insert(computer1);
    }

    connection_map
}

fn create_three_computer_sets<'a>(
    connection_map: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<Vec<&'a str>> {
    let mut sets_of_three_computers: HashSet<Vec<&str>> = HashSet::new();

    for computer1 in connection_map.keys() {
        for computer2 in &connection_map[computer1] {
            for computer3 in &connection_map[computer2] {
                if computer1 != computer3 && connection_map[computer3].contains(computer1) {
                    let mut three_computers: Vec<&str> = vec![computer1, computer2, computer3];
                    three_computers.sort();
                    sets_of_three_computers.insert(three_computers);
                }
            }
        }
    }

    sets_of_three_computers
}

fn count_sets_with_computer_name_t_(three_computer_sets: &HashSet<Vec<&str>>) -> usize {
    three_computer_sets
        .iter()
        .filter(|computers| computers.iter().any(|computer| computer.starts_with("t")))
        .count()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let computer_pairs: Vec<[String; 2]> = process_data("./input/23.txt");
    let connection_map: HashMap<&str, HashSet<&str>> = create_connection_map(&computer_pairs);
    let three_computer_sets: HashSet<Vec<&str>> = create_three_computer_sets(&connection_map);
    let result: usize = count_sets_with_computer_name_t_(&three_computer_sets);
    Ok(format!("Day 23 LAN Party (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_23_lan_party::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let computer_pairs = process_data("./test_input/23.txt");
        let connection_map = create_connection_map(&computer_pairs);
        let three_computer_sets = create_three_computer_sets(&connection_map);
        let result = count_sets_with_computer_name_t_(&three_computer_sets);
        assert_eq!(result, 7);
    }
}
