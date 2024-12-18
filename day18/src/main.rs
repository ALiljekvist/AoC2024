use std::collections::{HashMap, HashSet};

use aoc_tools::input::input::*;

// Find the 4-neighborhood of all positions with coordinates in the range [0, area_size]
fn create_steps(area_size: i64) -> HashMap<(i64, i64), Vec<(i64, i64)>> {
    let mut neighbs: HashMap<(i64, i64),Vec<(i64, i64)>> = HashMap::new();
    for x in 0..area_size+1 {
        for y in 0..area_size+1 {
            let mut neigh: Vec<(i64, i64)> = Vec::new();
            if x > 0 {
                neigh.push((x-1, y));
            }
            if x < area_size {
                neigh.push((x+1, y));
            }
            if y > 0 {
                neigh.push((x, y-1));
            }
            if y < area_size {
                neigh.push((x, y+1));
            }
            neighbs.insert((x, y), neigh);
        }
    }
    neighbs
}

// Function to use a regular vec as a priority queue
fn instert_prio(nodes: &mut Vec<(i64, (i64, i64))>, node: (i64, (i64, i64))) {
    let mut i = 0;
    while i < nodes.len() && node.0 < nodes[i].0 {
        i += 1
    }
    nodes.insert(i, node);
}
fn shortest_path(area_size: i64, byte_slice: &[(i64, i64)]) -> i64 {
    // Create a set of all the bytes
    let bytes: HashSet<(i64, i64)> = byte_slice.iter()
        .map(|p| *p)
        .collect();
    // Create the mapping of the 4-neighborhood of all positions
    let neighbs = create_steps(area_size);
    // Setup the priority queue and set of already visited positions
    let mut queue: Vec<(i64, (i64, i64))> = vec![(0, (0,0))];
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    while let Some((d, pos)) = queue.pop() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        if let Some(neighbs) = neighbs.get(&pos) {
            for new_pos in neighbs.iter() {
                // Found the final position
                if new_pos == &(area_size, area_size) {
                    return d + 1
                }
                // Can't wak into a bytes or have already visited
                if bytes.contains(new_pos) || visited.contains(&new_pos) {
                    continue;
                }
                // Free to add to queue
                instert_prio(&mut queue, (d+1, *new_pos));
            }
        }
    }
    let last_coord = byte_slice[byte_slice.len()-1];
    println!("part2: {},{}", last_coord.0, last_coord.1);
    -1
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let all_bytes: Vec<(i64, i64)> = input.iter()
        .map(|s| {
            let coords = ints_from_str(s);
            (coords[0], coords[1])
        })
        .collect();
    let mut kb_size = 1024;
    let area = 70;
    println!("part1: {}", shortest_path(area, &all_bytes[..kb_size]));
    while kb_size < all_bytes.len() {
        kb_size += 1;
        let dist = shortest_path(area, &all_bytes[..kb_size]);
        if dist == -1 {
            break
        }
    }
}
