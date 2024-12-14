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

fn split_guards_into_groups(guard_positions: &mut Vec<(i32, i32)>) -> usize {
    guard_positions.sort_unstable();
    guard_positions.dedup();
    let mut guard_groups: Vec<Vec<(i32, i32)>> = Vec::new();

    while let Some((x, y)) = guard_positions.pop() {
        let mut guard_group: Vec<(i32, i32)> = Vec::from([(x, y)]);
        let mut stack: Vec<(i32, i32)> = Vec::from([(x, y)]);

        while let Some((x, y)) = stack.pop() {
            for (nx, ny) in get_neighbours((x, y)) {
                if let Some(pos) = guard_positions
                    .iter()
                    .position(|&(px, py)| (px, py) == (nx, ny))
                {
                    guard_positions.swap_remove(pos);
                    stack.push((nx, ny));
                    guard_group.push((nx, ny));
                }
            }
        }

        guard_groups.push(guard_group);
    }

    guard_groups.len()
}

fn nth_guard_positions(details_of_guards: &[[i32; 4]], n: i32, h: i32, w: i32) -> Vec<(i32, i32)> {
    details_of_guards
        .iter()
        .map(|[px, py, vx, vy]| (((px + vx * n) % w + w) % w, ((py + vy * n) % h + h) % h))
        .collect()
}

fn print_tree(guard_positions_and_velocities: &[[i32; 4]], n: i32, height: i32, width: i32) {
    let guard_positions: Vec<(i32, i32)> =
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
    let mut min_number_of_groups: usize = usize::MAX;
    let mut tree_iter = 0;

    for n in 1..height * width {
        let mut guard_positions: Vec<(i32, i32)> =
            nth_guard_positions(guard_positions_and_velocities, n, height, width);

        let number_of_groups = split_guards_into_groups(&mut guard_positions);

        if number_of_groups < min_number_of_groups {
            min_number_of_groups = number_of_groups;
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
