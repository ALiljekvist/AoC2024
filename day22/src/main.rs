use std::{collections::{HashMap, HashSet}, iter::zip};

use aoc_tools::input::input::read_lines;

fn mix(a: i64, b: i64) -> i64 {
    return a ^ b
}

fn prune(a: i64) -> i64 {
    return a % 16777216
}

fn evolve(secret: i64) -> i64 {
    let mut new_secret = prune(mix(secret, secret * 64));
    new_secret = prune(mix(new_secret, new_secret / 32));
    new_secret = prune(mix(new_secret, new_secret * 2048));
    new_secret
}

fn main() {
    let buyers = read_lines::<i64>("input.txt").unwrap();
    let mut sequences: Vec<Vec<i64>> = vec![Vec::new(); buyers.len()];
    let p1 = buyers.iter()
        .enumerate()
        .map(|(monkey, starting_secret)| {
            let mut secret = *starting_secret;
            for _ in 0..2000 {
                secret = evolve(secret);
                sequences[monkey].push(secret%10);
            }
            secret
        }).sum::<i64>();
    println!("part1: {}", p1);
    let diffs: Vec<Vec<i64>> = sequences.iter()
        .map(|seq| zip(&seq[0..], &seq[1..]).map(|(a,b)| b-a).collect())
        .collect();
    let mut possible: HashMap<&[i64], i64> = HashMap::new();
    for (i, diff) in diffs.iter().enumerate() {
        let mut visited: HashSet<&[i64]> = HashSet::new();
        for (j, w) in diff.windows(4).enumerate() {
            if j >= sequences[i].len()-4 {
                break
            }
            if visited.contains(w) {
                continue;
            }
            let sold = possible.entry(w).or_default();
            *sold += sequences[i][j+4];
            visited.insert(w);
        }
    }
    let p2 = possible.values().max().unwrap();
    println!("part2: {}", p2);
}
