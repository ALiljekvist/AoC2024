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

// fn map_adder(gate: String, rev_map: &HashMap<String, (String, String, String)>, level: usize) -> String {
//     if level == 0 {
//         return gate
//     }
//     match gate.chars().nth(0).unwrap() {
//         'x' => {return gate}
//         'y' => {return gate}
//         _ => {}
//     }
//     let (op, dep1, dep2) = rev_map.get(&gate).unwrap();
//     let mut parts = vec![map_adder(dep1.clone(), rev_map, level-1), map_adder(dep2.clone(), rev_map, level-1)];
//     parts.sort();
//     format!("({} {} {})", parts[0], op, parts[1])
// }

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (gates, ops) = parse_input(input);
    // Part 1
    let all_gates: Vec<String> = ops.iter().map(|op| op[4].clone()).collect();
    // let rev_map: HashMap<String, (String, String, String)> = ops.iter()
    //     .map(|ps| {
    //         all_gates.push(ps[4].clone());
    //         (ps[4].clone(), (ps[1].clone(), ps[0].clone(), ps[2].clone()))
    //     })
    //     .collect();
    let rev_map = get_rev_map(&ops, &HashMap::new());
    let mut z_gates: Vec<String> = all_gates.iter()
        .filter_map(|s| if s.chars().nth(0).unwrap() == 'z' {Some(s.clone())} else {None})
        .collect();
    z_gates.sort();
    let finish: Vec<u8> = z_gates.iter().map(|gate| calc_gate(gate, &rev_map, &gates)).collect();
    println!("part1: {}", from_binary(finish));

    // Part 2:
    let wanted_bin = to_binary(get_binary_result(&gates, 'x') + get_binary_result(&gates, 'y'));
    // Realize that the input of earlier z-gates affect the later z-gates, which means
    // they should follow a pattern. Seems to be logic adders with a carry-over from
    // previous gates.
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

    // Manually find pairs by printing the patterns and see where it doesn't match.
    // Try to build the sum part of the adder for each z-gate, and print the parts.

    // Uncomment to print what was used to manually search through the input.
    // for gate in z_gates.iter() {
    //     println!("{}: {}", gate, map_adder(gate.clone(), &new_rev_map, 2))
    // }
    // // Also check the dep-chain for all carry-overs (Should all have OR-operations)
    // for (co, (op, _, _)) in new_rev_map.iter() {
    //     if op != "OR" {
    //         continue;
    //     }
    //     println!("{}: {}", co, map_adder(co.clone(), &new_rev_map, 2));
    // }

    let mut sorted_swaps: Vec<String> = swaps.keys().map(|s| s.clone()).collect();
    sorted_swaps.sort();
    println!("part2: {}", sorted_swaps.join(","));
}
