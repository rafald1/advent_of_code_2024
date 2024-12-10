fn process_data(file_path: &str) -> Vec<Vec<u8>> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Should be a digit between 0 and 9") as u8)
                .collect()
        })
        .collect()
}

fn get_starting_positions(topographic_map: &[Vec<u8>]) -> Vec<[usize; 2]> {
    topographic_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, &cell)| {
                    if cell == 0 {
                        Some([x, y])
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

fn get_valid_neighbours(map: &[Vec<u8>], [x, y]: [usize; 2], target: u8) -> Vec<[usize; 2]> {
    [
        [x + 1, y],
        [x, y + 1],
        [x.wrapping_sub(1), y],
        [x, y.wrapping_sub(1)],
    ]
    .into_iter()
    .filter_map(|[nx, ny]| {
        map.get(ny)
            .and_then(|row| row.get(nx))
            .filter(|&&c| c == target)
            .map(|_| [nx, ny])
    })
    .collect()
}

fn traverse(topographic_map: &[Vec<u8>], [x, y]: [usize; 2], target: u8) -> u64 {
    if topographic_map[y][x] == 9 {
        return 1;
    }

    get_valid_neighbours(topographic_map, [x, y], target)
        .iter()
        .map(|&[x, y]| traverse(topographic_map, [x, y], target + 1))
        .sum()
}

fn calculate_trailhead_ratings(topographic_map: &[Vec<u8>]) -> u64 {
    get_starting_positions(topographic_map)
        .iter()
        .map(|&[x, y]| traverse(topographic_map, [x, y], 1))
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let topographic_map: Vec<Vec<u8>> = process_data("./input/10.txt");
    let result: u64 = calculate_trailhead_ratings(&topographic_map);
    Ok(format!("Day 10 Hoof It (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_10_hoof_it::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let topographic_map = process_data("./test_input/10.txt");
        let result = calculate_trailhead_ratings(&topographic_map);
        assert_eq!(result, 81);
    }
}
