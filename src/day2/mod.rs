use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static MAX_DIFF: u32 = 3;

pub fn run() {
    println!(
        "{}",
        BufReader::new(File::open("src/day2/input").unwrap())
            .lines()
            .filter(|report| is_safe_report(report.as_ref().unwrap()))
            .count()
    );
}

fn is_safe_report(report: &str) -> bool {
    let mut levels = report.split_whitespace().map(|s| s.parse::<u32>().unwrap());
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
