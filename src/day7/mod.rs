use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(part: u32) {
    let file = File::open("src/day6/input").unwrap();

    match part {
        1 => part_one(file),
        //2 => part_two(file),
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
                    iter.next().unwrap().parse::<u32>().unwrap(),
                    iter.next()
                        .unwrap()
                        .split_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect::<Vec<_>>(),
                );
                if try_operands(
                    target,
                    &mut Vec::with_capacity(operands.len() - 1),
                    &operands,
                    0,
                ) {
                    Some(target)
                } else {
                    None
                }
            })
            .sum::<u32>()
    );
}

fn try_operands(target: u32, operators: &mut Vec<bool>, operands: &[u32], idx: usize) -> bool {
    if idx < operands.len() - 1 {
        operators.push(true);
        if try_operands(target, operators, operands, idx + 1) {
            true
        } else {
            operators.push(false);
            if !try_operands(target, operators, operands, idx + 1) {
                operators.pop();
                false
            } else {
                true
            }
        }
    } else {
        if calculate(operators, &mut operands.to_owned()) == target {
            true
        } else {
            operators.pop();
            false
        }
    }
}

fn calculate(operators: &[bool], operands: &mut [u32]) -> u32 {
    for i in 0..operators.len() {
        if operators[i] {
            operands[i + 1] = operands[i] * operands[i + 1];
            operands[i] = 0;
        }
    }

    operands.iter().sum()
}
