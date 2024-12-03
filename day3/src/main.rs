use std::fs::read_to_string;

fn perform_mul(s: &str) -> Option<i64> {
    let parts: Vec<&str> = s.split(",").collect();
    if parts.len() < 2 {
        return None
    }
    let num1 = match parts[0].parse::<i64>() {
        Ok(val) => {val}
        Err(_) => {return None}
    };
    let num2 = match parts[1].parse::<i64>() {
        Ok(val) => {val}
        Err(_) => {return None}
    };
    Some(num1*num2)
}

fn main() {
    let memory = read_to_string("input.txt").unwrap();
    let mut p1 = 0i64;
    let mut p2 = 0i64;
    let mut valid = true;
    for part in memory.split("mul(").filter(|x| !x.is_empty()) {
        if let Some(ind) = part.find(")") {
            if let Some(res) = perform_mul(&part[..ind]) {
                p1 += res;
                if valid {
                    p2 += res;
                }
            }
        }
        let dos: Vec<usize> = part.match_indices("do()").map(|x| x.0).collect();
        let last_do = if dos.len() > 0 {dos[dos.len()-1]} else {0};
        let donts: Vec<usize> = part.match_indices("don't()").map(|x| x.0).collect();
        let last_dont = if donts.len() > 0 {donts[donts.len()-1]} else {0};
        if last_do > last_dont {
            valid = true;
        } else if last_dont > last_do {
            valid = false;
        }
    }
    println!("part1: {}", p1);
    println!("part2: {}", p2);
}
