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
        return within((self.r, self.c), bounds)
    }

    fn walk_until_done(&mut self, bounds: (i64, i64), obstacles: &HashSet<(i64, i64)>, extra: (i64, i64)) -> bool {
        while self.within(bounds)
            && !self.hist.contains(&self.pos_dir()) {
            self.walk();
            while obstacles.contains(&self.next()) || self.next() == extra {
                self.turn_right();
            }
        }
        return self.hist.contains(&self.pos_dir())
    }
}

fn within(pos: (i64, i64), bounds: (i64, i64)) -> bool {
    return pos.0 >= 0  && pos.0 < bounds.0 && pos.1 >= 0  && pos.1 < bounds.1
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
    let org_pos = (guard.r, guard.c);

    let mut extra_obstacles: HashMap<(i64, i64), i64> = HashMap::new();
    while guard.within(bounds) {
        if obstacles.contains(&guard.next()) {
            guard.turn_right();
        }
        if !extra_obstacles.contains_key(&guard.next())
            && within(guard.next(), bounds)
            && guard.next() != org_pos {
            let new_obstacle = guard.next();
            let mut shadow = guard.clone();
            let mut spin_counter = 0;
            while obstacles.contains(&shadow.next()) || shadow.next() == new_obstacle{
                shadow.turn_right();
                spin_counter += 1;
                if spin_counter > 3 {
                    break
                }
            }
            extra_obstacles.insert(new_obstacle, if spin_counter < 4 && shadow.walk_until_done(bounds, &obstacles, new_obstacle) {1} else {0});
        }
        guard.walk();
    }
    
    let visited: HashSet<(i64, i64)> = guard.hist.iter().map(|x| (x.0, x.1)).collect();
    println!("part1: {}", visited.len());
    println!("part2: {}", extra_obstacles.values().into_iter().sum::<i64>());
}
