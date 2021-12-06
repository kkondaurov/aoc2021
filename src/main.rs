use std::env;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (day, filename) = parse_config(&args);

    match day {
        "1" => day01::run(filename),

        "2" => day02::run(filename),

        "3" => day03::run(filename),

        "4" => day04::run(filename),

        "5" => day05::run(filename),

        "6" => day06::run(filename),

        &_ => println!("Day {} not implemented!", day)
    }
}

fn parse_config(args: &[String]) -> (&str, &str) {
    if args.len() < 3 {
        panic!("Please privide day number and input filename");
    }

    let day = &args[1];
    let filename = &args[2];

    (day, filename)
}