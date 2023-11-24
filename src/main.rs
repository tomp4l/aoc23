use std::{
    collections::HashMap,
    fmt::Debug,
    fs,
    io::{BufRead, BufReader},
    process::exit,
};

use structopt::StructOpt;

mod days;

use day::Day;
use days::*;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "day")]
    day: i32,
}

fn default_error_handler<E: Debug, R>(error: E) -> R {
    println!("{:#?}", error);
    exit(1);
}

fn main() {
    let opt = Opt::from_args();
    let mut programs: HashMap<i32, Box<dyn Day>> = HashMap::new();
    programs.insert(1, Box::new(day1::Day1));

    let program = programs
        .get(&opt.day)
        .unwrap_or_else(|| default_error_handler(format!("Undefined day: {}", opt.day).as_str()));
    let file_contents: Vec<String> = fs::File::open(format!("input/day{}.txt", opt.day))
        .and_then(|file| BufReader::new(file).lines().collect())
        .unwrap_or_else(default_error_handler);
    program
        .run(file_contents)
        .unwrap_or_else(default_error_handler)
}
