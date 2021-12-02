use std::fs;

pub fn run(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let result_part1 = part1(contents.clone());
    println!("Day 2 part 1 result: {}", result_part1);

    let result_part2 = part2(contents.clone());
    println!("Day 2 part 2 result: {}", result_part2);
}

fn part1(contents: String) -> i32 {
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

fn part2(contents: String) -> i32 {
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
    }

    return position * depth;
}
