fn process_data(file_path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_start(maze: &[Vec<char>]) -> Option<(u32, u32)> {
    maze.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .position(|&char| char == 'S')
            .map(|x| (x as u32, y as u32))
    })
}

fn get_one_valid_neighbour(
    maze: &[Vec<char>],
    (x, y): (u32, u32),
    (px, py): (u32, u32),
) -> Option<(u32, u32)> {
    [
        (x + 1, y),
        (x, y + 1),
        (x.wrapping_sub(1), y),
        (x, y.wrapping_sub(1)),
    ]
    .into_iter()
    .find(|&(nx, ny)| (nx, ny) != (px, py) && maze[ny as usize][nx as usize] != '#')
}

fn find_path(maze: &[Vec<char>]) -> Vec<(u32, u32)> {
    let (mut x, mut y): (u32, u32) = find_start(maze).expect("Start position should be provided");
    let (mut px, mut py): (u32, u32) = (x, y);
    let mut path: Vec<(u32, u32)> = Vec::from([(x, y)]);

    while let Some((nx, ny)) = get_one_valid_neighbour(maze, (x, y), (px, py)) {
        path.push((nx, ny));
        (px, py) = (x, y);
        (x, y) = (nx, ny);
    }

    path
}

fn count_shortcuts_that_save_at_least_n_picoseconds(path: &[(u32, u32)], n: u32) -> u32 {
    let mut shortcut_count: u32 = 0;

    for (i, (x1, y1)) in path.iter().enumerate() {
        for (j, &(x2, y2)) in path.iter().enumerate().skip(i + n as usize) {
            let diff = x1.abs_diff(x2) + y1.abs_diff(y2);

            if diff <= 20 && (j - i) as u32 - diff >= n {
                shortcut_count += 1;
            }
        }
    }

    shortcut_count
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let maze: Vec<Vec<char>> = process_data("./input/20.txt");
    let path: Vec<(u32, u32)> = find_path(&maze);
    let result: u32 = count_shortcuts_that_save_at_least_n_picoseconds(&path, 100);
    Ok(format!("Day 20 Race Condition (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_20_race_condition::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let maze = process_data("./test_input/20.txt");
        let path = find_path(&maze);
        let result = count_shortcuts_that_save_at_least_n_picoseconds(&path, 50);
        assert_eq!(result, 285);
    }
}
