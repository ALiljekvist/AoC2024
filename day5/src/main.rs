use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn upgrade_in_order(upgrade: &Vec<i64>, rules: &HashMap<i64, HashSet<i64>>) -> bool {
    for i in 0..upgrade.len()-1 {
        let curr_page = upgrade[i];
        let remaining: HashSet<i64> = upgrade[i+1..].iter().map(|x| *x).collect();
        if !rules.contains_key(&curr_page) {
            break
        }
        if remaining.intersection(rules.get(&curr_page).unwrap()).count() > 0 {
            return false
        }
    }
    true
}

fn fix_upgrade(upgrade: &mut Vec<i64>, rules: &HashMap<i64, HashSet<i64>>) {
    for i in 0..upgrade.len() {
        let mut in_order = false;
        while !in_order {
            let curr_page = upgrade[i];
            let remaining: HashSet<i64> = upgrade[i+1..].iter().map(|x| *x).collect();
            if !rules.contains_key(&curr_page) {
                break
            }
            if remaining.intersection(rules.get(&curr_page).unwrap()).count() > 0 {
                // Place the current number in the back of the vector
                upgrade.remove(i);
                upgrade.push(curr_page);
                continue;
            }
            in_order = true;
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let sections: Vec<String> = input.split("\n\n")
        .filter_map(|s| if !s.is_empty() {Some(s.to_string())} else {None})
        .collect();
    // Parse rules into vec
    let rule_vec: Vec<Vec<i64>> = sections[0].split("\n")
        .filter_map(|s| if !s.is_empty() {
            Some(s.split("|").map(|x| x.parse().unwrap()).collect())
        } else {None})
        .collect();
    // Translate into HashMap in backwards order (the values need to be done before the key)
    let mut rules: HashMap<i64, HashSet<i64>> = HashMap::new();
    for rule_pair in rule_vec {
        let entry = rules.entry(rule_pair[1]).or_default();
        entry.insert(rule_pair[0]);
    }
    // Parse upgrades
    let upgrades: Vec<Vec<i64>> = sections[1].split("\n")
        .filter_map(|s| if !s.is_empty() {
            Some(s.split(",").map(|s| s.parse().unwrap()).collect())
        } else {None})
        .collect();
    let p1 = upgrades.iter()
        .filter(|upgrade| upgrade_in_order(upgrade, &rules))
        .map(|u| u[u.len()/2])
        .sum::<i64>();
    println!("part1: {}", p1);
    let p2 = upgrades.iter()
        .filter(|upgrade| !upgrade_in_order(upgrade, &rules))
        .map(|upgrade| {
            let mut new_upgrade = upgrade.clone();
            fix_upgrade(&mut new_upgrade, &rules);
            new_upgrade[new_upgrade.len()/2]
        })
        .sum::<i64>();
    println!("part2: {}", p2);
}