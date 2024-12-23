use std::{collections::{HashMap, HashSet}, iter::zip};

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

fn get_possible_steps(a: (i32, i32), b: (i32, i32), valid: &HashSet<(i32, i32)>) -> Vec<String> {
    let mut perms: Vec<String> = Vec::new();
    let mut start: Vec<char> = Vec::from(['A']); 
    add_remaining(&mut perms, &mut start, a, b, valid);
    perms
}

fn add_remaining(perms: &mut Vec<String>, curr: &mut Vec<char>, a: (i32, i32), b: (i32, i32), valid: &HashSet<(i32, i32)>) {
    if !valid.contains(&a) {
        return
    }
    if a == b {
        curr.push('A');
        perms.push(curr.iter().collect());
        curr.pop();
        return
    }
    let dr = b.0-a.0;
    if dr > 0 {
        curr.push('v');
        add_remaining(perms, curr, (a.0+1, a.1), b, valid);
        curr.pop();
    } else if dr < 0 {
        curr.push('^');
        add_remaining(perms, curr, (a.0-1, a.1), b, valid);
        curr.pop();
    }
    let dc = b.1-a.1;
    if dc > 0 {
        curr.push('>');
        add_remaining(perms, curr, (a.0, a.1+1), b, valid);
        curr.pop();
    } else if dc < 0 {
        curr.push('<');
        add_remaining(perms, curr, (a.0, a.1-1), b, valid);
        curr.pop();
    }
}

fn dfs_min_steps(
    instructions: String,
    layout: &HashMap<char, (i32, i32)>,
    valid: &HashSet<(i32, i32)>,
    cache: &mut HashMap<(usize, char, char), i64>,
    level: usize) -> i64 {
    if level == 0 {
        // Reached the end, return the presses (-1 due to sharing connecting chars)
        return instructions.len() as i64 - 1
    }
    let mut min_steps = 0;
    for (a, b) in zip(instructions[0..].chars(), instructions[1..].chars()) {
        if let Some(min_sub) = cache.get(&(level, a, b)) {
            min_steps += min_sub;
            continue;
        }
        let pos_a = layout.get(&a).unwrap();
        let pos_b = layout.get(&b).unwrap();
        let min_sub = get_possible_steps(*pos_a, *pos_b, valid)
                        .iter()
                        .map(|p| dfs_min_steps(p.clone(), &layout, valid, cache, level-1))
                        .min()
                        .unwrap();
        cache.insert((level, a, b), min_sub);
        min_steps += min_sub;
    }
    min_steps
}

fn code_to_val(code: &str) -> i64 {
    code.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap()
}

fn main() {
    let codes = read_lines::<String>("input.txt").unwrap();
    let keypad_layout = create_keypad_layout();
    let keypad_positions: HashSet<(i32, i32)> = keypad_layout.values().map(|v| *v).collect();
    let input_layout = create_input_pad_layout();
    let input_positions: HashSet<(i32, i32)> = input_layout.values().map(|v| *v).collect();

    let all: Vec<i64> = vec![2,25].iter()
        .map(|level| {
        let mut cache: HashMap<(usize, char, char), i64> = HashMap::new();
        codes.iter()
            .map(|c| {
                let mut tot = 0;
                let mut code: Vec<char> = c.chars().collect();
                code.insert(0, 'A');
                for (a,b) in zip(&code[0..], &code[1..]) {
                    let pos_a = keypad_layout.get(a).unwrap();
                    let pos_b = keypad_layout.get(b).unwrap();
                    tot += get_possible_steps(*pos_a, *pos_b, &keypad_positions)
                        .iter()
                        .map(|p| {
                            dfs_min_steps(p.clone(), &input_layout, &input_positions, &mut cache, *level)
                        })
                        .min()
                        .unwrap();
                }
                tot * code_to_val(c)
            }).sum::<i64>()
        }).collect();
    println!("part1: {}", all[0]);
    println!("part2: {}", all[1]);
}
