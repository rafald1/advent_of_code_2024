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

fn bron_kerbosch_algorithm<'a>(
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    cliques: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    for &v in &p.clone().into_iter().collect::<Vec<_>>() {
        let mut next_r = r.clone();
        next_r.insert(v);

        let neighbours: &HashSet<&str> = &graph[&v];
        let mut next_p: HashSet<&str> = p.intersection(neighbours).cloned().collect();
        let mut next_x: HashSet<&str> = x.intersection(neighbours).cloned().collect();

        p.remove(&v);
        x.insert(v);

        bron_kerbosch_algorithm(&mut next_r, &mut next_p, &mut next_x, graph, cliques);
    }
}

fn find_max_clique_and_create_password(cliques: &[HashSet<&str>]) -> String {
    let mut max: Vec<&str> = cliques
        .iter()
        .max_by_key(|&clique| clique.len())
        .expect("Should contain at least one element")
        .iter()
        .cloned()
        .collect();

    max.sort_unstable();
    max.join(",")
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let computer_pairs: Vec<[String; 2]> = process_data("./input/23.txt");
    let connection_map: HashMap<&str, HashSet<&str>> = create_connection_map(&computer_pairs);
    let mut cliques: Vec<HashSet<&str>> = Vec::new();

    bron_kerbosch_algorithm(
        &mut HashSet::new(),
        &mut connection_map.keys().cloned().collect(),
        &mut HashSet::new(),
        &connection_map,
        &mut cliques,
    );

    let result: String = find_max_clique_and_create_password(&cliques);
    Ok(format!("Day 23 LAN Party (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_23_lan_party::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let computer_pairs = process_data("./test_input/23.txt");
        let connection_map = create_connection_map(&computer_pairs);
        let mut cliques = Vec::new();

        bron_kerbosch_algorithm(
            &mut HashSet::new(),
            &mut connection_map.keys().cloned().collect(),
            &mut HashSet::new(),
            &connection_map,
            &mut cliques,
        );

        let result = find_max_clique_and_create_password(&cliques);
        assert_eq!(result, "co,de,ka,ta");
    }
}
