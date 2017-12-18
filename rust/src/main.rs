use std::io::Write;
use std::str::FromStr;

mod common;
mod day01;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        writeln!(std::io::stderr(), "Usage: {} day_number [args...]", args[0]).unwrap();
        std::process::exit(1);
    }

    let day: u32 = u32::from_str(&args[1]).expect("could not parse day selection; must be a number");

    let remaining = &args[2..];
    let runner = match day {
        1 => day01::run,
        _ => {
            println!("Couldn't find implementation for day {}", day);
            std::process::exit(1);
        }
    };

    runner(remaining);
}
