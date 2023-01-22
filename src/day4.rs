pub fn solve_a() -> String {
    std::fs::read_to_string("input/day4.txt")
        .expect("File not found!")
        .lines()
        .map(|s| s.split_once(",").unwrap())
        .map(|(f, s)| (f.split_once("-").unwrap(), s.split_once("-").unwrap()))
        .map(|((f_s, f_e), (s_s, s_e))| {
            (
                (f_s.parse::<u32>().unwrap(), f_e.parse::<u32>().unwrap()),
                (s_s.parse::<u32>().unwrap(), s_e.parse::<u32>().unwrap()),
            )
        })
        .fold(0, |sum, x| {
            let ((first_start, first_end), (second_start, second_end)) = x;
            if first_start >= second_start && first_end <= second_end {
                sum + 1
            } else if first_start <= second_start && first_end >= second_end {
                sum + 1
            } else {
                sum
            }
        })
        .to_string()
}

pub fn solve_b() -> String {
    std::fs::read_to_string("input/day4.txt")
        .expect("File not found!")
        .lines()
        .map(|s| s.split_once(",").unwrap())
        .map(|(f, s)| (f.split_once("-").unwrap(), s.split_once("-").unwrap()))
        .map(|((f_s, f_e), (s_s, s_e))| {
            (
                (f_s.parse::<u32>().unwrap(), f_e.parse::<u32>().unwrap()),
                (s_s.parse::<u32>().unwrap(), s_e.parse::<u32>().unwrap()),
            )
        })
        .fold(0, |sum, x| {
            let ((first_start, first_end), (second_start, second_end)) = x;
            if (first_start >= second_start && first_start <= second_end)
                || (first_end <= second_end && first_end >= second_start)
            {
                sum + 1
            } else if (second_start >= first_start && second_start <= first_end)
                || (second_end <= first_end && second_end >= first_start)
            {
                sum + 1
            } else {
                sum
            }
        })
        .to_string()
}
