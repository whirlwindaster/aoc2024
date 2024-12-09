use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(part: u32) {
    let file = File::open("src/day9/input").unwrap();

    match part {
        1 => part_one(file),
        2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

enum Toggler {
    NonEmpty(u32),
    Empty(u32),
}

fn part_one(file: File) {
    let mut id_or_free: Toggler = Toggler::NonEmpty(0);
    let mut disk: Vec<Option<u32>> = BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|ch| {
            let spaces = ch.to_digit(10).unwrap();
            match id_or_free {
                Toggler::NonEmpty(id) => {
                    id_or_free = Toggler::Empty(id);
                    vec![Some(id); spaces as usize]
                }
                Toggler::Empty(id) => {
                    id_or_free = Toggler::NonEmpty(id + 1);
                    vec![None; spaces as usize]
                }
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut rightmost_full = disk
        .iter()
        .rev()
        .enumerate()
        .find_map(|(i, space)| {
            if space.is_some() {
                Some(disk.len() - 1 - i)
            } else {
                None
            }
        })
        .unwrap();

    let mut leftmost_empty = disk
        .iter()
        .enumerate()
        .find_map(|(i, space)| if space.is_none() { Some(i) } else { None })
        .unwrap();

    while rightmost_full >= leftmost_empty {
        disk.swap(rightmost_full, leftmost_empty);

        leftmost_empty = disk
            .iter()
            .skip(leftmost_empty + 1)
            .enumerate()
            .find_map(|(i, space)| {
                if space.is_none() {
                    Some(i + leftmost_empty + 1)
                } else {
                    None
                }
            })
            .unwrap();

        rightmost_full = disk
            .iter()
            .rev()
            .skip(disk.len() - rightmost_full)
            .enumerate()
            .find_map(|(i, space)| {
                if space.is_some() {
                    Some(rightmost_full - i - 1)
                } else {
                    None
                }
            })
            .unwrap();
    }

    println!(
        "{}",
        disk.into_iter()
            .filter_map(|space| space)
            .fuse()
            .enumerate()
            .fold(0 as usize, |mut acc, (i, id)| {
                acc += i * (id as usize);
                acc
            })
    );
}

#[derive(Debug)]
struct Block {
    id: usize,
    filled: u32,
    free: u32,
}

fn part_two(file: File) {
    let mut blocks: Vec<Block> = Vec::new();
    let mut attempted: HashSet<usize> = HashSet::new();

    for (i, digit) in BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .enumerate()
    {
        if i % 2 == 0 {
            blocks.push(Block {
                id: i / 2,
                filled: digit,
                free: 0,
            });
        } else {
            blocks.last_mut().unwrap().free = digit;
        }
    }

    let mut curr = blocks.len() - 1;
    while curr > 0 {
        if !attempted.insert(blocks[curr].id) {
            curr -= 1;
        } else if let Some(available) =
            blocks.iter().take(curr).enumerate().find_map(|(j, block)| {
                if block.free >= blocks[curr].filled {
                    Some(j)
                } else {
                    None
                }
            })
        {
            let popped = blocks.remove(curr);
            blocks[curr - 1].free += popped.filled + popped.free;
            blocks.insert(available + 1, popped);
            blocks[available + 1].free = blocks[available].free - blocks[available + 1].filled;
            blocks[available].free = 0;
        } else {
            curr -= 1;
        }
    }

    let mut sum = 0;
    let mut position = 0;
    for block in blocks {
        for pos in position..(position + block.filled) {
            sum += (pos as usize) * block.id;
        }
        position += block.free + block.filled;
    }

    println!("{sum}");
}
