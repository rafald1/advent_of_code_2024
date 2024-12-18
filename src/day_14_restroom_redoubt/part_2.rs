use std::collections::HashSet;

fn process_data(file_path: &str) -> Vec<[i32; 4]> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .map(|line| {
            line.replace("p=", "")
                .replace("v=", "")
                .replace(" ", ",")
                .split(',')
                .map(|num| num.parse::<i32>().expect("Should be a valid i32 number"))
                .collect::<Vec<i32>>()
                .try_into()
                .expect("Should produce [i32; 4]")
        })
        .collect()
}

fn get_neighbours((x, y): (i32, i32)) -> [(i32, i32); 4] {
    [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
}

fn calculate_closeness_factor(guard_positions: &HashSet<(i32, i32)>) -> usize {
    guard_positions
        .iter()
        .flat_map(|&(x, y)| get_neighbours((x, y)))
        .filter(|&(nx, ny)| guard_positions.contains(&(nx, ny)))
        .count()
}

fn nth_guard_positions(
    guard_positions_and_velocities: &[[i32; 4]],
    n: i32,
    height: i32,
    width: i32,
) -> HashSet<(i32, i32)> {
    guard_positions_and_velocities
        .iter()
        .map(|[px, py, vx, vy]| {
            (
                (px + vx * n).rem_euclid(width),
                (py + vy * n).rem_euclid(height),
            )
        })
        .collect()
}

fn print_tree(guard_positions_and_velocities: &[[i32; 4]], n: i32, height: i32, width: i32) {
    let guard_positions: HashSet<(i32, i32)> =
        nth_guard_positions(guard_positions_and_velocities, n, height, width);
    let mut grid = [[' '; 101]; 103];

    for (x, y) in guard_positions {
        grid[y as usize][x as usize] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn find_tree(guard_positions_and_velocities: &[[i32; 4]], height: i32, width: i32) -> i32 {
    let mut max_closeness_factor: usize = 0;
    let mut tree_iter = 0;

    for n in 1..height * width {
        let guard_positions: HashSet<(i32, i32)> =
            nth_guard_positions(guard_positions_and_velocities, n, height, width);

        let closeness_factor = calculate_closeness_factor(&guard_positions);

        if closeness_factor > max_closeness_factor {
            max_closeness_factor = closeness_factor;
            tree_iter = n;
        }
    }

    print_tree(guard_positions_and_velocities, tree_iter, height, width);
    tree_iter
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let guard_positions_and_velocities: Vec<[i32; 4]> = process_data("./input/14.txt");
    let result = find_tree(&guard_positions_and_velocities, 103, 101);
    Ok(format!("Day 14 Restroom Redoubt (Part 2): {}.", result))
}
