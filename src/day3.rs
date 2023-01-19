pub fn solve_a() -> u32 {
    std::fs::read_to_string("input/day3.txt")
        .expect("File not found!")
        .lines()
        .flat_map(|s| {
            let (first, second) = s.split_at(s.len() / 2);
            let mut res: Vec<char> = std::vec![];

            for substr in second.split("") {
                if first.contains(substr) && substr != "" {
                    for c in substr.chars() {
                        if !res.contains(&c) {
                            res.push(c)
                        }
                    }
                }
            }

            return res;
        })
        .map(|i| (i as u32 - 48))
        .sum()
}

pub fn solve_b() -> u32 {
    todo!()
}
