use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
enum Dir {
    East,
    South,
    West,
    North,
}

impl Dir {
    fn rotate_left(&self) -> Self {
        match self {
            Dir::East => Dir::North,
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
            Dir::North => Dir::East,
        }
    }

    fn get_dx_dy(&self) -> [i32; 2] {
        match self {
            Dir::East => [1, 0],
            Dir::South => [0, 1],
            Dir::West => [-1, 0],
            Dir::North => [0, -1],
        }
    }
}

fn process_data(file_path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_start_and_end(maze: &[Vec<char>]) -> Option<[[i32; 2]; 2]> {
    let mut start: Option<[i32; 2]> = None;
    let mut end: Option<[i32; 2]> = None;

    for (y, row) in maze.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            match char {
                'S' => start = Some([x as i32, y as i32]),
                'E' => end = Some([x as i32, y as i32]),
                _ => {}
            }
        }
    }

    match [start, end] {
        [Some(start), Some(end)] => Some([start, end]),
        _ => None,
    }
}

fn find_tiles_on_best_paths(
    maze: &[Vec<char>],
    start: [i32; 2],
    end: [i32; 2],
    dir: &Dir,
) -> usize {
    let mut p_queue: BinaryHeap<(Reverse<u32>, [i32; 2], Dir)> =
        BinaryHeap::from([(Reverse(0), start, *dir)]);
    let mut backtrack = HashMap::new();
    let mut end_lowest_score: u32 = u32::MAX;
    let mut end_states: HashSet<([i32; 2], Dir)> = HashSet::new();
    let mut lowest_scores: HashMap<([i32; 2], Dir), u32> = HashMap::new();

    while let Some((Reverse(score), [x, y], dir)) = p_queue.pop() {
        if score > *lowest_scores.entry(([x, y], dir)).or_insert(u32::MAX) {
            continue;
        }

        if [x, y] == end {
            if score < end_lowest_score {
                end_lowest_score = score;
                end_states.insert(([x, y], dir));
            }

            continue;
        }

        for (next_dir, next_score) in [
            (dir, score + 1),
            (dir.rotate_left(), score + 1001),
            (dir.rotate_right(), score + 1001),
        ] {
            let [dx, dy] = next_dir.get_dx_dy();
            let [nx, ny] = [x + dx, y + dy];

            if maze[ny as usize][nx as usize] == '#' {
                continue;
            }

            let lowest_score = lowest_scores
                .entry(([nx, ny], next_dir))
                .or_insert(u32::MAX);

            if next_score > *lowest_score {
                continue;
            }

            if next_score < *lowest_score {
                lowest_scores
                    .entry(([nx, ny], next_dir))
                    .and_modify(|value| *value = next_score);
            }

            backtrack
                .entry(([nx, ny], next_dir))
                .or_insert(HashSet::new())
                .insert(([x, y], dir));

            p_queue.push((Reverse(next_score), [nx, ny], next_dir));
        }
    }

    let mut states: Vec<([i32; 2], Dir)> = end_states.clone().into_iter().collect();
    let mut seen: HashSet<([i32; 2], Dir)> = end_states;

    while let Some(current_state) = states.pop() {
        if let Some(previous_states) = backtrack.get(&current_state) {
            for &([x, y], dir) in previous_states {
                if seen.insert(([x, y], dir)) {
                    states.push(([x, y], dir));
                }
            }
        }
    }

    seen.iter()
        .map(|([x, y], _)| [*x, *y])
        .collect::<HashSet<[i32; 2]>>()
        .len()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let maze: Vec<Vec<char>> = process_data("./input/16.txt");
    let [start, end]: [[i32; 2]; 2] =
        find_start_and_end(&maze).expect("Start and end position should be provided");
    let result: usize = find_tiles_on_best_paths(&maze, start, end, &Dir::East);
    Ok(format!("Day 16 Reindeer Maze (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_16_reindeer_maze::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let maze = process_data("./test_input/16_01.txt");
        let [start, end] = find_start_and_end(&maze).unwrap();
        let result = find_tiles_on_best_paths(&maze, start, end, &Dir::East);
        assert_eq!(result, 45);

        let maze = process_data("./test_input/16_02.txt");
        let [start, end] = find_start_and_end(&maze).unwrap();
        let result = find_tiles_on_best_paths(&maze, start, end, &Dir::East);
        assert_eq!(result, 64);
    }
}
