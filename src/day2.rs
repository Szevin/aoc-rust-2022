use std::fs::read_to_string;

pub fn solve_a() -> String {
    read_to_string("src/input/day2.txt")
        .expect("File not found!")
        .split("\n")
        .map(|s| match s {
            "A X" => 1 + 3,
            "B X" => 1 + 0,
            "C X" => 1 + 6,

            "A Y" => 2 + 6,
            "B Y" => 2 + 3,
            "C Y" => 2 + 0,

            "A Z" => 3 + 0,
            "B Z" => 3 + 6,
            "C Z" => 3 + 3,
            _ => 0,
        })
        .sum::<u32>()
        .to_string()
}

pub fn solve_b() -> String {
    read_to_string("src/input/day2.txt")
        .expect("File not found!")
        .split("\n")
        .map(|s| match s {
            "A X" => 3 + 0,
            "B X" => 1 + 0,
            "C X" => 2 + 0,

            "A Y" => 1 + 3,
            "B Y" => 2 + 3,
            "C Y" => 3 + 3,

            "A Z" => 2 + 6,
            "B Z" => 3 + 6,
            "C Z" => 1 + 6,
            _ => 0,
        })
        .sum::<u32>()
        .to_string()
}
