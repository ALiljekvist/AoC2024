use std::{char, collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Submarine {
    r: i32,
    c: i32,
}

impl Submarine {
    fn go(&mut self, delta: (i32, i32)) {
        self.r += delta.0;
        self.c += delta.1
    }
}

fn parse_map(inp: String) -> (Vec<Vec<char>>, Submarine) {
    let mut sub: Submarine = Submarine{r: 0, c: 0};
    let map: Vec<Vec<char>> = inp.split("\n")
        .filter(|s| !s.is_empty())
        .into_iter()
        .enumerate()
        .map(|(r, row)| row.chars()
            .enumerate()
            .map(|(c, ch)| {
                if ch == '@' {
                    sub.r = r as i32;
                    sub.c = c as i32;
                    return '.'
                }
                ch
            })
            .collect())
        .collect();
    (map, sub)
}

fn parse_wide_map(inp: String) -> (Vec<Vec<char>>, Submarine) {
    let mut sub: Submarine = Submarine{r: 0, c: 0};
    let mut map: Vec<Vec<char>> = Vec::new();
    for (r, row) in inp.split("\n")
        .filter(|s| !s.is_empty())
        .into_iter()
        .enumerate() {
        let mut wide_row = vec!['.'; row.len()*2];
        for (c, ch) in row.chars().enumerate() {
            match ch {
                '.' => {
                    wide_row[c*2] = '.';
                    wide_row[c*2+1] = '.';
                }
                '#' => {
                    wide_row[c*2] = '#';
                    wide_row[c*2+1] = '#';
                }
                'O' => {
                    wide_row[c*2] = '[';
                    wide_row[c*2+1] = ']';
                }
                '@' => {
                    wide_row[c*2] = '.';
                    wide_row[c*2+1] = '.';
                    sub.r = r as i32;
                    sub.c = 2*c as i32;
                }
                _ => {}
            }
        }
        map.push(wide_row);
    }
    (map, sub)
}

fn make_move(sub: &mut Submarine, map: Vec<Vec<char>>, delta: (i32, i32)) -> Vec<Vec<char>> {
    let mut new_map = map.clone();
    let mut curr_pos: HashSet<(i32, i32)> = HashSet::from([(sub.r, sub.c)]);
    while curr_pos.len() > 0 {
        let mut next_pos: HashSet<(i32, i32)> = HashSet::new();
        for pos in curr_pos.iter() {
            match map[(pos.0+delta.0) as usize][(pos.1+delta.1) as usize] {
                '#' => {
                    return map
                }
                'O' => {
                    next_pos.insert((pos.0+delta.0, pos.1+delta.1));
                }
                '[' => {
                    if delta.1 != 0 {
                        next_pos.insert((pos.0+delta.0, pos.1+delta.1));
                    } else {
                        next_pos.insert((pos.0+delta.0, pos.1+delta.1));
                        next_pos.insert((pos.0+delta.0, pos.1+delta.1+1));
                    }
                }
                ']' => {
                    if delta.1 != 0 {
                        next_pos.insert((pos.0+delta.0, pos.1+delta.1));
                    } else {
                        next_pos.insert((pos.0+delta.0, pos.1+delta.1));
                        next_pos.insert((pos.0+delta.0, pos.1+delta.1-1));
                    }
                }
                _ => {}
            }
        }
        for pos in next_pos.iter() {
            new_map[pos.0 as usize][pos.1 as usize] = '.';
        }
        for pos in curr_pos {
            new_map[(pos.0 + delta.0) as usize][(pos.1 + delta.1) as usize] = map[pos.0 as usize][pos.1 as usize];
        }
        // Update which positions should be examined next
        curr_pos = next_pos;
    }
    sub.go(delta);
    new_map
}

fn calc_gps_score(map: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            score += match map[r][c] {
                'O' | '[' => {r * 100 + c}
                _ => {0}
            };
        }
    }
    score
}

fn main() {
    let input: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let (mut map, mut sub) = parse_map(input[0].clone());
    let (mut wide_map, mut sub2) = parse_wide_map(input[0].clone());

    // Part 1
    for ch in input[1].chars() {
        let delta = match ch {
            '^' => {(-1, 0)}
            '>' => {(0, 1)}
            'v' => {(1, 0)}
            '<' => {(0, -1)}
            _ => {continue;}
        };
        map = make_move(&mut sub, map, delta);
        wide_map = make_move(&mut sub2, wide_map, delta);
    }
    println!("part1: {}", calc_gps_score(&map));
    println!("part2: {}", calc_gps_score(&wide_map));
}
