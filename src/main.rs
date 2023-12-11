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
    programs.insert(1, Box::new(day1::Instance));
    programs.insert(2, Box::new(day2::Instance));
    programs.insert(3, Box::new(day3::Instance));
    programs.insert(4, Box::new(day4::Instance));
    programs.insert(5, Box::new(day5::Instance));
    programs.insert(6, Box::new(day6::Instance));
    programs.insert(7, Box::new(day7::Instance));
    programs.insert(8, Box::new(day8::Instance));
    programs.insert(9, Box::new(day9::Instance));
    programs.insert(10, Box::new(day10::Instance));
    programs.insert(11, Box::new(day11::Instance));
    programs.insert(12, Box::new(day12::Instance));

    let program = programs
        .get(&opt.day)
        .unwrap_or_else(|| default_error_handler(format!("Undefined day: {}", opt.day).as_str()));
    let file_contents: Vec<String> = fs::File::open(format!("input/day{}.txt", opt.day))
        .and_then(|file| BufReader::new(file).lines().collect())
        .unwrap_or_else(default_error_handler);
    let result = program
        .run(file_contents)
        .unwrap_or_else(default_error_handler);

    println!("Part 1: {}", result.part1);
    if let Some(v) = result.part2 {
        println!("Part 2: {}", v)
    }
}
