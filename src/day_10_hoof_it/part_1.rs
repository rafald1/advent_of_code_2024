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

fn calculate_trailhead_scores(topographic_map: &[Vec<u8>]) -> usize {
    let mut trailhead_scores: usize = 0;
    let starting_positions: Vec<[usize; 2]> = get_starting_positions(topographic_map);

    for starting_position in &starting_positions {
        let mut valid_cells: Vec<[usize; 2]> = Vec::from([*starting_position]);

        for target in 1_u8..=9 {
            let mut next_valid_cells: Vec<[usize; 2]> = valid_cells
                .into_iter()
                .flat_map(|[x, y]| get_valid_neighbours(topographic_map, [x, y], target))
                .collect();

            next_valid_cells.sort_unstable();
            next_valid_cells.dedup();
            valid_cells = next_valid_cells;
        }

        trailhead_scores += valid_cells.len();
    }

    trailhead_scores
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let topographic_map: Vec<Vec<u8>> = process_data("./input/10.txt");
    let result: usize = calculate_trailhead_scores(&topographic_map);
    Ok(format!("Day 10 Hoof It (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_10_hoof_it::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let topographic_map = process_data("./test_input/10.txt");
        let result = calculate_trailhead_scores(&topographic_map);
        assert_eq!(result, 36);
    }
}
