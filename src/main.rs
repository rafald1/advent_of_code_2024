mod day_01_historian_hysteria;
mod day_02_red_nosed_reports;
mod day_03_mull_it_over;
mod day_04_ceres_search;
mod day_05_print_queue;
mod day_06_guard_gallivant;
mod day_07_bridge_repair;
mod day_08_resonant_collinearity;
mod day_09_disk_fragmenter;
mod day_10_hoof_it;
mod day_11_plutonian_pebbles;
mod day_12_garden_groups;
mod day_13_claw_contraption;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solutions = [
        day_01_historian_hysteria::part_1::solve,
        day_01_historian_hysteria::part_2::solve,
        day_02_red_nosed_reports::part_1::solve,
        day_02_red_nosed_reports::part_2::solve,
        day_03_mull_it_over::part_1::solve,
        day_03_mull_it_over::part_2::solve,
        day_04_ceres_search::part_1::solve,
        day_04_ceres_search::part_2::solve,
        day_05_print_queue::part_1::solve,
        day_05_print_queue::part_2::solve,
        day_06_guard_gallivant::part_1::solve,
        day_06_guard_gallivant::part_2::solve,
        day_07_bridge_repair::part_1::solve,
        day_07_bridge_repair::part_2::solve,
        day_08_resonant_collinearity::part_1::solve,
        day_08_resonant_collinearity::part_2::solve,
        day_09_disk_fragmenter::part_1::solve,
        day_09_disk_fragmenter::part_2::solve,
        day_10_hoof_it::part_1::solve,
        day_10_hoof_it::part_2::solve,
        day_11_plutonian_pebbles::part_1::solve,
        day_11_plutonian_pebbles::part_2::solve,
        day_12_garden_groups::part_1::solve,
        day_12_garden_groups::part_2::solve,
        day_13_claw_contraption::part_1::solve,
        day_13_claw_contraption::part_2::solve,
    ];

    for solution in solutions {
        let start = std::time::Instant::now();
        let result = solution()?;
        let duration = start.elapsed();
        println!(
            "{} Solved in {:.3}ms.",
            result,
            duration.as_secs_f64() * 1000_f64
        );
    }

    Ok(())
}
