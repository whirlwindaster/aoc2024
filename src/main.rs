#![feature(binary_heap_into_iter_sorted)]
use clap::Parser;

pub mod day1;
pub mod day2;

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
        1 => day1::run(),
        2 => day2::run(),
        _ => panic!("day {} not implemented", args.day),
    };
}
