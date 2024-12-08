use std::collections::{HashMap, HashSet};

fn process_data(file_path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn group_by_same_frequency(antenna_map: &[Vec<char>]) -> HashMap<char, Vec<(i32, i32)>> {
    let mut hash_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, row) in antenna_map.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            if char != '.' {
                hash_map.entry(char).or_default().push((x as i32, y as i32));
            }
        }
    }

    hash_map
}

fn count_antinodes(map: &HashMap<char, Vec<(i32, i32)>>, height: i32, width: i32) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for antenna_positions in map.values() {
        for (x1, y1) in antenna_positions {
            for (x2, y2) in antenna_positions {
                if (x1, y1) != (x2, y2) {
                    let (dx, dy) = (x1 - x2, y1 - y2);
                    let (nx, ny) = (x1 + dx, y1 + dy);

                    if nx >= 0 && ny >= 0 && nx < width && ny < height {
                        antinodes.insert((nx, ny));
                    }
                }
            }
        }
    }

    antinodes.len()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let antennas: Vec<Vec<char>> = process_data("./input/08.txt");
    let (height, width): (i32, i32) = (antennas.len() as i32, antennas[0].len() as i32);
    let antenna_map: HashMap<char, Vec<(i32, i32)>> = group_by_same_frequency(&antennas);
    let result: usize = count_antinodes(&antenna_map, height, width);
    Ok(format!("Day 8 Resonant Collinearity (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_08_resonant_collinearity::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let antennas = process_data("./test_input/08.txt");
        let (height, width) = (antennas.len() as i32, antennas[0].len() as i32);
        let antenna_map = group_by_same_frequency(&antennas);
        let result = count_antinodes(&antenna_map, height, width);
        assert_eq!(result, 14);
    }
}
