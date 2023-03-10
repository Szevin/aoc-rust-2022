mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} <day> <part>", args[0]);
    }

    println!(
        "{}",
        match args[1].as_str() {
            "1" => match args[2].as_str() {
                "a" => day1::solve_a(),
                "b" => day1::solve_b(),
                _ => panic!("Unknown part {}", args[2]),
            },
            "2" => match args[2].as_str() {
                "a" => day2::solve_a(),
                "b" => day2::solve_b(),
                _ => panic!("Unknown part {}", args[2]),
            },
            "3" => match args[2].as_str() {
                "a" => day3::solve_a(),
                "b" => day3::solve_b(),
                _ => panic!("Unknown part {}", args[2]),
            },
            "4" => match args[2].as_str() {
                "a" => day4::solve_a(),
                "b" => day4::solve_b(),
                _ => panic!("Unknown part {}", args[2]),
            },
            "5" => match args[2].as_str() {
                "a" => day5::solve_a(),
                "b" => day5::solve_b(),
                _ => panic!("Unknown part {}", args[2]),
            },
            "6" => match args[2].as_str() {
                "a" => day6::solve_a(),
                "b" => day6::solve_b(),
                _ => panic!("Unknown part {}", args[2]),
            },
            "7" => match args[2].as_str() {
                "a" => day7::solve_a(),
                "b" => day7::solve_b(),
                _ => panic!("Unknown part {}", args[2]),
            },
            _ => panic!("Unknown day {}", args[1]),
        }
    )
}
