use std::{rc::Rc, vec};

const INPUT: &str = include_str!(".\\input\\day7.txt");

struct File {
    name: String,
    size: u32,
}

struct Directory<'a> {
    name: String,
    parent: Option<&'a Directory<'a>>,
    directories: Vec<&'a Directory<'a>>,
    files: Vec<&'a File>,
}

pub fn solve_a() -> String {
    let commands = INPUT
        .trim()
        .split("$")
        .map(|s| {
            s.split('\n')
                .map(|s| s.trim())
                .filter(|x| *x != "")
                .collect::<Vec<&str>>()
        })
        .filter(|x| x.len() > 0)
        .collect::<Vec<Vec<&str>>>();

    let os: &mut Directory = &mut Directory {
        name: "root".to_string(),
        parent: None,
        directories: vec![],
        files: vec![],
    };

    for command in commands {
        match command.get(0).unwrap().split(" ").next().unwrap() {
            "cd" => {
              os =
            },
            "ls" => {
                println!("ls");
            }
            _ => {
                println!("unknown command");
            }
        }
    }

    "".to_string()
}

pub fn solve_b() -> String {
    todo!()
}
