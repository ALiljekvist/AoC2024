use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Claw {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64)
}

impl Claw {
    fn lowest_prize(&self) -> i64 {
        let mut n_a = 0i64;
        let mut n_b = 0i64;
        let nom = self.prize.1 * self.b.0 - self.prize.0 * self.b.1;
        let den = self.a.1 * self.b.0 - self.a.0 * self.b.1;
        if nom % den == 0 {
            n_a = nom/den;
            n_b = (self.prize.0 - self.a.0 * n_a) / self.b.0;
        }
        n_a * 3 + n_b
    }
}

fn ints_from_string(s: &str) -> Vec<i64> {
    let mut ints: Vec<i64> = Vec::new();
    let mut val: Option<i64> = None;
    let mut neg = false;
    for k in s.chars() {
        if val == None {
            if k == '-' {
                neg = true;
                continue;
            }
            if k.is_digit(10) {
                val = Some(k.to_digit(10).unwrap() as i64);
            }
            continue;
        }
        if let Some(next_digit) = k.to_digit(10) {
            val = Some(val.unwrap()*10 + next_digit as i64);
            continue;
        }
        // non-digit with previous digits, add to vector and reset val
        let mut mult = 1i64;
        if neg {
            mult = -1;
        }
        ints.push(mult * val.unwrap());
        val = None;
        neg = false;
    }
    if let Some(mag) = val {
        let mut mult = 1i64;
        if neg {
            mult = -1;
        }
        ints.push(mult * mag);
    }
    ints
}

fn main() {
    let mut claws: Vec<Claw> = read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|s| {
            let vals = ints_from_string(s);
            Claw{a: (vals[0], vals[1]), b: (vals[2], vals[3]), prize: (vals[4], vals[5])}
        })
        .collect();
    let p1 = claws.iter().map(|c| c.lowest_prize()).sum::<i64>();
    println!("part1: {}", p1);
    let conversion_error = 10000000000000i64;
    for claw in claws.iter_mut() {
        claw.prize.0 += conversion_error;
        claw.prize.1 += conversion_error;
    }
    let p2 = claws.iter().map(|c| c.lowest_prize()).sum::<i64>();
    println!("part2: {}", p2);
}
