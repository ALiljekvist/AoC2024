use std::collections::{HashMap, HashSet, VecDeque};

use aoc_tools::input::input::read_lines;

// BFS from end to find all points available on the map
fn map_all_distances_from_end(map: &HashSet<(i32, i32)>, end: (i32, i32)) -> HashMap<(i32, i32), i64> {
    let mut remaining: HashMap<(i32, i32), i64> = HashMap::new();
    let mut queue: VecDeque<(i64, (i32, i32))> = VecDeque::new();
    queue.push_front((0, end));

    while let Some((d, pos)) = queue.pop_back() {
        if remaining.contains_key(&pos) {
            continue;
        }
        remaining.insert(pos, d);
        for n in [(pos.0-1,pos.1), (pos.0+1,pos.1), (pos.0,pos.1-1), (pos.0,pos.1+1)] {
            if map.contains(&n) || remaining.contains_key(&n) {
                continue;
            }
            queue.push_front((d+1, n));
        }
    }

    remaining
}

fn find_cheats(steps_left: &HashMap<(i32, i32), i64>, pos: (i32, i32), max_len: i32, min_improv: i64) -> i64 {
    let curr_left = steps_left.get(&pos).unwrap();
    let mut cheat_count = 0;
    for r in pos.0-max_len..pos.0+max_len+1 {
        for c in pos.1-max_len..pos.1+max_len+1 {
            let walking = ((pos.0-r).abs() + (pos.1-c).abs()) as i64;
            if walking > max_len as i64 {
                continue;
            }
            if let Some(d_left) = steps_left.get(&(r, c))  {
                if d_left + walking <= curr_left - min_improv {
                    cheat_count += 1;
                }
            }
        }
    }
    cheat_count
}

fn find_num_cheats(steps_left: &HashMap<(i32, i32), i64>, start: (i32, i32), end: (i32, i32), max_cheat_dist: i32) -> i64 {
    let mut cheat_count = 0;
    let mut p = start;
    while p != end {
        let left = steps_left.get(&p).unwrap();
        cheat_count += find_cheats(steps_left, p, max_cheat_dist, 100);
        for n in [(p.0+1,p.1),(p.0-1,p.1),(p.0,p.1+1),(p.0,p.1-1),].iter() {
            if let Some(n_left) = steps_left.get(&n) {
                if *n_left == left-1 {
                    p = *n;
                    break
                }
            }
        }
    }
    cheat_count
}

fn main() {
    let (mut start, mut end) = ((0, 0), (0, 0));
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    for (r, row) in read_lines::<String>("input.txt")
        .unwrap()
        .into_iter()
        .enumerate() {
        for (c, ch) in row.chars().enumerate() {
            match ch {
                'S' => {start = (r as i32, c as i32)}
                'E' => {end = (r as i32, c as i32)}
                '#' => {map.insert((r as i32, c as i32));}
                _ => {}
            }
        }
    }
    let paths = map_all_distances_from_end(&map, end);
    println!("part1: {}", find_num_cheats(&paths, start, end, 2));
    println!("part2: {}", find_num_cheats(&paths, start, end, 20));
}
