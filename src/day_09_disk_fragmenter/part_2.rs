fn process_data(file_path: &str) -> Vec<u32> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .chars()
        .map(|c| c.to_digit(10).expect("Should be a digit between 0 and 9"))
        .collect()
}

fn unpack_information(disk_map: &[u32]) -> [Vec<(u32, u32)>; 2] {
    let mut files: Vec<(u32, u32)> = Vec::new();
    let mut free_space: Vec<(u32, u32)> = Vec::new();
    let mut next_index: u32 = 0;
    let mut is_file = true;

    for &size in disk_map {
        if is_file {
            files.push((next_index, size));
        } else if size != 0 {
            free_space.push((next_index, size));
        }

        next_index += size;
        is_file = !is_file;
    }

    [files, free_space]
}

fn move_whole_files(files: &mut [(u32, u32)], free_space: &mut [(u32, u32)]) {
    for (file_start, file_length) in files.iter_mut().rev() {
        for (free_space_start, free_space_length) in free_space.iter_mut() {
            if *file_start < *free_space_start {
                break;
            }

            if *file_length > *free_space_length {
                continue;
            }

            *file_start = *free_space_start;

            if *file_length < *free_space_length {
                *free_space_start += *file_length;
                *free_space_length -= *file_length;
            } else {
                *free_space_length = 0;
            }
        }
    }
}

fn calculate_checksum(files: &[(u32, u32)]) -> u64 {
    files
        .iter()
        .enumerate()
        .map(|(index, &(file_start, file_length))| {
            (file_start..(file_start + file_length)).sum::<u32>() as u64 * index as u64
        })
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let dense_format_map: Vec<u32> = process_data("./input/09.txt");
    let [mut files, mut free_space]: [Vec<(u32, u32)>; 2] = unpack_information(&dense_format_map);
    move_whole_files(&mut files, &mut free_space);
    let result: u64 = calculate_checksum(&files);
    Ok(format!("Day 9 Disk Fragmenter (Part 2): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_09_disk_fragmenter::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let dense_format_disk_map = process_data("./test_input/09.txt");
        let [mut files, mut free_space] = unpack_information(&dense_format_disk_map);
        move_whole_files(&mut files, &mut free_space);
        let result = calculate_checksum(&files);
        assert_eq!(result, 2858);
    }
}
