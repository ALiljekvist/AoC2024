use std::collections::{HashMap, HashSet};

use aoc_tools::input::input::read_lines;

fn four_neigh(pos: &(i32, i32)) -> Vec<(i32, i32)> {
    return [(0,1), (1,0), (0,-1), (-1,0)].into_iter().map(|x| (pos.0+x.0, pos.1+x.1)).collect()
}

fn find_area(plant_type: char, start: (i32, i32), plants: &HashMap<(i32, i32), char>) -> Vec<(i32, i32)> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut to_visit: Vec<(i32, i32)> = Vec::from([start]);
    let mut area: Vec<(i32, i32)> = Vec::new();
    while to_visit.len() > 0 {
        let pos = to_visit.pop().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        area.push(pos);
        for neigh in four_neigh(&pos) {
            if visited.contains(&neigh) {
                continue;
            }
            if plants.contains_key(&neigh) && plants.get(&neigh).unwrap() == &plant_type {
                to_visit.push(neigh);
            }
        }
    }
    area
}

fn perimeter(plants: &Vec<(i32, i32)>) -> (usize, usize) {
    let p1 = plants[0];
    let (mut min_r, mut max_r, mut min_c, mut max_c) = (p1.0, p1.0, p1.1, p1.1);
    let mut plant_set: HashSet<(i32, i32)> = HashSet::new();
    for (r, c) in plants.iter() {
        if *r < min_r {
            min_r = *r
        }
        if *r > max_r {
            max_r = *r
        }
        if *c < min_c {
            min_c = *c
        }
        if *c > max_c {
            max_c = *c
        }
        plant_set.insert((*r, *c));
    }
    let mut fence_count = 0;
    let mut side_count = 0;
    let mut left = false;
    let mut right = false;
    for r in min_r-1..max_r+2 {
        for c in min_c-1..max_c+2 {
            if plant_set.contains(&(r, c)) {
                left = false;
                right = false;
                continue;
            }
            if plant_set.contains(&(r-1, c)) {
                if !left {
                    side_count += 1;
                }
                fence_count +=1;
                left = true;
            } else {
                left = false;
            }
            if plant_set.contains(&(r+1, c)) {
                if !right {
                    side_count += 1;
                }
                fence_count +=1;
                right = true;
            } else {
                right = false;
            }
        }
    }
    for c in min_c-1..max_c+2 {
        for r in min_r-1..max_r+2 {
            if plant_set.contains(&(r, c)) {
                left = false;
                right = false;
                continue;
            }
            if plant_set.contains(&(r, c+1)) {
                if !left {
                    side_count += 1;
                }
                fence_count +=1;
                left = true;
            } else {
                left = false;
            }
            if plant_set.contains(&(r, c-1)) {
                if !right {
                    side_count += 1;
                }
                fence_count +=1;
                right = true;
            } else {
                right = false;
            }
        }
    }
    (fence_count, side_count)
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let mut plants: HashMap<(i32, i32), char> = HashMap::new();
    for (r, row) in input.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            plants.insert((r as i32, c as i32), ch);
        }
    }
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut areas: Vec<(char, Vec<(i32, i32)>)> = Vec::new();
    for (pos, plant_type) in plants.iter() {
        if visited.contains(pos) {
            continue;
        }
        let mut area = find_area(*plant_type, *pos, &plants);
        for p in area.iter() {
            visited.insert(*p);
        }
        area.sort_unstable_by_key(|x| (x.0, x.1));
        areas.push((*plant_type, area));
    }
    let mut p1 = 0;
    let mut p2 = 0;
    for (_, positions) in areas.iter_mut() {
        let (fence, sides) = perimeter(positions);
        p1 += positions.len() * fence;
        p2 += positions.len() * sides;
    }

    println!("part1: {}", p1);
    println!("part2: {}", p2);
}
