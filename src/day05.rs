use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub fn run(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let result_part1 = part1(contents.clone());
    println!("Day 5 part 1 result: {}", result_part1);

    let result_part2 = part2(contents.clone());
    println!("Day 5 part 2 result: {}", result_part2);
}

fn part1(contents: String) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    for line in contents.lines() {
        let cap = re.captures(line).unwrap();
        let x0:i32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let y0:i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let x1:i32 = cap.get(3).unwrap().as_str().parse().unwrap();
        let y1:i32 = cap.get(4).unwrap().as_str().parse().unwrap();

        if !(x0 == x1 || y0 == y1) {
            continue;
        }
        let dx =
            if x1 > x0 {
                1
            } else if x1 < x0 {
                -1
            } else {
                0
            };
        let dy =
            if y1 > y0 {
                1
            } else if y1 < y0 {
                -1
            } else {
                0
            };

        let mut x: i32 = x0;
        let mut y: i32 = y0;

        if let Some(pos) = map.get_mut(&(x, y)) {
            *pos += 1;
        } else {
            map.insert((x, y), 1);
        }

        while x != x1 || y != y1 {
            x += dx;
            y += dy;
            if let Some(pos) = map.get_mut(&(x, y)) {
                *pos += 1;
            } else {
                map.insert((x, y), 1);
            }
        }
    }
    let mut cnt = 0;
    for (_, val) in map {
        if val > 1 {
            cnt += 1;
        }
    }
    return cnt;
}


fn part2(contents: String) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    for line in contents.lines() {
        let cap = re.captures(line).unwrap();
        let x0:i32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let y0:i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let x1:i32 = cap.get(3).unwrap().as_str().parse().unwrap();
        let y1:i32 = cap.get(4).unwrap().as_str().parse().unwrap();

        let dx =
            if x1 > x0 {
                1
            } else if x1 < x0 {
                -1
            } else {
                0
            };
        let dy =
            if y1 > y0 {
                1
            } else if y1 < y0 {
                -1
            } else {
                0
            };

        let mut x: i32 = x0;
        let mut y: i32 = y0;

        if let Some(pos) = map.get_mut(&(x, y)) {
            *pos += 1;
        } else {
            map.insert((x, y), 1);
        }

        while x != x1 || y != y1 {
            x += dx;
            y += dy;
            if let Some(pos) = map.get_mut(&(x, y)) {
                *pos += 1;
            } else {
                map.insert((x, y), 1);
            }
        }
    }
    let mut cnt = 0;
    for (_, val) in map {
        if val > 1 {
            cnt += 1;
        }
    }
    return cnt;
}
