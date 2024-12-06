use std::collections::HashSet;

fn process_data(file_path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_starting_position(map: &[Vec<char>]) -> Option<(i32, i32)> {
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '^') {
            return Some((x as i32, y as i32));
        }
    }

    None
}

fn simulate_guard_movement(map: &[Vec<char>], (x, y): (i32, i32)) -> usize {
    let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
    let (mut dx, mut dy): &(i32, i32) = directions.next().expect("Should produce (dx, dy)");
    let mut unique_steps: HashSet<(i32, i32)> = HashSet::from([(x, y)]);
    let (mut x, mut y): (i32, i32) = (x, y);

    loop {
        let (nx, ny) = (x + dx, y + dy);

        let cell = match map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
            Some(&c) => c,
            None => return unique_steps.len(),
        };

        match cell {
            '#' => (dx, dy) = *directions.next().expect("Should produce (dx, dy)"),
            '.' | '^' => {
                unique_steps.insert((nx, ny));
                (x, y) = (nx, ny);
            }
            c => panic!("Invalid symbol on the map: '{}'", c),
        }
    }
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let map: Vec<Vec<char>> = process_data("./input/06.txt");
    let start: (i32, i32) = find_starting_position(&map).ok_or("Starting position not found")?;
    let result: usize = simulate_guard_movement(&map, start);
    Ok(format!("Day 6 Guard Gallivant (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_06_guard_gallivant::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let map = process_data("./test_input/06.txt");
        let start = find_starting_position(&map).unwrap();
        let result = simulate_guard_movement(&map, start);
        assert_eq!(result, 41);
    }
}
