use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(part: u32) {
    let file = File::open("src/day8/input").unwrap();

    match part {
        1 => part_one(file),
        2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(file: File) {
    let mut buckets: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let mut max: (usize, usize) = (0, 0);

    for (row, line) in BufReader::new(file).lines().map(|l| l.unwrap()).enumerate() {
        line.chars()
            .enumerate()
            .filter(|(_, ch)| *ch != '.')
            .for_each(|(col, ch)| {
                if let Some(bucket) = buckets.get_mut(&ch) {
                    bucket.push((row, col));
                } else {
                    buckets.insert(ch, vec![(row, col)]);
                }
            });
        max.0 = row;
        max.1 = line.chars().count() - 1;
    }

    for bucket in buckets.values() {
        for (i, node) in bucket.iter().enumerate() {
            for other in &bucket[(i + 1)..] {
                antinodes.extend(antinode_coords(*node, *other, max));
            }
        }
    }

    println!("{}", antinodes.len());
}

fn part_two(file: File) {
    let mut buckets: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let mut max: (usize, usize) = (0, 0);

    for (row, line) in BufReader::new(file).lines().map(|l| l.unwrap()).enumerate() {
        line.chars()
            .enumerate()
            .filter(|(_, ch)| *ch != '.')
            .for_each(|(col, ch)| {
                if let Some(bucket) = buckets.get_mut(&ch) {
                    bucket.push((row, col));
                } else {
                    buckets.insert(ch, vec![(row, col)]);
                }
            });
        max.0 = row;
        max.1 = line.chars().count() - 1;
    }

    for bucket in buckets.values() {
        for (i, node) in bucket.iter().enumerate() {
            for other in &bucket[(i + 1)..] {
                antinodes.extend(harmonic_antinode_coords(*node, *other, max));
            }
        }
    }

    println!("{}", antinodes.len());
}

fn antinode_coords(
    c1: (usize, usize),
    c2: (usize, usize),
    max: (usize, usize),
) -> Vec<(usize, usize)> {
    let (x1, x2, y1, y2) = (c1.0 as isize, c2.0 as isize, c1.1 as isize, c2.1 as isize);
    [(x1 + x1 - x2, y1 + y1 - y2), (x2 + x2 - x1, y2 + y2 - y1)]
        .into_iter()
        .filter_map(|(x, y)| {
            if x > (max.0 as isize) || y > (max.1 as isize) {
                None
            } else {
                if let Ok(ux) = usize::try_from(x) {
                    if let Ok(uy) = usize::try_from(y) {
                        Some((ux, uy))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        })
        .collect::<Vec<_>>()
}

fn harmonic_antinode_coords(
    c1: (usize, usize),
    c2: (usize, usize),
    max: (usize, usize),
) -> Vec<(usize, usize)> {
    let (x1, x2, y1, y2) = (c1.0 as isize, c2.0 as isize, c1.1 as isize, c2.1 as isize);
    let mut out: Vec<(usize, usize)> = vec![];

    let mut curr = (x2, y2);
    while let Some(next) = {
        let next = (curr.0 + x1 - x2, curr.1 + y1 - y2);
        if next.0 > (max.0 as isize) || next.0 < 0 || next.1 > (max.1 as isize) || next.1 < 0 {
            None
        } else {
            Some(next)
        }
    } {
        out.push((next.0 as usize, next.1 as usize));
        curr = next;
    }

    curr = (x1, y1);
    while let Some(next) = {
        let next = (curr.0 + x2 - x1, curr.1 + y2 - y1);
        if next.0 > (max.0 as isize) || next.0 < 0 || next.1 > (max.1 as isize) || next.1 < 0 {
            None
        } else {
            Some(next)
        }
    } {
        out.push((next.0 as usize, next.1 as usize));
        curr = next;
    }

    out
}
