use std::{collections::HashMap, fs::read_to_string};

fn parse_input(input: String) -> (HashMap<String, u8>, Vec<Vec<String>>) {
    let parts: Vec<String> = input.split("\n\n")
        .filter_map(|s| if !s.is_empty() {Some(s.to_string())} else {None})
        .collect();
    let gates: HashMap<String, u8> = parts[0].split("\n")
        .filter(|s| !s.is_empty())
        .map(|g| {
            let gate_val: Vec<&str> = g.split(": ").collect();
            (gate_val[0].to_string(), gate_val[1].parse().unwrap())
        })
        .collect();
    let intstructions: Vec<Vec<String>> = parts[1].split("\n")
        .filter(|s| !s.is_empty())
        .map(|ins| ins.split(" ").map(|s| s.to_string()).collect())
        .collect();
    (gates, intstructions)
}

fn get_rev_map(ops: &Vec<Vec<String>>) -> HashMap<String, (String, String, String)> {
    ops.iter()
        .map(|ps| {
            (ps[4].clone(), (ps[1].clone(), ps[0].clone(), ps[2].clone()))
        })
        .collect()
}

fn from_binary(a: Vec<u8>) -> u64 {
    return a.iter().enumerate().map(|(i, v)| *v as u64 * 2u64.pow(i as u32)).sum::<u64>();
}

fn calc_gate(
    gate: &String,
    rev_map: &HashMap<String, (String, String, String)>,
    gates: &HashMap<String, u8>,
    level: usize,
) -> u8 {
    if level > 300 {
        return 7
    }
    if let Some(val) = gates.get(gate) {
        return *val
    }
    let (op, dep1, dep2) = match rev_map.get(gate) {
        Some(ent) => {(&ent.0, &ent.1, &ent.2)}
        None => {return 7}
    };
    let a = calc_gate(dep1, rev_map, gates, level + 1);
    let b = calc_gate(dep2, rev_map, gates, level + 1);
    match op.as_str() {
        "AND" => {
            return if a + b == 2 {1} else {0};
        }
        "OR" => {
            return if a + b > 0 {1} else {0};
        }
        "XOR" => {
            return if a != b {1} else {0};
        }
        _ => {panic!("Wrong operation {}", op)}
    };
}

fn find_input_count(gate: &String, rev_map: &HashMap<String, (String, String, String)>) -> (u8, u8, u8) {
    let (mut a, mut x, mut o) = (0, 0, 0);
    for (_, (op, d1, d2)) in rev_map.iter() {
        if gate == d1 || gate == d2 {
            match op.as_str() {
                "AND" => {a += 1}
                "XOR" => {x += 1}
                "OR" => {o += 1}
                _ => {}
            }
        }
    }
    (a, x, o)
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (gates, ops) = parse_input(input);
    // Part 1
    let all_gates: Vec<String> = ops.iter().map(|op| op[4].clone()).collect();
    let rev_map = get_rev_map(&ops);
    let mut z_gates: Vec<String> = all_gates.iter()
        .filter_map(|s| if s.chars().nth(0).unwrap() == 'z' {Some(s.clone())} else {None})
        .collect();
    z_gates.sort();
    let finish: Vec<u8> = z_gates.iter().map(|gate| calc_gate(gate, &rev_map, &gates, 0)).collect();
    println!("part1: {}", from_binary(finish));

    // Part 2:
    // Realize that the input of earlier z-gates affect the later z-gates, which means
    // they should follow a pattern. Seems to be logic adders with a carry-over from
    // previous gates.

    let mut sus: Vec<String> = Vec::new();
    for (gate, (op, a, b)) in rev_map.iter() {
        if gate == &z_gates[0] {
            // Manually check first output
            if op != "XOR" && (a != "x00" || a != "y00") && (b != "x00" || b != "y00") {
                sus.push(gate.clone());
            }
            continue;
        }
        if gate == &z_gates[z_gates.len()-1] {
            // Manually check last output
            if op == "OR" {
                continue;
            }
            let input_count = find_input_count(gate, &rev_map);
            if input_count.0 + input_count.1 + input_count.2 != 0 {
                sus.push(gate.clone());
            }
            continue;
        }
        // Check if it is an output, if so, the op should be "XOR" and a+b should be mapped
        if gate.chars().nth(0).unwrap() == 'z' {
            if op != "XOR" {
                sus.push(gate.clone());
                continue;
            }
            // Make sure the inputs are also in the map (i.e. not an input XOR)
            if !rev_map.contains_key(a) || !rev_map.contains_key(b) {
                sus.push(gate.clone());
                continue;
            }
            continue;
        }
        let input_count = find_input_count(gate, &rev_map);
        match op.as_str() {
            "XOR" => {
                // Should be read from input, check that it is used correctly
                if input_count.0 != 1 && input_count.1 != 1 {
                    sus.push(gate.clone());
                }
            }
            "AND" => {
                // Read from input or middle-layer before carry-over bit
                if input_count.2 != 1 {
                    sus.push(gate.clone());
                }
            }
            "OR" => {
                // Must be carry-over bit
                if input_count.0 != 1 && input_count.1 != 1 {
                    sus.push(gate.clone());
                }
                
            }
            _ => {panic!("WRONG OPERATION {}", op)}
        }
    }
    if sus.len() != 8 {
        println!("Did not find the correct amount of gates to swap ({} != 8)", sus.len());
        return
    }
    // Sort and print the gates (no need to find which pairs should switch)
    sus.sort();
    println!("part2: {}", sus.join(","));
}
