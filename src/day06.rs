use std::fs;
use std::collections::HashMap;

pub fn run(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let result_part1 = count_fish(contents.clone(), 80);
    println!("Day 6 part 1 result: {}", result_part1);

    let result_part2 = count_fish(contents.clone(), 256);
    println!("Day 6 part 2 result: {}", result_part2);
}

fn count_fish(contents: String, total_lifetime: i128) -> i128 {
    let numbers: Vec<i128> =
        contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| (x.parse().expect("First line should only contain comma-separated numbers!")))
        .collect();

    let mut precalc: HashMap<i128, i128> = HashMap::new();

    let mut starting_day = total_lifetime;
    while starting_day > -8 {
        let mut current_days_before_birth = 8;
        let mut birthdays: Vec<i128> = Vec::new();
        for current_day in starting_day..total_lifetime {
            if current_days_before_birth == 0 {
                current_days_before_birth = 6;
                if current_day < total_lifetime {
                    birthdays.push(current_day + 1);
                }
            } else {
                current_days_before_birth -= 1;
            }
        }
        let mut total = 1;
        for bday in birthdays {
            total += precalc[&bday];
        }
        precalc.insert(starting_day, total);
        starting_day -= 1;
    }

    let mut count = 0;
    for init_days_before_birth in numbers {
        let init_day = init_days_before_birth - 8;
        count += precalc[&init_day];
    }

    return count;
}