use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::neighbors;

pub fn run(part: u32) {
    let file = File::open("src/day10/input").unwrap();

    match part {
        1 => part_one(file),
        2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(file: File) {
    let trail = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (row, heights) in trail.iter().enumerate() {
        for col in heights
            .iter()
            .enumerate()
            .filter_map(|(c, h)| if *h == 0 { Some(c) } else { None })
        {
            sum += count_reachable_endpoints((row, col), &trail, &mut HashSet::new());
        }
    }

    println!("{sum}");
}

fn part_two(file: File) {
    let trail = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (row, heights) in trail.iter().enumerate() {
        for col in heights
            .iter()
            .enumerate()
            .filter_map(|(c, h)| if *h == 0 { Some(c) } else { None })
        {
            sum += count_paths((row, col), &trail);
        }
    }

    println!("{sum}");
}

fn count_reachable_endpoints(
    current: (usize, usize),
    trail: &[Vec<u32>],
    endpoints: &mut HashSet<(usize, usize)>,
) -> usize {
    if trail[current.0][current.1] == 9 {
        endpoints.insert(current);
        1
    } else {
        let mut paths = 0;
        for neighbor in neighbors(current, &trail)
            .into_iter()
            .filter(|neighbor| trail[neighbor.0][neighbor.1] == trail[current.0][current.1] + 1)
        {
            if !endpoints.contains(&neighbor) {
                paths += count_reachable_endpoints(neighbor, trail, endpoints);
            }
        }
        paths
    }
}

fn count_paths(current: (usize, usize), trail: &[Vec<u32>]) -> usize {
    if trail[current.0][current.1] == 9 {
        1
    } else {
        neighbors(current, &trail)
            .into_iter()
            .filter(|neighbor| trail[neighbor.0][neighbor.1] == trail[current.0][current.1] + 1)
            .fold(0, |mut acc, neighbor| {
                acc += count_paths(neighbor, &trail);
                acc
            })
    }
}
