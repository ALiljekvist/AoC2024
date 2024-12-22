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
    // Part 1
    let p1 = buyers.iter()
        .enumerate()
        .map(|(monkey, starting_secret)| {
            let mut secret = *starting_secret;
            sequences[monkey].push(secret%10);
            for _ in 0..2000 {
                secret = evolve(secret);
                sequences[monkey].push(secret%10);
            }
            secret
        }).sum::<i64>();
    println!("part1: {}", p1);

    // Part 2
    let mut patterns: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    for seq in sequences {
        let mut visited: HashSet<(i64, i64, i64, i64)> = HashSet::new();
        for w in seq.windows(5) {
            let key = (w[1]-w[0], w[2]-w[1], w[3]-w[2], w[4]-w[3]);
            if visited.contains(&key) {
                continue;
            }
            let sold = patterns.entry(key).or_default();
            visited.insert(key);
            *sold += w[4];
        }
    }
    let p2 = patterns.values().max().unwrap();
    println!("part2: {}", p2);
}
