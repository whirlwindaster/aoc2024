use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(part: u32) {
    let file = File::open("src/day1/input").unwrap();

    match part {
        1 => part_one(file),
        2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(file: File) {
    let mut left_heap: BinaryHeap<u32> = BinaryHeap::new();
    let mut right_heap: BinaryHeap<u32> = BinaryHeap::new();

    read_to_heaps(BufReader::new(file), &mut left_heap, &mut right_heap);

    println!(
        "{}",
        calculate_total_distance(left_heap.into_iter_sorted(), right_heap.into_iter_sorted())
    );
}

fn part_two(file: File) {
    let mut left_vec: Vec<u32> = Vec::new();
    let mut right_map: HashMap<u32, u32> = HashMap::new();

    read_to_vec_and_map(BufReader::new(file), &mut left_vec, &mut right_map);

    println!("{}", calculate_similarity_score(left_vec, &right_map));
}

fn read_to_vec_and_map<R>(reader: R, vec: &mut Vec<u32>, map: &mut HashMap<u32, u32>)
where
    R: BufRead,
{
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut vals = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());

        vec.push(vals.next().unwrap());

        let right_val = vals.next().unwrap();
        if let Some(quantity) = map.get_mut(&right_val) {
            *quantity += 1;
        } else {
            map.insert(right_val, 1);
        }
    }
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

fn calculate_similarity_score<U>(list1: U, list2: &HashMap<u32, u32>) -> u32
where
    U: IntoIterator<Item = u32>,
{
    list1.into_iter().fold(0, |mut acc, val| {
        acc += list2.get(&val).unwrap_or(&0) * val;
        acc
    })
}

fn calculate_total_distance<U, V>(list1: U, list2: V) -> u32
where
    U: IntoIterator<Item = u32>,
    V: IntoIterator<Item = u32>,
{
    list1
        .into_iter()
        .zip(list2)
        .fold(0, |mut acc: u32, (a, b)| {
            acc += a.abs_diff(b);
            acc
        })
}
