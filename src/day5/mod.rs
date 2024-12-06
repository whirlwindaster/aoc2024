use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use petgraph::prelude::DiGraphMap;

pub fn run(part: u32) {
    let file = File::open("src/day5/input").unwrap();

    match part {
        1 => part_one(file),
        2 => part_two(file),
        _ => panic!("part {part} not implemented"),
    };
}

fn part_one(file: File) {
    let mut lines = BufReader::new(file).lines().map(|l| l.unwrap());

    let mut deps: DiGraphMap<u32, bool> = DiGraphMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let nodes = line
            .split('|')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        deps.add_edge(nodes[1], nodes[0], false);
    }

    println!(
        "{}",
        lines
            .map(|line| get_update_val(
                &line
                    .split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
                &mut deps
            ))
            .sum::<u32>()
    )
}

fn part_two(file: File) {
    let mut lines = BufReader::new(file).lines().map(|l| l.unwrap());

    let mut deps: DiGraphMap<u32, bool> = DiGraphMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let nodes = line
            .split('|')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        deps.add_edge(nodes[1], nodes[0], false);
    }

    println!(
        "{}",
        lines
            .map(|line| get_bad_update_val(
                &mut line
                    .split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
                &mut deps
            ))
            .sum::<u32>()
    )
}

fn get_update_val(update: &[u32], deps: &mut DiGraphMap<u32, bool>) -> u32 {
    let mut visited: HashSet<u32> = HashSet::new();

    for val in update.iter() {
        visited.insert(*val);
        // satisfy dependencies where dependency is not already satisfied and we've seen the
        // dependency
        for (from, to) in deps
            .edges(*val)
            .filter_map(|(v, neighbor, edge)| {
                if !edge && visited.contains(&neighbor) {
                    Some((v, neighbor))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
        {
            *deps.edge_weight_mut(from, to).unwrap() = true;
        }
    }

    if update.iter().any(|val| {
        deps.edges(*val)
            .any(|(_, neighbor, edge)| !edge && visited.contains(&neighbor))
    }) {
        0
    } else {
        update[update.len() / 2]
    }
}

fn get_bad_update_val(update: &mut [u32], deps: &mut DiGraphMap<u32, bool>) -> u32 {
    let mut visited: HashMap<u32, usize> = HashMap::new();

    for (i, val) in update.iter().enumerate() {
        visited.insert(*val, i);
        // satisfy dependencies where dependency is not already satisfied and we've seen the
        // dependency
        for (from, to) in deps
            .edges(*val)
            .filter_map(|(v, neighbor, edge)| {
                if !edge && visited.contains_key(&neighbor) {
                    Some((v, neighbor))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
        {
            *deps.edge_weight_mut(from, to).unwrap() = true;
        }
    }

    for i in 0..update.len() {
        for dep_idx in
            deps.edges(update[i]).filter_map(
                |(_, neighbor, edge)| {
                    if !edge {
                        visited.get(&neighbor)
                    } else {
                        None
                    }
                },
            )
        {
            update.swap(i, *dep_idx);
            deps.all_edges_mut().for_each(|(_, _, edge)| *edge = false);
            return get_fixed_update_val(update, deps);
        }
    }

    0
}

fn get_fixed_update_val(update: &mut [u32], deps: &mut DiGraphMap<u32, bool>) -> u32 {
    let mut visited: HashMap<u32, usize> = HashMap::new();

    for (i, val) in update.iter().enumerate() {
        visited.insert(*val, i);
        // satisfy dependencies where dependency is not already satisfied and we've seen the
        // dependency
        for (from, to) in deps
            .edges(*val)
            .filter_map(|(v, neighbor, edge)| {
                if !edge && visited.contains_key(&neighbor) {
                    Some((v, neighbor))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
        {
            *deps.edge_weight_mut(from, to).unwrap() = true;
        }
    }

    for i in 0..update.len() {
        for dep_idx in
            deps.edges(update[i]).filter_map(
                |(_, neighbor, edge)| {
                    if !edge {
                        visited.get(&neighbor)
                    } else {
                        None
                    }
                },
            )
        {
            update.swap(i, *dep_idx);
            deps.all_edges_mut().for_each(|(_, _, edge)| *edge = false);
            return get_fixed_update_val(update, deps);
        }
    }

    deps.all_edges_mut().for_each(|(_, _, edge)| *edge = false);
    update[update.len() / 2]
}
