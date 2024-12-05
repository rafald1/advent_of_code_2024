use std::collections::HashMap;

fn process_data(file_path: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");
    let mut lines = file_content.lines();

    let page_ordering_rules: Vec<(u8, u8)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once("|").expect("Should contain '|' character"))
        .map(|(num_1, num_2)| {
            (
                num_1.parse::<u8>().expect("Should be a valid u8 number"),
                num_2.parse::<u8>().expect("Should be a valid u8 number"),
            )
        })
        .collect();

    let updates: Vec<Vec<u8>> = lines
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<u8>().expect("Should be a valid u8 number"))
                .collect()
        })
        .collect();

    (page_ordering_rules, updates)
}

fn build_page_ordering_map(page_ordering_rules: &[(u8, u8)]) -> HashMap<u8, Vec<u8>> {
    page_ordering_rules
        .iter()
        .fold(HashMap::new(), |mut map, &(key, value)| {
            map.entry(key).or_default().push(value);
            map.entry(value).or_default();
            map
        })
}

fn is_in_right_order(pages_to_produce: &[u8], page_ordering_map: &HashMap<u8, Vec<u8>>) -> bool {
    for pair in pages_to_produce.windows(2) {
        if !page_ordering_map
            .get(&pair[0])
            .expect("Should be a valid key")
            .contains(&pair[1])
        {
            return false;
        }
    }

    true
}

fn sum_valid_updates_middle_page(updates: &[Vec<u8>], ordering_map: &HashMap<u8, Vec<u8>>) -> u32 {
    updates
        .iter()
        .filter(|pages_to_produce| is_in_right_order(pages_to_produce, ordering_map))
        .map(|right_order_pages| right_order_pages[right_order_pages.len() / 2] as u32)
        .sum()
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let (ordering_rules, updates): (Vec<(u8, u8)>, Vec<Vec<u8>>) = process_data("./input/05.txt");
    let page_ordering_map: HashMap<u8, Vec<u8>> = build_page_ordering_map(&ordering_rules);
    let result: u32 = sum_valid_updates_middle_page(&updates, &page_ordering_map);
    Ok(format!("Day 5 Print Queue (Part 1): {}.", result))
}

#[cfg(test)]
mod tests {
    use crate::day_05_print_queue::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let (page_ordering_rules, updates) = process_data("./test_input/05.txt");
        let page_ordering_map = build_page_ordering_map(&page_ordering_rules);
        let result = sum_valid_updates_middle_page(&updates, &page_ordering_map);
        assert_eq!(result, 143);
    }
}
