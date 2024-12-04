use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn run(part: u32) {
    let file = File::open("src/day3/input").unwrap();

    match part {
        1 => part_one(file),
        2 => part_two(file),
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

fn part_two(mut file: File) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut contents = String::new();
    let mut sum = 0;
    file.read_to_string(&mut contents).unwrap();
    let mut contents = contents.as_str();

    while let Some((do_mul, rest)) = contents.split_once("don't()") {
        re.captures_iter(&do_mul).for_each(|cap| {
            sum += cap.get(1).unwrap().as_str().parse::<u32>().unwrap()
            * cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        });

        if let Some((_, continuation)) = rest.split_once("do()") {
            contents = continuation;
        } else {
            break;
        }
    }

    println!("{sum}");
}
