use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static MAX_DIFF: u32 = 3;

pub fn run(part: u32) {
    let file = File::open("src/day2/input").unwrap();

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
            .filter(|report| is_safe_report(
                report
                    .as_ref()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
            ))
            .count()
    );
}

fn part_two(file: File) {
    println!(
        "{}",
        BufReader::new(file)
            .lines()
            .filter(|report| is_safe_dampened_report(
                report
                    .as_ref()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
            ))
            .count()
    );
}

fn is_safe_report<U>(report: U) -> bool
where
    U: IntoIterator<Item = u32>,
{
    let mut levels = report.into_iter();
    if let Some(start) = levels.next() {
        if let Some(mut prev) = levels.next() {
            let cmp = if start > prev && start - prev <= MAX_DIFF {
                |a: u32, b: u32| a > b && a - b <= MAX_DIFF
            } else if start < prev && prev - start <= MAX_DIFF {
                |a: u32, b: u32| a < b && b - a <= MAX_DIFF
            } else {
                return false;
            };

            for level in levels {
                if !cmp(prev, level) {
                    return false;
                }
                prev = level;
            }
        }
    }

    true
}

fn is_safe_dampened_report<U>(report: U) -> bool
where
    U: IntoIterator<Item = u32>,
{
    let levels: Vec<u32> = report.into_iter().collect();
    let len = levels.len();

    for i in 0..len {
        if is_safe_report(levels.iter().enumerate().filter_map(|(j, val)| {
            if i == j {
                None
            } else {
                Some(val.clone())
            }
        })) {
            return true;
        }
    }

    false
}
