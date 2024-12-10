use std::collections::{HashMap, HashSet};

use aoc_tools::input::input::read_lines;

fn four_neigh(pos: (i32, i32)) -> Vec<(i32, i32)> {
    return Vec::from([(1, 0), (-1, 0), (0, 1), (0, -1)])
        .into_iter()
        .map(|delta| (pos.0 + delta.0, pos.1 + delta.1))
        .collect()
}

fn search_trail_paths(start: (i32, i32), trail_map: &HashMap<(i32, i32), u8>) -> (i32, i32) {
    let mut trail_ends: HashSet<(i32, i32)> = HashSet::new();
    let mut nbr_trails = 0i32;
    let mut to_visit: Vec<(i32, i32)> = Vec::from([start]);
    while to_visit.len() > 0 {
        let pos = to_visit.pop().unwrap();
        if trail_map.get(&pos).unwrap() == &9 {
            nbr_trails += 1;
            trail_ends.insert(pos);
            continue;
        }
        for next_pos in four_neigh(pos) {
            if !trail_map.contains_key(&next_pos) {
                continue;
            }
            if *trail_map.get(&next_pos).unwrap() != *trail_map.get(&pos).unwrap() + 1 {
                continue;
            }
            to_visit.push(next_pos);
        }
    }
    (trail_ends.len() as i32, nbr_trails)
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let mut trail_map: HashMap<(i32, i32), u8> = HashMap::new();
    for (r, row) in input.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            trail_map.insert((r as i32, c as i32), ch.to_string().parse().unwrap());
        }
    }
    let parts: Vec<(i32, i32)> = trail_map.iter()
        .filter_map(|x| {
            if *x.1 == 0 {Some(search_trail_paths(*x.0, &trail_map))} else {None}
        })
        .collect();
    let p1 = parts.iter().map(|x| x.0).sum::<i32>();
    println!("part1: {}", p1);
    let p2 = parts.iter().map(|x| x.1).sum::<i32>();
    println!("part2: {}", p2);
}
