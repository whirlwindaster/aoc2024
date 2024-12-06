use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

pub fn run(part: u32) {
    let file = File::open("src/day6/input").unwrap();

    match part {
        1 => part_one(file),
        2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(file: File) {
    let mut board: Vec<Vec<bool>> = vec![];
    let mut current = (0, 0, Direction::Up);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (row, line) in BufReader::new(file).lines().map(|l| l.unwrap()).enumerate() {
        board.push(vec![]);
        for (col, ch) in line.chars().enumerate() {
            if ch == '^' {
                current = (row, col, Direction::Up);
                visited.insert((current.0, current.1));
                board[row].push(false);
            } else if ch == '#' {
                board[row].push(true);
            } else {
                board[row].push(false);
            }
        }
    }

    while let Some(next) = next_tile(&board, current) {
        visited.insert((next.0, next.1));
        current = next;
    }

    println!("{}", visited.len());
}

fn part_two(file: File) {
    let mut board: Vec<Vec<bool>> = vec![];
    let mut start = (0, 0, Direction::Up);
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut num_positions = 0;

    for (row, line) in BufReader::new(file).lines().map(|l| l.unwrap()).enumerate() {
        board.push(vec![]);
        for (col, ch) in line.chars().enumerate() {
            if ch == '^' {
                start = (row, col, Direction::Up);
                board[row].push(false);
            } else if ch == '#' {
                board[row].push(true);
            } else {
                board[row].push(false);
            }
        }
    }

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if (row == start.0 && col == start.1) || board[row][col] {
                continue;
            }

            board[row][col] = true;
            visited.insert(start);

            let mut current = start;
            while let Some(next) = next_tile(&board, current) {
                if !visited.insert(next) {
                    println!("place dat at {row}, {col}");
                    num_positions += 1;
                    break;
                }
                current = next;
            }

            board[row][col] = false;
            visited.clear();
        }
    }

    println!("{num_positions}");
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn next_tile(board: &[Vec<bool>], current: (usize, usize, Direction)) -> Option<(usize, usize, Direction)> {
    use Direction::{Up, Right, Down, Left};
    match current.2 {
        Up => {
            if current.0 == 0 {
                None
            } else if board[current.0 - 1][current.1] {
                Some((current.0, current.1, Right))
            } else {
                Some((current.0 - 1, current.1, current.2))
            }
        },
        Right => {
            if current.1 >= board[current.0].len() - 1 {
                None
            } else if board[current.0][current.1 + 1] {
                Some((current.0, current.1, Down))
            } else {
                Some((current.0, current.1 + 1, current.2))
            }
        },
        Down => {
            if current.0 >= board.len() - 1 {
                None
            } else if board[current.0 + 1][current.1] {
                Some((current.0, current.1, Left))
            } else {
                Some((current.0 + 1, current.1, current.2))
            }
        },
        Left => {
            if current.1 == 0 {
                None
            } else if board[current.0][current.1 - 1] {
                Some((current.0, current.1, Up))
            } else {
                Some((current.0, current.1 - 1, current.2))
            }
        },
    }
}
