use std::collections::{HashMap, HashSet};

use aoc_tools::input::input::read_lines;

#[derive(Clone, Debug)]
struct Guard {
    r: i64,
    c: i64,
    dir: i64,
    hist: HashSet<(i64, i64, i64)>,
}

impl Guard {
    fn walk(&mut self) {
        self.hist.insert(self.pos_dir());
        let (nr, nc) = self.next();
        self.r = nr;
        self.c = nc;
    }

    fn next(&self) -> (i64, i64) {
        match self.dir {
            0 => {
                // east
                return (self.r, self.c+1)
            }
            1 => {
                // south
                return (self.r+1, self.c)
            }
            2 => {
                // west
                return (self.r, self.c-1)
            }
            3 => {
                // north
                return (self.r-1, self.c)
            }
            _ => {panic!("Guard has invalid direction")}
        }
    }

    fn turn_right(& mut self) {
        self.dir = (self.dir + 1) % 4;
    }

    fn pos_dir(&self) -> (i64, i64, i64) {
        return (self.r, self.c, self.dir)
    }

    fn within(&self, bounds: (i64, i64)) -> bool {
        return self.r >= 0  && self.r < bounds.0 && self.c >= 0  && self.c < bounds.1
    }

    fn walk_until_done(&mut self, bounds: (i64, i64), obstacles: &HashSet<(i64, i64)>) -> bool {
        while self.within(bounds)
            && !self.hist.contains(&self.pos_dir()) {
            self.walk();
            while obstacles.contains(&self.next()) {
                self.turn_right();
            }
        }
        return self.hist.contains(&self.pos_dir())
    }
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let bounds = (input[0].len() as i64, input.len() as i64);
    let mut guard = Guard{r: 0, c: 0, dir: 0, hist: HashSet::new()};
    let mut obstacles: HashSet<(i64, i64)> = HashSet::new();
    for (r, row) in input.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            match ch {
                '#' => {
                    obstacles.insert((r as i64, c as i64));
                }
                // Only guard always starts in "north" direction
                '^' => {
                    guard.r = r as i64;
                    guard.c = c as i64;
                    guard.dir = 3;
                }
                _ => {}
            }
        }
    }

    let mut p1_guard = guard.clone();
    p1_guard.walk_until_done(bounds, &obstacles);
    let visited: HashSet<(i64, i64)> = p1_guard.hist.iter().map(|x| (x.0, x.1)).collect();
    println!("part1: {}", visited.len());

    let loops = visited.iter()
        .filter_map(|(r1, c1)| {
            if *r1 == guard.r && *c1 == guard.c {
                return None;
            }
            obstacles.insert((*r1, *c1));
            let mut p2_guard = guard.clone();
            let loops = p2_guard.walk_until_done(bounds, &obstacles);
            obstacles.remove(&(*r1, *c1));
            if loops {
                Some(1)
            } else {
                None
            }
        })
        .count();
    println!("part2: {}", loops);
}
