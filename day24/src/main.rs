use std::{collections::HashMap, fs::read_to_string, time::Instant};

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

fn get_binary_result(gates: &HashMap<String, u8>, b: char) -> u64 {
    let mut b_gates: Vec<String> = gates.keys().filter(|s| s.chars().nth(0).unwrap() == b).map(|s| s.clone()).collect();
    b_gates.sort();
    b_gates.iter()
        .enumerate()
        .map(|(i, gate)| *gates.get(gate).unwrap() as u64 * 2u64.pow(i as u32))
        .sum::<u64>()
}

fn from_binary(a: Vec<u8>) -> u64 {
    return a.iter().enumerate().map(|(i, v)| *v as u64 * 2u64.pow(i as u32)).sum::<u64>();
}

fn to_binary(a: u64) -> Vec<u8> {
    let mut bins = Vec::new();
    let mut i = 0;
    while a > 2u64.pow(i as u32) {
        bins.push(if a & 2u64.pow(i as u32) > 0 {1} else {0});
        i += 1
    }
    bins
}

fn get_or_swap(a: String, swaps: &HashMap<String, String>) -> String {
    match swaps.get(&a) {
        Some(b) => {b.clone()}
        None => {a}
    }
}

fn calc_gate(
    gate: &String,
    rev_map: &HashMap<String, (String, String, String)>,
    gates: &HashMap<String, u8>,
    swaps: &HashMap<String, String>,
) -> u8 {
    if let Some(val) = gates.get(gate) {
        return *val
    }
    let (op, dep1, dep2) = rev_map.get(gate).unwrap();
    let a = calc_gate(&get_or_swap(dep1.clone(), swaps), rev_map, gates, swaps);
    let b = calc_gate(&get_or_swap(dep2.clone(), swaps), rev_map, gates, swaps);
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

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (gates, ops) = parse_input(input);
    // Part 1
    let mut all_gates: Vec<String> = Vec::new();
    let rev_map: HashMap<String, (String, String, String)> = ops.iter()
        .map(|ps| {
            all_gates.push(ps[4].clone());
            (ps[4].clone(), (ps[1].clone(), ps[0].clone(), ps[2].clone()))
        })
        .collect();
    let mut z_gates: Vec<String> = all_gates.iter()
        .filter_map(|s| if s.chars().nth(0).unwrap() == 'z' {Some(s.clone())} else {None})
        .collect();
    z_gates.sort();
    let now = Instant::now();
    let finish: Vec<u8> = z_gates.iter().map(|gate| calc_gate(gate, &rev_map, &gates, &HashMap::new())).collect();
    println!("part1: {} ({:?})", from_binary(finish), now.elapsed());

    // Part 2
    let wanted = get_binary_result(&gates, 'x') + get_binary_result(&gates, 'y');
    let mut wanted_bin = to_binary(wanted);
    while wanted_bin.len() < z_gates.len() {
        wanted_bin.push(0);
    }
    // Realize that the input of earlier z-gates affect the later z-gates, which means
    // they should follow a pattern. Probably have to swap all z-gates that is not
    // calculated with an XOR.
}
