use std::collections::HashSet;

use aoc_tools::input::input::*;

#[derive(Debug, Clone)]
struct Robot {
    p: (i64, i64),
    v: (i64, i64)
}

impl Robot {
    fn pos_after_time(&self, t: i64, width: i64, height: i64) -> (i64, i64) {
        let mut new_x = (self.p.0 + self.v.0 * t) % width;
        if new_x < 0 {
            new_x += width;
        }
        let mut new_y = (self.p.1 + self.v.1 * t) % height;
        if new_y < 0 {
            new_y += height;
        }
        (new_x, new_y)
    }

    fn period(&self, width: i64, height: i64) -> i64 {
        let mut period = 1;
        while self.p != self.pos_after_time(period, width, height) {
            period += 1;
        }
        period
    }
}

fn which_half(x: i64, tot: i64) -> Option<usize> {
    if x < tot/2 {
        return Some(0)
    }
    if x > tot/2 {
        return Some(1)
    }
    None
}

fn max_row_sum(robots: &HashSet<(i64, i64)>, row: i64, width: i64) -> i64 {
    let mut row_sum = 0;
    for c in 0..width {
        if robots.contains(&(row, c)) {
            row_sum += 1;
        }
    }
    row_sum
}

fn print_robots(robots: HashSet<(i64, i64)>, width: i64, height: i64) {
    for i in 0..height {
        for j in 0..width {
            if robots.contains(&(j, i)) {
                print!("#");
            } else {
                print!(" ")
            }
        }
        println!("");
    }
}

fn main() {
    let robots: Vec<Robot> = read_lines::<String>("input.txt")
        .unwrap()
        .into_iter()
        .map(|s| {
            let ints = ints_from_str(&s);
            Robot{p: (ints[0], ints[1]), v: (ints[2], ints[3])}
        })
        .collect();
    // This must be change to (11, 7) if example is used
    let (width, height) = (101, 103);

    // Part 1
    let mut quads = vec![0,0,0,0];
    for r in robots.iter() {
        let (f_x, f_y) = r.pos_after_time(100, width, height);
        let q_row = match which_half(f_x, width) {
            Some(val) => {val}
            None => {continue;}
        };
        let q_col = match which_half(f_y, height) {
            Some(val) => {val}
            None => {continue;}
        };
        quads[q_row * 2 + q_col] += 1
    }
    let mut p1 = 1;
    for q in quads.iter() {
        p1 = p1 * q
    }
    println!("part1: {}", p1);

    // Part 2
    // I noticed that all the robots have the same periodicity,
    // so I only have to check all time steps before that.
    //
    // I assumed that the tree would have a long base, so I made a
    // heuristic that the wanted time step has the largest single sum
    // of unique position in a row.
    let period = robots[0].period(width, height);
    let mut largest_row_sums: (i64, i64, HashSet<(i64, i64)>) = (0, 0, HashSet::new());
    for t in 0..period {
        let robot_positions: HashSet<(i64, i64)> = robots.iter()
            .map(|r| r.pos_after_time(t, width, height))
            .collect();
        let max_row = (0..height).map(|row| max_row_sum(&robot_positions, row, width)).max().unwrap();
        if max_row > largest_row_sums.1 {
            largest_row_sums = (t, max_row, robot_positions);
        }
    }
    // change this to true to print a nice christmas tree
    if false {
        print_robots(largest_row_sums.2, width, height);
    }
    println!("part2: {}", largest_row_sums.0);
}
