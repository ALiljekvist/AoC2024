use std::fs::read_to_string;

fn to_mat(s: &String) -> Vec<Vec<char>> {
    s.split("\n")
        .filter(|s| !s.is_empty())
        .map(|r| r.chars().collect())
        .collect()
}

fn parse_key(s: &String) -> Vec<i32> {
    let mut key = Vec::new();
    let mat = to_mat(s);
    let mut cc = 0;
    while cc < mat[0].len() {
        let mut cr = mat.len()-1;
        while mat[cr][cc] == '#' {
            if cr == 0 {
                break;
            }
            cr -= 1;
        }
        key.push((mat.len()-2-cr) as i32);
        cc += 1;
    }
    key
}

fn parse_lock(s: &String) -> Vec<i32> {
    let mut lock = Vec::new();
    let mat = to_mat(s);
    let mut cc = 0;
    while cc < mat[0].len() {
        let mut cr = 0;
        while cr < mat.len() && mat[cr][cc] == '#' {
            cr += 1;
        }
        lock.push((cr-1) as i32);
        cc += 1;
    }
    lock
}

fn main() {
    let input: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .filter_map(|s| if !s.is_empty() {Some(s.to_string())} else {None})
        .collect();

    let mut keys: Vec<Vec<i32>> = Vec::new();
    let mut locks: Vec<Vec<i32>> = Vec::new();
    for block in &input {
        match block.chars().nth(0).unwrap() {
            '#' => {locks.push(parse_lock(block));}
            '.' => {keys.push(parse_key(block));}
            _ => {}
        }
    }

    let tot = keys.iter().map(|key| {
        locks.iter().map(|lock| {
            if (0..lock.len()).all(|i| key[i]+lock[i]<6) {1} else {0}
        }).sum::<i32>()
    }).sum::<i32>();

    println!("{}", tot);
}
