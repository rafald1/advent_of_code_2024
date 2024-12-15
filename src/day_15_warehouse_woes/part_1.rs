#[derive(PartialEq)]
enum Object {
    Wall,
    Box,
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
        '#' => Object::Wall,
        'O' => Object::Box,
        '@' => Object::Robot,
        '.' => Object::Nothing,
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
        .map(|line| line.chars().map(parse_object).collect())
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

fn try_to_move_box(
    warehouse: &mut [Vec<Object>],
    (nx, ny): (usize, usize),
    (dx, dy): (i32, i32),
) -> bool {
    let (mut nx, mut ny) = (nx, ny);

    loop {
        (nx, ny) = ((nx as i32 + dx) as usize, (ny as i32 + dy) as usize);
        let object = warehouse
            .get(ny)
            .and_then(|row| row.get(nx))
            .expect("Should always get a valid object");

        if object == &Object::Nothing {
            warehouse[ny][nx] = Object::Box;
            return true;
        } else if object == &Object::Wall {
            return false;
        }
    }
}

fn move_robot(warehouse: &mut [Vec<Object>], movements: &[Dir]) {
    let (mut x, mut y): (usize, usize) =
        find_robot_position(warehouse).expect("Starting position should be provided");

    for movement in movements {
        let (dx, dy): (i32, i32) = movement.get_dx_dy();
        let (nx, ny): (usize, usize) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);

        let object = warehouse
            .get(ny)
            .and_then(|row| row.get(nx))
            .expect("Should always get a valid object");

        match object {
            Object::Wall => continue,
            Object::Box => {
                if try_to_move_box(warehouse, (nx, ny), (dx, dy)) {
                    warehouse[y][x] = Object::Nothing;
                    warehouse[ny][nx] = Object::Robot;
                    (x, y) = (nx, ny);
                }
            }
            Object::Nothing => {
                warehouse[y][x] = Object::Nothing;
                warehouse[ny][nx] = Object::Robot;
                (x, y) = (nx, ny);
            }
            Object::Robot => unreachable!("It's impossible to find another robot"),
        }
    }
}

fn sum_gps_coordinate_of_boxes(warehouse: &[Vec<Object>]) -> usize {
    warehouse
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, object)| object == &Object::Box)
                .map(move |(x, _)| 100 * y + x)
        })
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let (mut warehouse_map, robot_movements) = process_data("./input/15.txt");
    move_robot(&mut warehouse_map, &robot_movements);
    let result: usize = sum_gps_coordinate_of_boxes(&warehouse_map);
    Ok(format!("Day 15 Warehouse Woes (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_15_warehouse_woes::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let (mut warehouse_map, robot_movements) = process_data("./test_input/15.txt");
        move_robot(&mut warehouse_map, &robot_movements);
        let result = sum_gps_coordinate_of_boxes(&warehouse_map);
        assert_eq!(result, 10092);
    }
}
