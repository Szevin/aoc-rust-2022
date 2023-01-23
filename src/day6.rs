fn find_distinct_set_by_width(width: usize) -> usize {
    let input = std::fs::read_to_string("src/input/day6.txt")
        .expect("File not found!")
        .chars()
        .collect::<Vec<char>>();

    let mut index = width;
    while index < input.len() {
        let mut set = std::collections::HashSet::new();
        for j in index - (width - 1)..=index {
            set.insert(input[j]);
        }

        index += 1;
        if set.len() == width {
            break;
        }
    }
    index
}

pub fn solve_a() -> String {
    find_distinct_set_by_width(4).to_string()
}

pub fn solve_b() -> String {
    find_distinct_set_by_width(14).to_string()
}
