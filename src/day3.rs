const INPUT: &str = include_str!(".\\input\\day3.txt");

pub fn solve_a() -> String {
    INPUT
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
        .sum::<u32>()
        .to_string()
}

pub fn solve_b() -> String {
    INPUT
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|c| {
            let (first, second, third) = (
                c[0].chars().collect::<Vec<char>>(),
                c[1].chars().collect::<Vec<char>>(),
                c[2].chars().collect::<Vec<char>>(),
            );

            first
                .iter()
                .find(|x| second.contains(x) && third.contains(x))
                .unwrap()
                .clone()
        })
        .map(|c| {
            if c.is_lowercase() {
                (c as u8) - (b'a' - 1)
            } else {
                (c as u8) + 27 - (b'A')
            }
        })
        .map(|x| x as u32)
        .sum::<u32>()
        .to_string()
}
