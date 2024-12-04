fn process_data(file_path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn is_in_grid(x: i64, y: i64, height: usize, width: usize) -> bool {
    x >= 0 && y >= 0 && x < width as i64 && y < height as i64
}

fn get_neighbour_deltas() -> Vec<(i64, i64)> {
    (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| (x, y)))
        .filter(|&(x, y)| x != 0 || y != 0)
        .collect()
}

fn find_xmas(letters: &[Vec<char>]) -> u64 {
    let mut result: u64 = 0;
    let (height, width) = (letters.len(), letters[0].len());

    for (y, line) in letters.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char == 'X' {
                for (dx, dy) in get_neighbour_deltas() {
                    let (mut nx, mut ny) = (x as i64, y as i64);

                    result += ['M', 'A', 'S'].iter().all(|&target| {
                        (nx, ny) = (nx + dx, ny + dy);

                        is_in_grid(nx, ny, height, width)
                            && letters[ny as usize][nx as usize] == target
                    }) as u64;
                }
            }
        }
    }

    result
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let letters: Vec<Vec<char>> = process_data("./input/04.txt");
    let result: u64 = find_xmas(&letters);
    Ok(format!("Day 4 Ceres Search (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_04_ceres_search::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let letters = process_data("./test_input/04.txt");
        let result = find_xmas(&letters);
        assert_eq!(result, 18);
    }
}
