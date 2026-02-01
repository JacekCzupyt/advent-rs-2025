use clap::Parser;

pub fn launch(part1: fn(input: std::path::PathBuf), part2: fn(input: std::path::PathBuf)) {
    let args = Cli::parse();

    match args.part {
        1 => part1(args.input),
        2 => part2(args.input),
        _ => panic!("Invalid part"),
    }
}

#[derive(Parser)]
pub struct Cli {
    part: i32,
    #[arg(default_value = "./input.txt")]
    input: std::path::PathBuf,
}