use std::iter::zip;
use std::collections::HashMap;

use aoc_tools::input::input::read_lines;

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    // Set up data structures and parse the input into them
    let mut l1: Vec::<i64> = Vec::new();
    let mut l2: Vec::<i64> = Vec::new();
    let mut c1: HashMap<i64, i64> = HashMap::new();
    let mut c2: HashMap<i64, i64> = HashMap::new();
    for l in input {
        let nums: Vec<i64> = l.split("   ").into_iter().map(|x| x.parse().unwrap()).collect();
        // Add the numbers to the vectors
        l1.push(nums[0]);
        l2.push(nums[1]);
        // Add to each number count
        let count1 = c1.entry(nums[0]).or_default();
        *count1 += 1;
        let count2 = c2.entry(nums[1]).or_default();
        *count2 += 1;
    }
    // Order the values
    l1.sort();
    l2.sort();
    // Calculate the values
    println!("part1: {}", zip(l1, l2).map(|(x, y)| (x-y).abs()).sum::<i64>());
    println!("part2: {}", c1.iter().map(|(num, count)| count * num * (*c2.entry(*num).or_default())).sum::<i64>());
}
