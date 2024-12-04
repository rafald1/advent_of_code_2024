fn process_data(file_path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_neighbours(x: usize, y: usize, height: usize, width: usize) -> Option<[[usize; 2]; 4]> {
    let neighbours = [
        [x + 1, y.wrapping_sub(1)],
        [x + 1, y + 1],
        [x.wrapping_sub(1), y + 1],
        [x.wrapping_sub(1), y.wrapping_sub(1)],
    ];

    neighbours
        .iter()
        .all(|&[nx, ny]| nx < width && ny < height)
        .then_some(neighbours)
}

fn find_x_mas(letters: &[Vec<char>]) -> u64 {
    let check_pair = |[a_x, a_y]: [usize; 2], [b_x, b_y]: [usize; 2]| {
        (letters[a_y][a_x] == 'M' && letters[b_y][b_x] == 'S')
            || (letters[a_y][a_x] == 'S' && letters[b_y][b_x] == 'M')
    };

    let (height, width) = (letters.len(), letters[0].len());
    let mut result = 0;

    for (y, line) in letters.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char == 'A' {
                if let Some(corner_neighbours) = get_neighbours(x, y, height, width) {
                    result += (check_pair(corner_neighbours[0], corner_neighbours[2])
                        && check_pair(corner_neighbours[1], corner_neighbours[3]))
                        as u64;
                }
            }
        }
    }

    result
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let letters: Vec<Vec<char>> = process_data("./input/04.txt");
    let result: u64 = find_x_mas(&letters);
    Ok(format!("Day 4 Ceres Search (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_04_ceres_search::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let letters: Vec<Vec<char>> = process_data("./test_input/04.txt");
        let result: u64 = find_x_mas(&letters);
        assert_eq!(result, 9);
    }
}
