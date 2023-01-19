pub fn solve_a() -> u32 {
    std::fs::read_to_string("./input/day1.txt")
        .expect("Unable to read file!")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|amount| amount.parse::<u32>())
                .map(|s| s.unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn solve_b() -> u32 {
    let mut res: Vec<u32> = std::fs::read_to_string("./input/day1.txt")
        .expect("Unable to read file!")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|amount| amount.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    res.sort_by(|a, b| b.cmp(a));

    res.iter().take(3).sum::<u32>()
}
