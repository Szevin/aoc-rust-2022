pub fn solve_a() {
    let file = std::fs::read_to_string("./input/day1.txt").expect("Unable to read file!");

    let result: u32 = file
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|amount| amount.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap();

    println!("{}", result)
}

pub fn solve_b() {
    let file = std::fs::read_to_string("./input/day1.txt").expect("Unable to read file!");

    let mut result = file
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|amount| amount.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    let sum = result.iter().take(3).sum::<u32>();
    println!("{}", sum);
}
