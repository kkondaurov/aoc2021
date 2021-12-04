use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Board {
    rows: [i32; 5],
    columns: [i32; 5],
    numbers: HashMap<i32, (usize, usize)>,
    discarded: bool,
}

pub fn run(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let result_part1 = part1(contents.clone());
    println!("Day 4 part 1 result: {}", result_part1);

    let result_part2 = part2(contents.clone());
    println!("Day 4 part 2 result: {}", result_part2);
}

fn part1(contents: String) -> i32 {
    let mut lines: Vec<&str> = contents.lines().collect();
    let draws: Vec<&str> =
        lines
        .remove(0)
        .split(",")
        .collect();

    let num_draws: Vec<i32> =
        draws
        .iter()
        .map(|&x| x.parse().expect("First line should only contain comma-separated numbers!"))
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    while lines.len() > 0 {
        let board_lines: Vec<&str> = lines.drain(0..6).collect();
        let mut row = 0;
        let mut row_numbers: HashMap<i32, (usize, usize)> = HashMap::new();
        for line in board_lines {
            if line != "" {
                let mut col = 0;
                let num_strs: Vec<&str> = line.split_whitespace().collect();
                let numbers: Vec<i32> = num_strs.iter().map(|&x| x.trim().parse().expect("Rows should only contain whitespace-separated numbers")).collect();
                for n in numbers {
                    row_numbers.insert(n, (row, col));
                    col += 1;
                }
                row += 1;
            }
        }
        let board = Board{
            rows: [5; 5],
            columns: [5; 5],
            numbers: row_numbers,
            discarded: false
        };
        boards.push(board);
    }

    for draw in num_draws {
        for board in boards.iter_mut() {
            if board.numbers.contains_key(&draw) {
                let (row, col) = board.numbers[&draw];
                board.rows[row] -= 1;
                board.columns[col] -= 1;
                board.numbers.remove(&draw);
                if board.rows[row] == 0 || board.columns[col] == 0 {
                    let sum:i32 = board.numbers.keys().sum();
                    return sum * draw;
                }
            }
        }
    }

    return 0;
}

fn part2(contents: String) -> i32 {
    let mut lines: Vec<&str> = contents.lines().collect();
    let draws: Vec<&str> =
        lines
        .remove(0)
        .split(",")
        .collect();

    let num_draws: Vec<i32> =
        draws
        .iter()
        .map(|&x| x.parse().expect("First line should only contain comma-separated numbers!"))
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    while lines.len() > 0 {
        let board_lines: Vec<&str> = lines.drain(0..6).collect();
        let mut row = 0;
        let mut row_numbers: HashMap<i32, (usize, usize)> = HashMap::new();
        for line in board_lines {
            if line != "" {
                let mut col = 0;
                let num_strs: Vec<&str> = line.split_whitespace().collect();
                let numbers: Vec<i32> = num_strs.iter().map(|&x| x.trim().parse().expect("Rows should only contain whitespace-separated numbers")).collect();
                for n in numbers {
                    row_numbers.insert(n, (row, col));
                    col += 1;
                }
                row += 1;
            }
        }
        let board = Board{
            rows: [5; 5],
            columns: [5; 5],
            numbers: row_numbers,
            discarded: false
        };
        boards.push(board);
    }

    let mut last_board_result = 0;
    for draw in num_draws {
        for board in boards.iter_mut() {
            if board.numbers.contains_key(&draw) && !board.discarded {
                let (row, col) = board.numbers[&draw];
                board.rows[row] -= 1;
                board.columns[col] -= 1;
                board.numbers.remove(&draw);
                if board.rows[row] == 0 || board.columns[col] == 0 {
                    board.discarded = true;
                    let sum:i32 = board.numbers.keys().sum();
                    last_board_result = sum * draw;
                }
            }
        }
    }
    return last_board_result;

}

