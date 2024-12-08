use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(part: u32) {
    let file = File::open("src/day8/test").unwrap();

    match part {
        1 => part_one(file),
        //2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(file: File) {
    let mut buckets: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let mut max: (usize, usize) = (0, 0);

    for (row, line) in BufReader::new(file).lines().map(|l| l.unwrap()).enumerate() {
        max.1 = line.chars().count();
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
    }

    //println!("{:?}", buckets);

    for bucket in buckets.values() {
        for (i, node) in bucket.iter().enumerate() {
            for other in &bucket[(i + 1)..] {
                //for other in bucket.iter().filter(|n| *n != node) {
                antinodes.extend(antinode_coords(*node, *other, max));
            }
        }
    }

    //println!("{:?}", antinodes);
    println!("{}", antinodes.len());
}

//fn antinode_coords(
//    a: (usize, usize),
//    b: (usize, usize),
//    max: (usize, usize),
//) -> Vec<(usize, usize)> {
//    let mut x_vals: [Option<usize>; 2] = [None, None];
//    let mut y_vals: [Option<usize>; 2] = [None, None];
//
//    if let Some(diff) = a.0.checked_sub(b.0) {
//        x_vals[0] = b.0.checked_sub(diff);
//        x_vals[1] = if a.0 + diff > max.0 {
//            None
//        } else {
//            Some(a.0 + diff)
//        }
//    } else {
//        let diff = b.0 - a.0;
//        x_vals[1] = a.0.checked_sub(diff);
//        x_vals[0] = if b.0 + diff > max.0 {
//            None
//        } else {
//            Some(b.0 + diff)
//        }
//    };
//
//    if let Some(diff) = a.1.checked_sub(b.1) {
//        y_vals[0] = b.1.checked_sub(diff);
//        y_vals[1] = if a.1 + diff > max.1 {
//            None
//        } else {
//            Some(a.1 + diff)
//        }
//    } else {
//        let diff = b.1 - a.1;
//        y_vals[1] = a.1.checked_sub(diff);
//        y_vals[0] = if b.1 + diff > max.1 {
//            None
//        } else {
//            Some(b.1 + diff)
//        }
//    };
//
//    //println!("{}, {}", max.0, max.1);
//
//    println!("{:?}, {:?}:", a, b);
//    println!("\t{:?}", x_vals);
//    println!("\t{:?}", y_vals);
//
//    x_vals
//        .into_iter()
//        .zip(y_vals)
//        .filter_map(|(x, y)| {
//            if let Some(x) = x {
//                if let Some(y) = y {
//                    Some((x, y))
//                } else {
//                    None
//                }
//            } else {
//                None
//            }
//        })
//        .collect::<Vec<_>>()
//}

fn antinode_coords(
    c1: (usize, usize),
    c2: (usize, usize),
    max: (usize, usize),
) -> Vec<(usize, usize)> {
    let (x1, x2, y1, y2) = (c1.0 as f64, c2.0 as f64, c1.1 as f64, c2.1 as f64);
    let m = (y2 - y1) / (x2 - x1);
    let b = y1 - (m * x1);

    let x_outs = [x1 + (x1 - x2), x2 + (x2 - x1)];
    println!("{:?}, {:?}:", c1, c2);
    println!("b: {b}");
    println!("m: {m}");
    println!("{:?}", x_outs);
    let outs = x_outs
        .into_iter()
        .filter_map(|x| {
            let y = m * x + b;
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
        .collect::<Vec<_>>();

    for out in outs.iter() {
        println!("\t{:?}", out);
    }

    outs
}
