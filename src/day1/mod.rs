use std::{
    fs::File,
    io::{BufRead, BufReader},
    mem::swap,
};

pub fn run() {
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];

    read_file(
        &File::open("src/day1/input").unwrap(),
        &mut left_list,
        &mut right_list,
    );

    println!("{}", calculate_difference(&left_list, &right_list));
}

fn read_file(fp: &File, l1: &mut Vec<u32>, l2: &mut Vec<u32>) {
    let mut buf = String::with_capacity(14);
    let mut reader = BufReader::new(fp);

    loop {
        let bytes_read = reader.read_line(&mut buf).unwrap();
        let mut iter = buf.as_str().split_whitespace();

        if bytes_read == 0 {
            break;
        }

        insert_sorted(l1, iter.next().unwrap().parse::<u32>().unwrap());
        insert_sorted(l2, iter.next().unwrap().parse::<u32>().unwrap());

        buf.clear();
    }
}

fn insert_sorted<T>(vec: &mut Vec<T>, val: T)
where
    T: Ord,
{
    let mut i = vec.len();
    vec.push(val);

    while i > 0 && vec[i - 1] > vec[i] {
        let tup = (&mut vec[i - 1..=i]).split_at_mut(1);
        swap(&mut tup.0[0], &mut tup.1[0]);
        i -= 1;
    }
}

fn calculate_difference(l1: &[u32], l2: &[u32]) -> u32 {
    l1.iter().zip(l2.iter()).fold(0, |mut acc: u32, (a, b)| {
        acc += a.abs_diff(*b);
        acc
    })
}
