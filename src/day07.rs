use std::fs;
use num;

pub fn run(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let result_part1 = dist_median(contents.clone());
    println!("Day 7 part 1 result: {}", result_part1);

    let result_part2 = dist_average(contents.clone());
    println!("Day 7 part 2 result: {}", result_part2);
}

fn dist_median(contents: String) -> i32 {
    let mut numbers: Vec<i32> =
        contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| (x.parse().expect("First line should only contain comma-separated numbers!")))
        .collect();

    numbers.sort();
    let median = numbers[numbers.len() / 2];

    let mut sum = 0;
    for n in numbers {
        sum += num::abs(n - median);
    }
    return sum;
}

fn dist_average(contents: String) -> i32 {
    let mut numbers: Vec<i32> =
        contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| (x.parse().expect("First line should only contain comma-separated numbers!")))
        .collect();

    numbers.sort();
    let mut sum_numbers = 0;
    let len = numbers.len() as f64;
    for n in &numbers {
        sum_numbers += n;
    }
    let average = (sum_numbers as f64 * 1.0 / len).round() as i32;
    let candidates = (average - 1)..(average + 1);

    let mut min_sum = 0;
    for candidate in candidates {
        let mut sum = 0;
        for n in &numbers {
            let moves = num::abs(n - candidate);
            sum += moves * (1 + moves) / 2;
        }
        if min_sum == 0 || min_sum > sum {
            min_sum = sum;
        }
    }

    return min_sum;
}