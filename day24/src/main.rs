use std::{collections::HashMap, fs::read_to_string};

// struct Adder {
//     c_in: Option<String>,
//     xor_in: Option<String>,
//     and_in: Option<String>,
//     and_mid: Option<String>,
//     xor_out: Option<String>
// }

// impl Adder {
//     fn new() -> Self {
//         Adder { c_in: None, xor_in: None, and_in: None, and_mid: None, xor_out: None }
//     }
// }

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

fn get_rev_map(ops: &Vec<Vec<String>>, swaps: &HashMap<String, String>) -> HashMap<String, (String, String, String)> {
    ops.iter()
        .map(|ps| {
            (get_or_swap(ps[4].clone(), swaps), (ps[1].clone(), ps[0].clone(), ps[2].clone()))
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
) -> u8 {
    if let Some(val) = gates.get(gate) {
        return *val
    }
    let (op, dep1, dep2) = rev_map.get(gate).unwrap();
    let a = calc_gate(dep1, rev_map, gates);
    let b = calc_gate(dep2, rev_map, gates);
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
    let rev_map = get_rev_map(&ops, &HashMap::new());
    let mut z_gates: Vec<String> = all_gates.iter()
        .filter_map(|s| if s.chars().nth(0).unwrap() == 'z' {Some(s.clone())} else {None})
        .collect();
    z_gates.sort();
    let finish: Vec<u8> = z_gates.iter().map(|gate| calc_gate(gate, &rev_map, &gates)).collect();
    println!("part1: {}", from_binary(finish));

    // Part 2:
    // Realize that the input of earlier z-gates affect the later z-gates, which means
    // they should follow a pattern. Seems to be logic adders with a carry-over from
    // previous gates.

    let mut sus: Vec<String> = Vec::new();
    for (gate, (op, a, b)) in rev_map.iter() {
        // We trust the first and last output gate (manually verified)
        if gate == &z_gates[0] || gate == &z_gates[z_gates.len()-1] {
            continue;
        }
        // Since the first gate does not have a carry-over bit, the pattern
        // will not hold for first input gates. Skip those, manually verified.
        if vec!["x00", "y00"].contains(&a.as_str()) {
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
    sus.sort();
    println!("part2: {}", sus.join(","));

    // Could add checking pairs of swaps until we get a match. This check is still hard-coded from manual findings.
    let wanted_bin = to_binary(get_binary_result(&gates, 'x') + get_binary_result(&gates, 'y'));
    let mut swaps: HashMap<String, String> = HashMap::new();
    for (p1, p2) in vec![("z05", "tst"), ("z11", "sps"), ("z23","frt"), ("cgh", "pmd")] {
        swaps.insert(p1.to_string(), p2.to_string());
        swaps.insert(p2.to_string(), p1.to_string());
    }
    //  Create reverse map and check that the swaps made the logic work
    let new_rev_map = get_rev_map(&ops, &swaps);
    let p2_finish: Vec<u8> = z_gates.iter().map(|gate| calc_gate(gate, &new_rev_map, &gates)).collect();
    if (0..p2_finish.len()).map(|i| if p2_finish[i] != wanted_bin[i] {1} else {0}).sum::<i32>() > 0 {
        println!("Not correct");
    }
}
