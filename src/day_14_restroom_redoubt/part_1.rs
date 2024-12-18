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

fn calculate_safety_factor(guard_positions: &[[i32; 2]], height: i32, width: i32) -> u32 {
    let mut guard_count: [u32; 4] = [0; 4];

    for &[x, y] in guard_positions {
        if x > (width - 1) / 2 && y < (height - 1) / 2 {
            guard_count[0] += 1;
        } else if x < (width - 1) / 2 && y < (height - 1) / 2 {
            guard_count[1] += 1;
        } else if x < (width - 1) / 2 && y > (height - 1) / 2 {
            guard_count[2] += 1;
        } else if x > (width - 1) / 2 && y > (height - 1) / 2 {
            guard_count[3] += 1;
        }
    }

    guard_count.iter().product()
}

fn simulate_guard_positions_after_n_seconds(
    guard_positions_and_velocities: &[[i32; 4]],
    n: i32,
    height: i32,
    width: i32,
) -> Vec<[i32; 2]> {
    guard_positions_and_velocities
        .iter()
        .map(|[px, py, vx, vy]| {
            [
                (px + vx * n).rem_euclid(width),
                (py + vy * n).rem_euclid(height),
            ]
        })
        .collect()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let guard_positions_and_velocities: Vec<[i32; 4]> = process_data("./input/14.txt");
    let height: i32 = 103;
    let width: i32 = 101;
    let guard_positions: Vec<[i32; 2]> = simulate_guard_positions_after_n_seconds(
        &guard_positions_and_velocities,
        100,
        height,
        width,
    );
    let result: u32 = calculate_safety_factor(&guard_positions, height, width);
    Ok(format!("Day 14 Restroom Redoubt (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_14_restroom_redoubt::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let guard_positions_and_velocities = process_data("./test_input/14.txt");
        let height = 11;
        let width = 7;
        let guard_positions = simulate_guard_positions_after_n_seconds(
            &guard_positions_and_velocities,
            100,
            height,
            width,
        );
        let result = calculate_safety_factor(&guard_positions, height, width);
        assert_eq!(result, 12);
    }
}
