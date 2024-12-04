use std::collections::HashMap;

use aoc_tools::input::input::read_lines;

fn count_xmas(r: i32, c: i32, s: &HashMap<(i32, i32), char>) -> i64 {
    match s.get(&(r, c)) {
        Some('X') => {}
        _ => {return 0}
    }
    let mut count = 0i64;
    for dr in -1..2i32 {
        for dc in -1..2i32 {
            let new_word: String = (0..4).filter_map(|i| s.get(&(r+dr*i, c+dc*i))).collect();
            if new_word == "XMAS" {
                count += 1;
            }
        }
    }
    count
}

fn count_mas_x(r: i32, c: i32, s: &HashMap<(i32, i32), char>) -> i64 {
    match s.get(&(r, c)) {
        Some('A') => {}
        _ => {return 0}
    }
    
    let w1: String = (-1..2).filter_map(|i| s.get(&(r+i, c-i))).collect();
    let w2: String = (-1..2).filter_map(|i| s.get(&(r+i, c+i))).collect();
    if (w1 == "MAS" || w1 == "SAM") && (w2 == "MAS" || w2 == "SAM") {
        return 1
    }
    0
}

fn main() {
    let char_mat: Vec<Vec<char>> = read_lines::<String>("input.txt")
        .unwrap()
        .iter()
        .map(|c| c.chars().collect())
        .collect();
    // Make it a HashMap instead to avoid having to deal with borders
    let mut char_map: HashMap<(i32, i32), char> = HashMap::new();
    for (row_num, row) in char_mat.iter().enumerate() {
        for (col_num, ch) in row.iter().enumerate() {
            char_map.insert((row_num as i32, col_num as i32), *ch);
        }
    }
    let p1 = char_map.iter()
        .map(|((r, c), _) | count_xmas(*r, *c, &char_map))
        .sum::<i64>();
    println!("part1: {}", p1);
    let p2 = char_map.iter()
        .map(|((r, c), _) | count_mas_x(*r, *c, &char_map))
        .sum::<i64>();
    println!("part2: {}", p2);
}
