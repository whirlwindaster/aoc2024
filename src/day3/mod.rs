use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn run(part: u32) {
    let file = File::open("src/day3/input").unwrap();

    match part {
        1 => part_one(file),
        //2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(mut file: File) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!(
        "{}",
        re.captures_iter(&contents).fold(0, |mut acc, cap| {
            acc += cap.get(1).unwrap().as_str().parse::<u32>().unwrap()
                * cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
            acc
        })
    );
}
