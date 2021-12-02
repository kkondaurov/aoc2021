use std::fs;
use std::collections::VecDeque;

pub fn run(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let result_part1 = part1(contents.clone());
    println!("Day 2 part 1 result: {}", result_part1);

    let result_part2 = part2(contents.clone());
    println!("Day 2 part 2 result: {}", result_part2);
}

fn part1(contents: String) -> i32 {
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

fn part2(contents: String) -> i32 {
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
