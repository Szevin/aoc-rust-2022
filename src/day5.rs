fn default_cargo() -> [Vec<char>; 9] {
    [
        vec!['J', 'H', 'P', 'M', 'S', 'F', 'N', 'V'],
        vec!['S', 'R', 'L', 'M', 'J', 'D', 'Q'],
        vec!['N', 'Q', 'D', 'H', 'C', 'S', 'W', 'B'],
        vec!['R', 'S', 'C', 'L'],
        vec!['M', 'V', 'T', 'P', 'F', 'B'],
        vec!['T', 'R', 'Q', 'N', 'C'],
        vec!['G', 'V', 'R'],
        vec!['C', 'Z', 'S', 'P', 'D', 'L', 'R'],
        vec!['D', 'S', 'J', 'V', 'G', 'P', 'B', 'F'],
        //
        // vec!['Z', 'N'],
        // vec!['M', 'C', 'D'],
        // vec!['P'],
    ]
}

pub fn solve_a() -> String {
    std::fs::read_to_string("src/input/day5.txt")
        .expect("File not found!")
        .lines()
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .map(|s| {
            (
                s.get(1).unwrap().parse::<usize>().unwrap(),
                s.get(3).unwrap().parse::<usize>().unwrap() - 1,
                s.get(5).unwrap().parse::<usize>().unwrap() - 1,
            )
        })
        .fold(default_cargo(), |mut cargo, s| {
            let (mv, from, to) = s;

            let keep_amount = cargo[from].len().saturating_sub(mv);

            let mut cp = cargo[from].split_off(keep_amount);
            cp.reverse();

            cargo[to].append(&mut cp);

            cargo
        })
        .map(|cargo_slot| match cargo_slot.last() {
            Some(item) => item.to_string(),
            None => String::new(),
        })
        .iter()
        .fold(String::new(), |res, item| res + item)
}

pub fn solve_b() -> String {
    std::fs::read_to_string("src/input/day5.txt")
        .expect("File not found!")
        .lines()
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .map(|s| {
            (
                s.get(1).unwrap().parse::<usize>().unwrap(),
                s.get(3).unwrap().parse::<usize>().unwrap() - 1,
                s.get(5).unwrap().parse::<usize>().unwrap() - 1,
            )
        })
        .fold(default_cargo(), |mut cargo, s| {
            let (mv, from, to) = s;

            let keep_amount = cargo[from].len().saturating_sub(mv);

            let mut cp = cargo[from].split_off(keep_amount);

            cargo[to].append(&mut cp);

            cargo
        })
        .map(|cargo_slot| match cargo_slot.last() {
            Some(item) => item.to_string(),
            None => String::new(),
        })
        .iter()
        .fold(String::new(), |res, item| res + item)
}
