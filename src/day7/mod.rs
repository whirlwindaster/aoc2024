use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(part: u32) {
    let file = File::open("src/day7/input").unwrap();

    match part {
        1 => part_one(file),
        2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(file: File) {
    println!(
        "{}",
        BufReader::new(file)
            .lines()
            .filter_map(|l| {
                let line = l.unwrap();
                let mut iter = line.split(":");
                let (target, operands) = (
                    iter.next().unwrap().parse::<u64>().unwrap(),
                    iter.next()
                        .unwrap()
                        .split_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect::<Vec<_>>(),
                );
                if try_operands(target, &operands) {
                    Some(target)
                } else {
                    None
                }
            })
            .sum::<u64>()
    );
}

fn part_two(file: File) {
    println!(
        "{}",
        BufReader::new(file)
            .lines()
            .filter_map(|l| {
                let line = l.unwrap();
                let mut iter = line.split(":");
                let (target, operands) = (
                    iter.next().unwrap().parse::<u64>().unwrap(),
                    iter.next()
                        .unwrap()
                        .split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<_>>(),
                );
                if try_operands_with_concat(target, &operands) {
                    Some(target)
                } else {
                    None
                }
            })
            .sum::<u64>()
    );
}

fn try_operands(target: u64, operands: &[u64]) -> bool {
    if (target == 0) ^ operands.is_empty() {
        false
    } else if target == 0 {
        true
    } else {
        let last = *operands.last().unwrap();
        if let Some(sub) = target.checked_sub(last) {
            if try_operands(sub, &operands[..operands.len() - 1]) {
                return true;
            }
        }
        if let Some(div) = target.checked_div(last) {
            if (target % last == 0) && try_operands(div, &operands[..operands.len() - 1]) {
                return true;
            }
        }
        false
    }
}

fn try_operands_with_concat(target: u64, operands: &[u64]) -> bool {
    if (target == 0) ^ operands.is_empty() {
        false
    } else if target == 0 {
        true
    } else {
        let last = *operands.last().unwrap();
        if let Some(sub) = target.checked_sub(last) {
            if try_operands_with_concat(sub, &operands[..operands.len() - 1]) {
                return true;
            }
        }
        if let Some(div) = target.checked_div(last) {
            if (target % last == 0)
                && try_operands_with_concat(div, &operands[..operands.len() - 1])
            {
                return true;
            }
        }
        if let Some(unconcat) = unconcatenate(target, last) {
            try_operands_with_concat(unconcat, &operands[..operands.len() - 1])
        } else {
            false
        }
    }
}

fn unconcatenate(lhs: u64, rhs: u64) -> Option<u64> {
    let mut magnitude: u64 = 10;
    while rhs / magnitude > 0 {
        magnitude *= 10;
    }
    if lhs % magnitude == rhs {
        Some(lhs / magnitude)
    } else {
        None
    }
}
