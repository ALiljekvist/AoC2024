use aoc_tools::input::input::read_lines;
use std::iter::zip;

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let nums: Vec<Vec<i64>> = input.iter()
                                    .map(|s| s.split(" ")
                                                        .map(|x| x.parse().unwrap())
                                                        .collect())
                                    .collect();
    let mut nbr_safe = 0;
    for levels in nums.iter() {
        let diffs: Vec<i64> = zip(&levels[0..], &levels[1..]).map(|(x, y)| y-x).collect();
        if diffs.iter().map(|f| f.signum()).sum::<i64>().abs() != diffs.len() as i64 {
            continue
        }
        if diffs.iter().filter(|x| x.abs() < 1 || x.abs() > 3).count() > 0 {
            continue;
        }
        nbr_safe += 1 ;
    }
    println!("part1: {}", nbr_safe);
    let mut nbr_safe2 = 0;
    for levels in nums {
        let mut is_safe = false;
        for i in 0..levels.len() {
            let new_levels: Vec<i64> = levels.iter().enumerate()
                .filter(|(i2, _)| *i2 != i)
                .map(|(_, x)| *x).collect();
            let diffs: Vec<i64> = zip(&new_levels[0..], &new_levels[1..]).map(|(x, y)| y-x).collect();
            if diffs.iter().map(|f| f.signum()).sum::<i64>().abs() != diffs.len() as i64 {
                continue
            }
            if diffs.iter().filter(|x| x.abs() < 1 || x.abs() > 3).count() > 0 {
                continue;
            }
            is_safe = true;
            break
        }
        if is_safe {
            nbr_safe2 += 1;
        }
    }
    println!("part2: {}", nbr_safe2);
}
