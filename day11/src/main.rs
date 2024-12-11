use std::{collections::HashMap, fs::read_to_string};

fn num_digits(num: &i64) -> u32 {
    let mut digits = 1;
    while *num >= 10i64.pow(digits) {
        digits += 1;
    }
    return digits
}

fn blink(stone: i64, depth: usize, max_depth: usize, cache: &mut HashMap<(i64, usize), i64>) -> i64 {
    if depth >= max_depth {
        return 1
    }
    if let Some(cached_val) = cache.get(&(stone, max_depth-depth)) {
        return *cached_val
    }
    if stone == 0 {
        let tot_under = blink(1, depth+1, max_depth, cache);
        cache.insert((stone, max_depth-depth), tot_under);
        return tot_under
    }
    let num_len = num_digits(&stone);
    if num_len % 2 == 0 {
        let power_split = 10i64.pow(num_len/2);
        let second_stone = stone % power_split;
        let first_stone = (stone - second_stone) / power_split;
        let tot_under = blink(first_stone, depth+1, max_depth, cache) +
            blink(second_stone, depth+1, max_depth, cache);
        cache.insert((stone, max_depth-depth), tot_under);
        return tot_under
    }
    let tot_under = blink(stone * 2024, depth+1, max_depth, cache);
    cache.insert((stone, max_depth-depth), tot_under);
    tot_under
}

fn main() {
    let stones: Vec<i64> = read_to_string("input.txt").unwrap()
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut stone_cache: HashMap<(i64, usize), i64> = HashMap::new();
    let p1 = stones.iter()
        .map(|s| blink(*s, 0, 25, &mut stone_cache))
        .sum::<i64>();
    println!("part1: {}", p1);
    let p2 = stones.iter()
        .map(|s| blink(*s, 0, 75, &mut stone_cache))
        .sum::<i64>();
    println!("part2: {}", p2);
}
