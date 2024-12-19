use std::collections::{HashMap, HashSet};

use aoc_tools::input::input::*;

fn num_pattern_combinations(pattern: &str, max_pat_len: usize, towels: &HashSet<String>, cache: &mut HashMap<String, i64>) -> i64 {
    if pattern.len() == 0 {
        return 1
    }
    if let Some(num_possible) = cache.get(pattern) {
        return *num_possible
    }
    let max_it = if max_pat_len > pattern.len() {pattern.len()} else {max_pat_len};
    let mut num_possible = 0;
    for p_len in 1..max_it+1 {
        if !towels.contains(&pattern[..p_len]) {
            continue;
        }
        num_possible +=  num_pattern_combinations(&pattern[p_len..], max_pat_len, towels, cache)
    }
    cache.insert(pattern.to_string(), num_possible);
    num_possible
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let mut max_pat_len = 0;
    let towels: HashSet<String> = input[0].split(", ").map(|s| {
        let pat = s.to_string();
        if pat.len() > max_pat_len {max_pat_len = pat.len()}
        pat
    }).collect();
    let mut cache: HashMap<String, i64> = HashMap::new();
    let pat_tots: Vec<i64> = input[1..].iter()
        .map(|pat| {
            num_pattern_combinations(pat, max_pat_len, &towels, &mut cache)
        })
        .collect();
    println!("part1: {}", pat_tots.iter().filter(|n| n > &&0).count());
    println!("part2: {}", pat_tots.iter().sum::<i64>());
}
