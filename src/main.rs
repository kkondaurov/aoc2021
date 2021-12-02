use std::env;
use std::fs;
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (day, filename) = parse_config(&args);

    match day {
        "1" => day01(filename),

        "2" => day02(filename),

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


fn day02(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let result_part1 = day02_part1(contents.clone());
    println!("Day 2 part 1 result: {}", result_part1);

    let result_part2 = day02_part2(contents.clone());
    println!("Day 2 part 2 result: {}", result_part2);
}

fn day02_part1(contents: String) -> i32 {
    let mut position = 0;
    let mut depth = 0;

    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        let command = iter.next().unwrap();
        let arg = iter.next().unwrap().parse::<i32>().unwrap();
        match command {
            "forward" => position += arg,

            "up" => if depth >= arg {
                depth -= arg;
            },

            "down" => depth += arg,

            &_ => println!("Command {} not implemented!", command)
        }
    }
    return position * depth;
}

fn day02_part2(contents: String) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        let command = iter.next().unwrap();
        let arg = iter.next().unwrap().parse::<i32>().unwrap();
        match command {
            "forward" => {
                position += arg;
                let new_depth = depth + aim * arg;
                if new_depth < 0 {
                    depth = 0;
                } else {
                    depth = new_depth;
                }
            },

            "up" => aim -= arg,

            "down" => aim += arg,

            &_ => println!("Command {} not implemented!", command)
        }
        println!("{} {}, position: {}, depth: {}, aim: {}", command, arg, position, depth, aim);
    }

    return position * depth;
}

fn day01(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let result_part1 = day01_part1(contents.clone());
    println!("Day 1 part 1 result: {}", result_part1);

    let result_part2 = day01_part2(contents.clone());
    println!("Day 1 part 2 result: {}", result_part2);
}

fn day01_part1(contents: String) -> i32 {
    let mut seen_prev = false;
    let mut prev_value = 0;
    let mut increases = 0;

    for line in contents.lines() {
        let current_value = line.parse::<i32>().unwrap();
        if seen_prev {
            if current_value > prev_value {
                increases += 1;
            }
        } else {
            seen_prev = true;
        }
        prev_value = current_value;
    }
    return increases;
}

fn day01_part2(contents: String) -> i32 {
    let mut last3 = VecDeque::from([0, 0, 0]);
    let mut seen = 0;
    let mut prev_sum = 0;
    let mut increases = 0;

    for line in contents.lines() {
        let current_value = line.parse::<i32>().unwrap();
        last3.pop_front();
        last3.push_back(current_value);
        let mut current_sum = 0;
        for val in last3.iter() {
            current_sum += val;
        }
        if seen >= 3 && current_sum > prev_sum {
            increases += 1;
        }
        seen += 1;
        prev_sum = current_sum;
    }

    return increases;
}