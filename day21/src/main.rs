use std::{collections::{HashMap, HashSet}, iter::zip, vec};

use aoc_tools::input::input::read_lines;

/*
    Keypad layout looks like:
    _______
    |7|8|9|
    |4|5|6|
    |1|2|3|
    | |0|A|
    -------
*/
fn create_keypad_layout() -> HashMap<char, (i32, i32)> {
    HashMap::<char, (i32, i32)>::from([
        ('7', (0, 0)), ('8', (0, 1)), ('9', (0, 2)),
        ('4', (1, 0)), ('5', (1, 1)), ('6', (1, 2)),
        ('1', (2, 0)), ('2', (2, 1)), ('3', (2, 2)),
        ('0', (3, 1)), ('A', (3, 2)),
        ])
}


/*
    Input layout looks like:
    _______
    | |^|A|
    |<|v|>|
    -------
*/
fn create_input_pad_layout() -> HashMap<char, (i32, i32)> {
    HashMap::<char, (i32, i32)>::from([
        ('^', (0, 1)), ('A', (0, 2)),
        ('<', (1, 0)), ('v', (1, 1)), ('>', (1, 2)),
        ])
}

fn min_steps(a: char, b: char, layout: &HashMap<char, (i32, i32)>, positions: &HashSet<(i32, i32)>) -> String {
    let p0 = layout.get(&a).unwrap();
    let p1 = layout.get(&b).unwrap();
    let dr = p1.0-p0.0;
    let dc = p1.1-p0.1;
    let mut new_steps: Vec<char> = Vec::new();
    if dc > 0 {
        if positions.contains(&(p1.0, p0.1)) {
            new_steps.append(&mut get_up_down(dr));
            new_steps.append(&mut get_right_left(dc));
        } else {
            new_steps.append(&mut get_up_down(dr));
            new_steps.append(&mut get_right_left(dc));
        }
    } else {
        if !positions.contains(&(p0.0, p1.1)) {
            new_steps.append(&mut get_up_down(dr));
            new_steps.append(&mut get_right_left(dc));
        } else {
            new_steps.append(&mut get_right_left(dc));
            new_steps.append(&mut get_up_down(dr));
        }
    }
    new_steps.push('A');
    new_steps.iter().collect()
}

fn get_up_down(dr: i32) -> Vec<char> {
    let mut up_down = Vec::new();
    for _ in 0..dr.abs() {
        if dr < 0 {
            up_down.push('^');
        } else {
            up_down.push('v');
        }
    }
    up_down
}

fn get_right_left(dc: i32) -> Vec<char> {
    let mut right_left = Vec::new();
    for _ in 0..dc.abs() {
        if dc < 0 {
            right_left.push('<');
        } else {
            right_left.push('>');
        }
    }
    right_left
}

fn code_to_inputs<'a>(code: &'a str, layout: &HashMap<char, (i32, i32)>) -> String {
    let positions: HashSet<(i32, i32)> = layout.values().map(|v| *v).collect();
    let mut path: Vec<(char, char)> = zip(code[0..].chars(), code[1..].chars()).collect();
    // Have to insert first step as well
    path.insert(0, ('A', code.chars().nth(0).unwrap()));
    let mut key_presses: Vec<char> = Vec::new();
    for step in &path {
        let elevated_steps = min_steps(step.0, step.1, layout, &positions);
        key_presses.append(&mut elevated_steps.chars().collect());
    }
    key_presses.into_iter().collect()
}

fn dfs_min_steps(
    instructions: String,
    map: &HashMap<(char, char),String>,
    cache: &mut HashMap<(usize, char, char),i64>,
    level: usize,
    max_level: usize) -> i64 {
    if level == max_level {
        return instructions.len() as i64
    }
    let mut full_input = instructions.clone();
    full_input.insert_str(0, "A");
    let mut min_steps = 0;
    for (a, b) in zip(full_input[0..].chars(), full_input[1..].chars()) {
        if let Some(min_sub) = cache.get(&(level, a, b)) {
            min_steps += min_sub;
            continue;
        }
        let steps = map.get(&(a,b)).unwrap().clone();
        let min_sub = dfs_min_steps(steps, map, cache, level+1, max_level);
        cache.insert((level, a, b), min_sub);
        min_steps += min_sub;
    }
    min_steps
}

// fn find_least_button_pushes(
//     code: &str,
//     keypad: &HashMap<char, (i32, i32)>,
//     input_pad: &HashMap<char, (i32, i32)>,
//     num_input_pads: usize) -> i64 {
//     let mut instructions = code_to_inputs(&code, &keypad);
//     for _i in 0..num_input_pads {
//         instructions = code_to_inputs(&instructions, input_pad);
//     }
//     instructions.len() as i64
// }

fn code_to_val(code: &str) -> i64 {
    code.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap()
}

fn main() {
    let codes = read_lines::<String>("input.txt").unwrap();
    let keypad_layout = create_keypad_layout();
    let input_layout = create_input_pad_layout();
    let input_positions = input_layout.values().map(|v| *v).collect();
    let input_set = vec!['A', '^', '>', 'v', '<'];
    let mut input_pair_mapping: HashMap<(char, char), String> = HashMap::new();
    for a in input_set.iter() {
        for b in input_set.iter() {
            input_pair_mapping.insert((*a, *b),min_steps(*a, *b, &input_layout, &input_positions));
        }
    }
    let mut cache: HashMap<(usize, char, char), i64> = HashMap::new();
    let mut p1 = 0;
    for code in &codes {
        let b = code_to_inputs(&code, &keypad_layout);
        let m_steps = dfs_min_steps(b, &input_pair_mapping, &mut cache, 0, 2);
        p1 += m_steps * code_to_val(&code)
    }
    println!("part1: {}", p1);

    
    let mut p2_cache: HashMap<(usize, char, char), i64> = HashMap::new();
    let mut p2 = 0;
    for code in &codes {
        let b = code_to_inputs(&code, &keypad_layout);
        let m_steps = dfs_min_steps(b, &input_pair_mapping, &mut p2_cache, 0, 25);
        p2 += m_steps * code_to_val(&code)
    }

    println!("part2: {}", p2);
}
