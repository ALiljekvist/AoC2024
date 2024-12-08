use std::collections::{HashMap, HashSet};

use aoc_tools::input::input::read_lines;

fn create_antinodes(a: (i64, i64), b: (i64, i64)) -> ((i64, i64), (i64, i64)) {
    let (dr, dc) = ((a.0 - b.0), (a.1 - b.1));
    ((a.0 + dr, a.1 + dc), (b.0 - dr, b.1 - dc))
}

fn create_harmonic_antinodes(a: (i64, i64), b: (i64, i64), bounds: (i64, i64)) -> Vec<(i64, i64)> {
    let mut harm_ants: Vec<(i64, i64)> = Vec::new();
    let (dr, dc) = ((a.0 - b.0), (a.1 - b.1));
    let mut k = 0;
    while within((a.0 + dr * k, a.1 + dc * k), bounds) {
        harm_ants.push((a.0 + dr * k, a.1 + dc * k));
        k += 1;
    }
    k = 0;
    while within((b.0 - dr*k, b.1 - dc*k), bounds) {
        harm_ants.push((b.0 - dr*k, b.1 - dc*k));
        k += 1;
    }
    harm_ants
}


fn within(a: (i64, i64), bounds: (i64, i64)) -> bool {
    return a.0 >= 0 && a.0 < bounds.0 && a.1 >= 0 && a.1 < bounds.1
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for (i, row) in input.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            let entry = antennas.entry(ch).or_default();
            entry.push((i as i64, j as i64))
        }
    }
    let bounds: (i64, i64) = (input.len() as i64, input[0].len() as i64);
    let antinodes: HashMap<char, HashSet<(i64, i64)>> = antennas.iter()
        .map(|(k, ants)| {
            let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
            for i in 0..ants.len() {
                for j in i+1..ants.len() {
                    // Add both antinodes
                    let (n1, n2) = create_antinodes(ants[i], ants[j]);
                    antinodes.insert(n1);
                    antinodes.insert(n2);
                }
            }
            (*k, antinodes)
        })
        .collect();
    let mut all_antinodes: HashSet<(i64, i64)> = HashSet::new();
    for (_, ants) in antinodes.iter() {
        for node in ants {
            if !within(*node, bounds) {
                continue;
            }
            all_antinodes.insert(*node);
        }
    }
    println!("part1: {}", all_antinodes.len());
    let mut harmonic_antinodes: HashSet<(i64, i64)> = HashSet::new();
    for (_, ants) in antennas.iter() {
        for i in 0..ants.len() {
            for j in i+1..ants.len() {
                for nod in create_harmonic_antinodes(ants[i], ants[j], bounds) {
                    harmonic_antinodes.insert(nod);
                }
            }
        }
    }
    println!("part2: {}", harmonic_antinodes.len());
}
