use std::fs;
use common::launcher;

fn main() {
    launcher::launch(part1, part2);
}

fn part1(input_path: std::path::PathBuf) {
    let input = fs::read_to_string(input_path).unwrap();

    let turns = prase_input(input);

    let mut val = 50;
    let mut count = 0;

    for turn in turns {
        val = (val + turn)%100;
        if val == 0 {
            count += 1;
        }
    }

    println!("{}", count);
}

fn part2(_input: std::path::PathBuf) {
    todo!()
}

fn prase_input(input: String) -> Vec<i32> {
    input
        .lines()
        .map(
            |s| {
                let dir = s.chars().next().unwrap();
                let steps: i32 = s[1..].parse().unwrap();

                match dir {
                    'R' => steps,
                    'L' => -steps,
                    _ => panic!("Invalid direction"),
                }
            }
        )
        .collect()
}
