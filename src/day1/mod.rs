use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(part: u32) {
    let file = File::open("src/day1/input").unwrap();

    match part {
        1 => part_one(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(file: File) {
    let mut left_heap: BinaryHeap<u32> = BinaryHeap::new();
    let mut right_heap: BinaryHeap<u32> = BinaryHeap::new();

    read_to_heaps(BufReader::new(file), &mut left_heap, &mut right_heap);

    println!(
        "{}",
        calculate_difference(left_heap.into_iter_sorted(), right_heap.into_iter_sorted())
    );
}

fn read_to_heaps<R>(reader: R, heap1: &mut BinaryHeap<u32>, heap2: &mut BinaryHeap<u32>)
where
    R: BufRead,
{
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut vals = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
        heap1.push(vals.next().unwrap());
        heap2.push(vals.next().unwrap());
    }
}

fn calculate_difference<U, V>(it1: U, it2: V) -> u32
where
    U: IntoIterator<Item = u32>,
    V: IntoIterator<Item = u32>,
{
    it1.into_iter().zip(it2).fold(0, |mut acc: u32, (a, b)| {
        acc += a.abs_diff(b);
        acc
    })
}
