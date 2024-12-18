use std::collections::{HashSet, VecDeque};

fn process_data(file_path: &str) -> Vec<[u8; 2]> {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");

    file_content
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().expect("Should be a valid u8 number"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Should contain two u8 values")
        })
        .collect()
}

fn build_memory_maze<const N: usize>(positions: &[[u8; 2]], n: usize) -> [[bool; N]; N] {
    let mut maze: [[bool; N]; N] = [[false; N]; N];

    for &[x, y] in positions.iter().take(n) {
        maze[y as usize][x as usize] = true;
    }

    maze
}

fn get_valid_neighbours<const N: usize>(
    maze: &[[bool; N]; N],
    [x, y]: [usize; 2],
) -> Vec<[usize; 2]> {
    [
        [x + 1, y],
        [x, y + 1],
        [x.wrapping_sub(1), y],
        [x, y.wrapping_sub(1)],
    ]
    .into_iter()
    .filter_map(|[nx, ny]| {
        maze.get(ny)
            .and_then(|row| row.get(nx))
            .filter(|&&is_obstacle| !is_obstacle)
            .map(|_| [nx, ny])
    })
    .collect()
}

fn find_shortest_path<const N: usize>(
    maze: &[[bool; N]; N],
    start: [usize; 2],
    end: [usize; 2],
) -> u32 {
    let mut queue: VecDeque<([usize; 2], u32)> = VecDeque::from([(start, 0)]);
    let mut visited: HashSet<[usize; 2]> = HashSet::from([start]);

    while let Some(([x, y], step_count)) = queue.pop_front() {
        if [x, y] == end {
            return step_count;
        }

        for [nx, ny] in get_valid_neighbours(maze, [x, y]) {
            if visited.insert([nx, ny]) {
                queue.push_back(([nx, ny], step_count + 1));
            }
        }
    }

    0
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let positions: Vec<[u8; 2]> = process_data("./input/18.txt");
    let maze: [[bool; 71]; 71] = build_memory_maze::<71>(&positions, 1024);
    let result: u32 = find_shortest_path(&maze, [0, 0], [70, 70]);
    Ok(format!("Day 18 RAM Run (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_18_ram_run::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let positions = process_data("./test_input/18.txt");
        let maze = build_memory_maze::<7>(&positions, 12);
        let result = find_shortest_path(&maze, [0, 0], [6, 6]);
        assert_eq!(result, 22);
    }
}
