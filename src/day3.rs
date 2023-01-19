pub fn solve_a() -> u32 {
    std::fs::read_to_string("input/day3.txt")
        .expect("File not found!")
        .lines()
        .flat_map(|s| {
            let (first, second) = s.split_at(s.len() / 2);

            second
                .chars()
                .into_iter()
                .filter(move |c| first.chars().collect::<Vec<char>>().contains(c))
                .collect::<std::collections::HashSet<_>>()
        })
        .map(|c| {
            if c.is_lowercase() {
                (c as u8) - (b'a' - 1)
            } else {
                (c as u8) + 27 - (b'A')
            }
        })
        .map(|x| x as u32)
        .sum()
}

pub fn solve_b() -> u32 {
    todo!()
}