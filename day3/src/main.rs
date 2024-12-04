use std::{fs::read_to_string, iter::zip};

use regex::Regex;

fn do_mul(s: &str) -> i64 {
    let mul_re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    mul_re.captures_iter(s)
    .into_iter()
    .map(|c| {
        let (_, [num1, num2]) = c.extract();
        num1.parse::<i64>().unwrap() * num2.parse::<i64>().unwrap()
    })
    .sum::<i64>()
}

fn main() {
    let memory = read_to_string("input.txt").unwrap();
    let p1 = do_mul(&memory);
    println!("part1: {}", p1);
    // Setup a regex to find do's and dont's to be able to split the memory
    // into pieces of valid and non-valid muls
    let do_re = Regex::new(r"do\(\)|don't\(\)").unwrap();
    // Pad the memory with break points on each end to encapsulate the ends
    let padded_memory = format!("do(){}do()", &memory);
    let breaks: Vec<(usize, bool)> = do_re.captures_iter(&padded_memory)
        .into_iter()
        .map(|cap| {
            let m = cap.get(0).unwrap();
            (m.start(), match m.as_str() {
                "do()" => true,
                _ => false,
            })
        })
        .collect();
    let p2 = zip(&breaks[0..], &breaks[1..])
        .filter_map(|(b1, b2)| if b1.1 {Some(do_mul(&padded_memory[b1.0..b2.0]))} else {None})
        .sum::<i64>();
    println!("part2: {}", p2);
}
