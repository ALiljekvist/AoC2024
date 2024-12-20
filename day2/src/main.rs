use aoc_tools::input::input::read_lines;
use std::iter::zip;

fn is_safe(levels: &Vec<i64>) -> bool {
    let diffs: Vec<i64> = zip(&levels[0..], &levels[1..]).map(|(x, y)| y-x).collect();
    if diffs.iter().map(|f| f.signum()).sum::<i64>().abs() != diffs.len() as i64 {
        return false;
    }
    if diffs.iter().filter(|x| x.abs() > 3).count() > 0 {
        return false;
    }
    true
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let nums: Vec<Vec<i64>> = input.iter()
                                    .map(|s| s.split(" ")
                                                        .map(|x| x.parse().unwrap())
                                                        .collect())
                                    .collect();
    let nbr_safe = nums.iter().filter(|levels| is_safe(levels)).count();
    println!("part1: {}", nbr_safe);
    let nbr_safe2 = nums.iter().filter(|levels|
        (0..levels.len()).any(|i| is_safe(&levels[..i].iter().chain(levels[i+1..].iter()).cloned().collect()))
    ).count();
    println!("part2: {}", nbr_safe2);
}
