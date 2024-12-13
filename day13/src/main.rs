use std::fs::read_to_string;

use aoc_tools::input::input::ints_from_str;

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

fn main() {
    let mut claws: Vec<Claw> = read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|s| {
            let vals = ints_from_str(s);
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
