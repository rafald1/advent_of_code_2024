fn process_data(file_path: &str) -> Vec<u32> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .chars()
        .map(|c| c.to_digit(10).expect("Should be a digit between 0 and 9"))
        .collect()
}

fn unpack_information(disk_map: &[u32]) -> Vec<Option<u32>> {
    let mut unpacked_information: Vec<Option<u32>> = Vec::new();
    let mut file_id: u32 = 0;
    let mut is_file = true;

    for &size in disk_map {
        let item: Option<u32> = is_file.then_some(file_id);
        unpacked_information.extend(std::iter::repeat_n(item, size as usize));
        file_id += is_file as u32;
        is_file = !is_file;
    }

    unpacked_information
}

fn fragment_unpacked_information(unpacked_information: &mut [Option<u32>]) {
    let mut free_space_index: usize = 0;
    let mut occupied_space_index: usize = unpacked_information.len() - 1;

    loop {
        while unpacked_information[free_space_index].is_some() {
            free_space_index += 1;
        }

        while unpacked_information[occupied_space_index].is_none() {
            occupied_space_index -= 1;
        }

        if free_space_index > occupied_space_index {
            break;
        }

        unpacked_information.swap(free_space_index, occupied_space_index);
    }
}

fn calculate_checksum(fragmented_information: &[Option<u32>]) -> u64 {
    fragmented_information
        .iter()
        .enumerate()
        .map(|(index, value)| (index as u32 * value.unwrap_or(0)) as u64)
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let dense_format_disk_map: Vec<u32> = process_data("./input/09.txt");
    let mut unpacked_information: Vec<Option<u32>> = unpack_information(&dense_format_disk_map);
    fragment_unpacked_information(&mut unpacked_information);
    let result: u64 = calculate_checksum(&unpacked_information);
    Ok(format!("Day 9 Disk Fragmenter (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_09_disk_fragmenter::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let dense_format_disk_map = process_data("./test_input/09.txt");
        let mut unpacked_information = unpack_information(&dense_format_disk_map);
        fragment_unpacked_information(&mut unpacked_information);
        let result = calculate_checksum(&unpacked_information);
        assert_eq!(result, 1928);
    }
}
