#![feature(linked_list_cursors)]
#![feature(binary_heap_into_iter_sorted)]
use clap::Parser;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    day: u32,

    #[arg(short, long, default_value_t = 1)]
    part: u32,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => day1::run(args.part),
        2 => day2::run(args.part),
        3 => day3::run(args.part),
        4 => day4::run(args.part),
        5 => day5::run(args.part),
        6 => day6::run(args.part),
        7 => day7::run(args.part),
        8 => day8::run(args.part),
        9 => day9::run(args.part),
        10 => day10::run(args.part),
        11 => day11::run(args.part),
        _ => panic!("day {} not implemented", args.day),
    };
}

pub fn neighbors<T>(of: (usize, usize), matrix: &[Vec<T>]) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::with_capacity(2);

    if of.0 < matrix.len() - 1 {
        out.push((of.0 + 1, of.1));
    }

    if of.0 > 0 {
        out.push((of.0 - 1, of.1));
    }

    if of.1 < matrix[of.1].len() - 1 {
        out.push((of.0, of.1 + 1));
    }

    if of.1 > 0 {
        out.push((of.0, of.1 - 1));
    }

    out
}
