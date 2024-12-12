use std::collections::{HashMap, HashSet};

use aoc_tools::input::input::read_lines;

// Takes 
fn find_perimeter(plants: &Vec<(i32, i32)>) -> (Vec<(i32, i32)>, i32) {
    let plant_set: HashSet<(i32, i32)> = plants.iter().map(|s| *s).collect();
    let mut perimeter: Vec<(i32, i32)> = Vec::new();
    let mut num_sides = 0;
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    let start = ((plants[0].0, plants[0].1), 0);
    let mut pos = start.0;
    let mut dir = start.1;
    // let mut first_step = true;
    while !visited.contains(&(pos.0, pos.1, dir)) {
        visited.insert((pos.0, pos.1, dir));
        // println!("{:?}, {}", pos, dir);
        let forward = step(pos, dir);
        let left = step(pos, turn(dir, false));
        if plant_set.contains(&left) {
            dir = turn(dir, false);
            pos = step(pos, dir);
            num_sides += 1;
            continue;
        }
        if !plant_set.contains(&forward) {
            perimeter.push(left);
            dir = turn(dir, true);
            num_sides += 1;
            continue;
        }
        perimeter.push(left);
        pos = forward;
    }
    (perimeter, num_sides)
}

fn step(pos: (i32, i32), dir: i32) -> (i32, i32) {
    match dir {
        0 => {(pos.0, pos.1+1)}
        1 => {(pos.0+1, pos.1)}
        2 => {(pos.0, pos.1-1)}
        3 => {(pos.0-1, pos.1)}
        _ => {panic!("invalid dir {}", dir)}
    }
}

fn turn(dir: i32, right: bool) -> i32 {
    if right {
        return (dir + 1) % 4
    }
    let mut new_dir = dir - 1;
    if new_dir < 0 {
        new_dir += 4
    }
    new_dir
}

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

// fn count_sides(perimeter: &mut Vec<(i32, i32)>) -> i32 {
//     let mut num_sides = 0;
//     while perimeter.len() > 0 {
//         let mut 
//     }
//     num_sides
// }

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
    let mut perimeters: Vec<(char, Vec<(i32, i32)>)> = Vec::new();
    for (plant_type, positions) in areas.iter_mut() {
        let (mut perim, _num_sides) = find_perimeter(&positions);
        let mut perimeter: Vec<(i32, i32)> = Vec::new();
        for pos in positions.iter() {
            for delta in [(1,0), (-1,0), (0,1), (0,-1)].into_iter() {
                let n_pos = (pos.0 + &delta.0, pos.1 +&delta.1);
                if let Some(plant) = plants.get(&n_pos) {
                    if plant == plant_type {
                        continue;
                    }
                }
                perimeter.push(n_pos);
            }
        }
        perimeter.sort_unstable_by_key(|x| (x.0, x.1));
        perim.sort_unstable_by_key(|x| (x.0, x.1));
        if perim.len() == 0 {
            println!("no perim");
            return;
        }
        // for i in 0..perim.len() {
            // if i > perimeter.len() || perim[i] != perimeter[i] {
            //     println!("{}", plant_type);
            //     println!("{:?} {}", perim, perim.len());
            //     println!("{:?} {}", perimeter, perimeter.len());
            //     // println!("mismatch {} != {}", )
            //     return
            // }
        // }
        perimeters.push((*plant_type, perimeter));
        p1 += positions.len() * perim.len();
        p2 += positions.len() * (_num_sides as usize);
    }
    // let mut p1 = 0;
    // let mut p2 = 0;
    // for (i, (_, l)) in areas.iter().enumerate() {
    //     let mut p = perimeters[i].1.clone();
    //     p.sort_unstable_by_key(|x| (x.0, x.1));

    //     p1 += l.len() * p.len();
    // }

    println!("part1: {}", p1);
    println!("part2: {}", p2);
}
