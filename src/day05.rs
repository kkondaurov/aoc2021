use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub fn run(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let result_part1 = count_intersections(contents.clone(), true);
    println!("Day 5 part 1 result: {}", result_part1);

    let result_part2 = count_intersections(contents.clone(), false);
    println!("Day 5 part 2 result: {}", result_part2);
}

fn count_intersections(contents: String, skip_diagonal: bool) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    for line in contents.lines() {
        let cap = re.captures(line).unwrap();
        let x0:i32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let y0:i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let x1:i32 = cap.get(3).unwrap().as_str().parse().unwrap();
        let y1:i32 = cap.get(4).unwrap().as_str().parse().unwrap();

        if skip_diagonal && !(x0 == x1 || y0 == y1) {
            continue;
        }

        let dx = delta(x0, x1);
        let dy = delta(y0, y1);

        let mut x: i32 = x0;
        let mut y: i32 = y0;
        map_inc(&mut map, x, y);

        while x != x1 || y != y1 {
            x += dx;
            y += dy;
            map_inc(&mut map, x, y);
        }
    }
    return map.iter().fold(0, |acc, (_coords, &val)| acc + (if val > 1 { 1 } else { 0 } ));
}

fn map_inc(map: &mut HashMap<(i32, i32), i32>, x:i32, y:i32) {
    if let Some(pos) = map.get_mut(&(x, y)) {
        *pos += 1;
    } else {
        map.insert((x, y), 1);
    }
}

fn delta(v0:i32, v1:i32) -> i32 {
    return
        if v1 > v0 {
            1
        } else if v1 < v0 {
            -1
        } else {
            0
        };
}