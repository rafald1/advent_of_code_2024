use std::collections::VecDeque;

#[derive(PartialEq, Copy, Clone)]
enum Object {
    Wall,
    BoxLeftHalf,
    BoxRightHalf,
    Nothing,
    Robot,
}

enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn get_dx_dy(&self) -> (i32, i32) {
        match self {
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
        }
    }
}

fn process_data(file_path: &str) -> (Vec<Vec<Object>>, Vec<Dir>) {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to open file");
    let mut lines = file_content.lines();

    let parse_object = |char| match char {
        '#' => [Object::Wall, Object::Wall],
        'O' => [Object::BoxLeftHalf, Object::BoxRightHalf],
        '@' => [Object::Robot, Object::Nothing],
        '.' => [Object::Nothing, Object::Nothing],
        _ => panic!("Invalid object found: {}", char),
    };

    let parse_direction = |char| match char {
        '>' => Dir::Right,
        'v' => Dir::Down,
        '<' => Dir::Left,
        '^' => Dir::Up,
        _ => panic!("Invalid direction found: {}", char),
    };

    let warehouse_map = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().flat_map(parse_object).collect())
        .collect();

    let robot_movements = lines
        .flat_map(|line| line.chars().map(parse_direction))
        .collect();

    (warehouse_map, robot_movements)
}

fn find_robot_position(warehouse: &[Vec<Object>]) -> Option<(usize, usize)> {
    for (y, row) in warehouse.iter().enumerate() {
        if let Some(x) = row.iter().position(|object| object == &Object::Robot) {
            return Some((x, y));
        }
    }

    None
}

fn get_object(warehouse: &[Vec<Object>], (x, y): (usize, usize)) -> Object {
    *warehouse
        .get(y)
        .and_then(|row| row.get(x))
        .expect("Should always get a valid object")
}

fn swap_objects(warehouse: &mut [Vec<Object>], (x, y): (usize, usize), (nx, ny): (usize, usize)) {
    let tmp = get_object(warehouse, (x, y));
    warehouse[y][x] = get_object(warehouse, (nx, ny));
    warehouse[ny][nx] = tmp;
}

fn move_robot(warehouse: &mut Vec<Vec<Object>>, movements: &[Dir]) {
    for movement in movements {
        let (x, y) = find_robot_position(warehouse).expect("Starting position should be provided");
        let (dx, dy) = movement.get_dx_dy();
        let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);

        match get_object(warehouse, (nx, ny)) {
            Object::Wall => continue,
            Object::BoxLeftHalf | Object::BoxRightHalf => {
                try_to_move_multiple_boxes(&mut *warehouse, (x, y), movement);
            }
            Object::Nothing => swap_objects(warehouse, (x, y), (nx, ny)),
            Object::Robot => unreachable!("It's impossible to find another robot"),
        }
    }
}

fn try_to_move_multiple_boxes(warehouse: &mut [Vec<Object>], (x, y): (usize, usize), dir: &Dir) {
    let mut queue = VecDeque::from([(x, y)]);
    let mut seen: Vec<(usize, usize)> = Vec::from([]);
    let (dx, dy) = dir.get_dx_dy();

    while let Some((x, y)) = queue.pop_front() {
        if !seen.contains(&(x, y)) {
            seen.push((x, y));
            let (x2, y2) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);

            match get_object(warehouse, (x2, y2)) {
                Object::Wall => return,
                Object::BoxLeftHalf => queue.extend([(x2, y2), (x2 + 1, y2)]),
                Object::BoxRightHalf => queue.extend([(x2, y2), (x2 - 1, y2)]),
                Object::Nothing => continue,
                Object::Robot => unreachable!("It's impossible to find another robot"),
            }
        }
    }

    match dir {
        Dir::Right => seen.sort_by(|(x1, _), (x2, _)| x2.cmp(x1)),
        Dir::Down => seen.sort_by(|(_, y1), (_, y2)| y2.cmp(y1)),
        Dir::Left => seen.sort_by(|(x1, _), (x2, _)| x1.cmp(x2)),
        Dir::Up => seen.sort_by(|(_, y1), (_, y2)| y1.cmp(y2)),
    }

    for (x, y) in seen {
        let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
        swap_objects(warehouse, (x, y), (nx, ny));
    }
}

fn sum_gps_coordinate_of_boxes(warehouse: &[Vec<Object>]) -> usize {
    warehouse
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, object)| object == &Object::BoxLeftHalf)
                .map(move |(x, _)| 100 * y + x)
        })
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let (mut warehouse_map, robot_movements) = process_data("./input/15.txt");
    move_robot(&mut warehouse_map, &robot_movements);
    let result: usize = sum_gps_coordinate_of_boxes(&warehouse_map);
    Ok(format!("Day 15 Warehouse Woes (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_15_warehouse_woes::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let (mut warehouse_map, robot_movements) = process_data("./test_input/15.txt");
        move_robot(&mut warehouse_map, &robot_movements);
        let result = sum_gps_coordinate_of_boxes(&warehouse_map);
        assert_eq!(result, 9021);
    }
}
