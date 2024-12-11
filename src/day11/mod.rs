use std::{
    collections::{HashMap, LinkedList},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(part: u32) {
    let file = File::open("src/day11/input").unwrap();

    match part {
        1 => part_one(file),
        2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(file: File) {
    let mut stones = BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<LinkedList<u64>>();

    watch_stones(&mut stones, 25);
    println!("{}", stones.len());
}

fn part_two(file: File) {
    let mut stones: HashMap<u64, u64> = HashMap::new();

    for stone in BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
    {
        if let Some(count) = stones.get_mut(&stone) {
            *count += 1;
        } else {
            stones.insert(stone, 1);
        }
    }

    for _ in 0..75 {
        let mut next = HashMap::new();

        for (stone, count) in stones {
            for next_stone in next_stones(stone) {
                if let Some(next_count) = next.get_mut(&next_stone) {
                    *next_count += count;
                } else {
                    next.insert(next_stone, count);
                }
            }
        }

        stones = next;
    }

    println!("{}", stones.values().sum::<u64>())
}

fn next_stones(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else if let Some((left, right)) = split_num(stone) {
        vec![left, right]
    } else {
        vec![stone * 2024]
    }
}

fn watch_stones(stones: &mut LinkedList<u64>, blinks: u32) {
    for i in 0..blinks {
        let mut cursor = stones.cursor_front_mut();

        loop {
            let current;
            if let Some(value) = cursor.current() {
                current = value.clone();
            } else {
                break;
            }

            if current == 0 {
                *cursor.current().unwrap() = 1;
            } else if let Some((left, right)) = split_num(current) {
                cursor.insert_before(left);
                *cursor.current().unwrap() = right;
            } else {
                *cursor.current().unwrap() *= 2024;
            }

            cursor.move_next();
        }

        println!("blinked {i} times");
    }
}

fn split_num(x: u64) -> Option<(u64, u64)> {
    let digits = num_digits(x);

    if digits % 2 == 1 {
        None
    } else {
        Some((
            x / ((10 as u64).pow(digits / 2)),
            x % ((10 as u64).pow(digits / 2)),
        ))
    }
}

fn num_digits(x: u64) -> u32 {
    let mut magnitude = 1;

    while x / ((10 as u64).pow(magnitude)) != 0 {
        magnitude += 1;
    }

    magnitude
}
