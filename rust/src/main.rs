use std::io::Write;
use std::str::FromStr;

extern crate regex;
#[macro_use]
extern crate lazy_static;

mod common;
mod day01;
mod day02;
mod day03;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        writeln!(std::io::stderr(), "Usage: {} day_number [args...]", args[0]).unwrap();
        std::process::exit(1);
    }

    let day: u32 = u32::from_str(&args[1]).expect("could not parse day selection; must be a number");

    let remaining = &args[2..];
    let runner = match day {
        1  => day01::run,
        2  => day02::run,
        3  => day03::run,
        8  => day08::run,
        9  => day09::run,
        10 => day10::run,
        11 => day11::run,
        12 => day12::run,
        13 => day13::run,
        14 => day14::run,
        15 => day15::run,
        16 => day16::run,
        17 => day17::run,
        18 => day18::run,
        19 => day19::run,
        20 => day20::run,
        _  => {
            println!("Couldn't find implementation for day {}", day);
            std::process::exit(1);
        }
    };

    runner(remaining);
}
