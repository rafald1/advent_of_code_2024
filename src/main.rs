mod day_01_historian_hysteria;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solutions = [
        day_01_historian_hysteria::part_1::solve,
        day_01_historian_hysteria::part_2::solve,
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
