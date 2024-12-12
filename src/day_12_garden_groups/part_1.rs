use std::collections::HashMap;

fn process_data(file_path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn group_by_same_type(garden_plots: &[Vec<char>]) -> Vec<Vec<(u32, u32)>> {
    let mut same_type_plots: HashMap<char, Vec<(u32, u32)>> = HashMap::new();

    for (y, row) in garden_plots.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            same_type_plots
                .entry(char)
                .or_default()
                .push((x as u32, y as u32));
        }
    }

    same_type_plots.into_values().collect()
}

fn get_neighbours((x, y): (u32, u32)) -> [(u32, u32); 4] {
    [
        (x + 1, y),
        (x, y + 1),
        (x.wrapping_sub(1), y),
        (x, y.wrapping_sub(1)),
    ]
}

fn split_into_separate_plots(same_type_plots: &mut Vec<(u32, u32)>) -> Vec<Vec<(u32, u32)>> {
    let mut separate_plots: Vec<Vec<(u32, u32)>> = Vec::new();

    while let Some((x, y)) = same_type_plots.pop() {
        let mut new_plot_group: Vec<(u32, u32)> = Vec::from([(x, y)]);
        let mut stack: Vec<(u32, u32)> = Vec::from([(x, y)]);

        while let Some((x, y)) = stack.pop() {
            for (nx, ny) in get_neighbours((x, y)) {
                if let Some(pos) = same_type_plots
                    .iter()
                    .position(|&(px, py)| (px, py) == (nx, ny))
                {
                    same_type_plots.swap_remove(pos);
                    stack.push((nx, ny));
                    new_plot_group.push((nx, ny));
                }
            }
        }

        separate_plots.push(new_plot_group);
    }

    separate_plots
}

fn calculate_plot_perimeter(plot: &[(u32, u32)]) -> u32 {
    plot.iter()
        .map(|&(x, y)| {
            get_neighbours((x, y))
                .into_iter()
                .filter(|&(nx, ny)| !plot.contains(&(nx, ny)))
                .count() as u32
        })
        .sum()
}

fn calculate_price_of_fencing(garden_plots: &[Vec<char>]) -> u32 {
    let mut same_type_plots: Vec<Vec<(u32, u32)>> = group_by_same_type(garden_plots);

    let separate_plots: Vec<Vec<(u32, u32)>> = same_type_plots
        .iter_mut()
        .flat_map(split_into_separate_plots)
        .collect();

    separate_plots
        .iter()
        .map(|plot| plot.len() as u32 * calculate_plot_perimeter(plot))
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let garden_plots: Vec<Vec<char>> = process_data("./input/12.txt");
    let result: u32 = calculate_price_of_fencing(&garden_plots);
    Ok(format!("Day 12 Garden Groups (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_12_garden_groups::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let garden_plots = process_data("./test_input/12.txt");
        let result = calculate_price_of_fencing(&garden_plots);
        assert_eq!(result, 1930);
    }
}
