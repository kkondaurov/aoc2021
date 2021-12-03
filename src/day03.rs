use std::fs;
use num;

pub fn run(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let result_part1 = part1(contents.clone());
    println!("Day 3 part 1 result: {}", result_part1);

    let result_part2 = part2(contents.clone());
    println!("Day 3 part 2 result: {}", result_part2);
}

fn part1(contents: String) -> i32 {
    let mut sums = [0; 12];
    let mut lines_count = 0;

    for line in contents.lines() {
        let chars = line.chars().enumerate();
        for (i, chr) in chars {
            if chr == '1' {
                sums[i] += 1;
            }
        }
        lines_count += 1;
    }

    let mut gamma = 0;
    let mut exp = 0;
    while exp < 12 {
        if sums[exp] > lines_count / 2 {
            gamma += num::pow(2, 11 - exp)
        }
        exp += 1;
    }

    return gamma * (gamma ^ 4095);
}

fn part2(contents: String) -> i32 {
    let oxy_str = oxy(contents.lines().collect(), 0);
    let co2_str = co2(contents.lines().collect(), 0);
    let oxy = int_from_binary_string(oxy_str);
    let co2 = int_from_binary_string(co2_str);
    return oxy * co2;
}

fn oxy(lines: Vec<&str>, index: usize) -> &str {
    let (ones, zeroes) = split_by_bit(lines, index);

    if ones.len() == zeroes.len() && ones.len() == 1 {
        return ones.first().unwrap();
    } else if ones.len() >= zeroes.len() {
        return oxy(ones, index + 1);
    } else {
        return oxy(zeroes, index + 1);
    }
}

fn co2(lines: Vec<&str>, index: usize) -> &str {
    let (ones, zeroes) = split_by_bit(lines, index);

    if ones.len() == zeroes.len() && ones.len() == 1 {
        return zeroes.first().unwrap();
    } else if ones.len() >= zeroes.len() {
        return co2(zeroes, index + 1);
    } else {
        return co2(ones, index + 1);
    }
}

fn int_from_binary_string(s: &str) -> i32 {
    let str_len = s.len();
    let mut res = 0;
    let mut exp = 0;

    for chr in s.chars() {
        if chr == '1' {
            res += num::pow(2, str_len - 1 - exp);
        }
        exp += 1;
    }

    return res;
}

fn split_by_bit(lines: Vec<&str>, index: usize) -> (Vec<&str>, Vec<&str>) {
    let mut ones = Vec::new();
    let mut zeroes = Vec::new();

    for line in lines {
        if line.chars().nth(index).unwrap() == '1' {
            ones.push(line);
        } else {
            zeroes.push(line);
        }
    }
    return (ones, zeroes);
}
