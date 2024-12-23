use std::collections::{HashMap, HashSet};

use aoc_tools::input::input::read_lines;

fn combinations(conns: &Vec<String>, size: usize) -> Vec<Vec<String>> {
    let mut combs = Vec::new();
    pick(&mut combs, conns, &mut Vec::new(), size, 0);
    combs
}

fn pick(combs: &mut Vec<Vec<String>>, conns: &Vec<String>, curr: &mut Vec<String>, size: usize, ind: usize) {
    if curr.len() == size {
        combs.push(curr.clone());
    }
    for i in ind..conns.len() {
        curr.push(conns[i].clone());
        pick(combs, conns, curr, size, i+1);
        curr.pop();
    }
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let mut network: HashMap<String, Vec<String>> = HashMap::new();
    for conn in input.iter() {
        let parts:Vec<&str> = conn.split("-").collect();
        network.entry(parts[0].to_string()).or_default().push(parts[1].to_string());
        network.entry(parts[1].to_string()).or_default().push(parts[0].to_string());
    }
    let mut groups: HashSet<Vec<&String>> = HashSet::new();
    for (comp, conns) in network.iter() {
        if comp.chars().nth(0) != Some('t') {
            continue;
        }
        for i in 0..conns.len() {
            for j in i+1..conns.len() {
                if !network.get(&conns[i]).unwrap().contains(&conns[j]) {
                    continue;
                }
                if !network.get(&conns[j]).unwrap().contains(&conns[i]) {
                    continue;
                }
                let mut group = Vec::from([comp, &conns[i], &conns[j]]);
                group.sort();
                groups.insert(group);
            }
        }
    }
    println!("part1: {}", groups.len());
    let mut largest_lan: Vec<String> = Vec::new();
    for (_comp, conns) in network.iter() {
        // Assume there is only one largest group
        // Therefore the group must be larger than 4
        let mut size = largest_lan.len();
        if size < 3 {size = 3}
        let mut combs = combinations(&conns, size);
        while combs.iter().any(|lan| {
            for i in 0..lan.len() {
                for j in i+1..lan.len() {
                    if !network.get(&conns[i]).unwrap().contains(&conns[j]) {
                        return false
                    }
                }
            }
            largest_lan = lan.clone();
            largest_lan.insert(0, _comp.clone());
            largest_lan.sort();
            true
        }) {
            size += 1;
            combs = combinations(&conns, size);
        }
    }
    println!("part2: {}", largest_lan.join(","));
}
