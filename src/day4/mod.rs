use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Range;

static SEARCH: &str = "XMAS";

pub fn run(part: u32) {
    let file = File::open("src/day4/input").unwrap();

    match part {
        1 => part_one(file),
        //2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(file: File) {
    let contents = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start, rest) = SEARCH.split_at(2);
    let mut start_iter = start.chars();
    let first = start_iter.next().unwrap();
    let second = start_iter.next().unwrap();
    let mut sum = 0;

    for (row, line) in contents.iter().enumerate() {
        for (col, _) in line.into_iter().enumerate().filter(|(_, ch)| **ch == first) {
            for candidate in find_candidates(&contents, (row, col), second) {
                if is_word_match(&contents, candidate, rest, |(r, c)| {
                    let row_diff = (candidate.0 as isize) - (row as isize);
                    let col_diff = (candidate.1 as isize) - (col as isize);

                    if (r as isize) + row_diff < 0
                        || (r as isize) + row_diff >= (line.len() as isize)
                        || (c as isize) + col_diff < 0
                        || (c as isize) + col_diff >= (line.len() as isize)
                    {
                        None
                    } else {
                        Some((
                            <isize as TryInto<usize>>::try_into((r as isize) + row_diff).unwrap(),
                            <isize as TryInto<usize>>::try_into((c as isize) + col_diff).unwrap(),
                        ))
                    }
                }) {
                    sum += 1;
                }
            }
        }
    }

    println!("{sum}");
}

fn get_range(i: usize, min: usize, max: usize) -> Range<usize> {
    let start = if i == 0 || i - 1 <= min { min } else { i - 1 };
    let end = if i + 2 >= max { max } else { i + 2 };
    start..end
}

fn find_candidates(
    matrix: &[Vec<char>],
    current: (usize, usize),
    search: char,
) -> Vec<(usize, usize)> {
    get_range(current.0, 0, matrix.len())
        .into_iter()
        .fold(Vec::new(), |mut acc, row| {
            let chars_range = get_range(current.1, 0, matrix[row].len());
            let offset = chars_range.start;
            acc.append(
                &mut matrix[row][chars_range]
                    .iter()
                    .enumerate()
                    .filter_map(|(col, ch)| {
                        if (row, col + offset) != current && *ch == search {
                            Some((row, col + offset))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            );
            acc
        })
}

fn is_word_match<F>(matrix: &[Vec<char>], current: (usize, usize), word: &str, get_next: F) -> bool
where
    F: Fn((usize, usize)) -> Option<(usize, usize)>,
{
    if word.is_empty() {
        true
    } else if let Some(next) = get_next(current) {
        if matrix[next.0][next.1] == word.chars().next().unwrap() {
            is_word_match(matrix, next, &word[1..], get_next)
        } else {
            false
        }
    } else {
        false
    }
}
