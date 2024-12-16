use std::collections::{HashMap, HashSet};

use aoc_tools::input::input::read_lines;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Reindeer {
    r: i32,
    c: i32,
    dir: i32,
    score: i64,
    end: (i32, i32),
}

fn instert_prio(deers: &mut Vec<(Reindeer, HashSet<(i32, i32, i32)>)>, deer: Reindeer, history: HashSet<(i32, i32, i32)>) {
    let mut i = 0;
    while i < deers.len() && deer.score < deers[i].0.score {
        i += 1
    }
    deers.insert(i, (deer, history));
}

impl Reindeer {
    fn next(&self) -> (i32, i32) {
        match self.dir {
            0 => {(self.r, self.c+1)}
            1 => {(self.r+1, self.c)}
            2 => {(self.r, self.c-1)}
            3 => {(self.r-1, self.c)}
            _ => {panic!("faulty dir {}", self.dir)}
        }
    }

    fn pos_dir(&self) -> (i32, i32, i32) {
        return (self.r, self.c, self.dir % 2)
    }

    fn turn(&mut self, clockwise: bool) {
        if clockwise {
            self.dir = (self.dir + 1) % 4;
            return
        }
        self.dir -= 1;
        if self.dir < 0 {
            self.dir += 4;
        }
    }

    fn shortest_paths(&self, rocks: &HashSet<(i32, i32)>) -> (i64, HashSet<(i32, i32)>) {
        let mut visited: HashMap<(i32, i32, i32), i64> = HashMap::new();
        let mut deers: Vec<(Reindeer, HashSet<(i32, i32, i32)>)> = vec![(self.clone(), HashSet::new())];
        let mut paths: HashSet<(i32, i32)> = HashSet::new();
        let mut lowest_score: Option<i64> = None;

        while let Some((deer, mut history)) = deers.pop() {
            match lowest_score {
                Some(score) => {
                    // Lowest ongoing score is lower than final minimum so no more
                    // paths can be optimal. Return result.
                    if deer.score > score {
                        return (score, paths)
                    }
                }
                None => {}
            }
            if let Some(min_score) = visited.get(&deer.pos_dir()) {
                if deer.score > *min_score {
                    continue;
                }
            } else {
                visited.insert(deer.pos_dir(), deer.score);
            }
            if history.contains(&deer.pos_dir()) {
                continue;
            }
            history.insert(deer.pos_dir());
            let next = deer.next();
            if deer.end == next {
                history.insert((next.0, next.1, deer.dir % 2));
                match lowest_score {
                    Some(score) => {
                        if deer.score + 1 == score {
                            for pd in history {
                                paths.insert((pd.0, pd.1));
                            }
                        }
                    }
                    None => {
                        lowest_score = Some(deer.score + 1);
                        for pd in history {
                            paths.insert((pd.0, pd.1));
                        }
                    }
                }
                continue;
            }
            if !rocks.contains(&next) && !history.contains(&(next.0, next.1, deer.dir % 2)) {
                let mut forward_deer = deer.clone();
                forward_deer.r = next.0;
                forward_deer.c = next.1;
                forward_deer.score += 1;
                instert_prio(&mut deers, forward_deer, history.clone());
            }
            for b in [false, true] {
                let mut turned_deer = deer.clone();
                turned_deer.turn(b);
                if rocks.contains(&turned_deer.next()) {
                    continue;
                }
                turned_deer.score += 1000;
                instert_prio(&mut deers, turned_deer, history.clone());
            }
        }
        match lowest_score {
            Some(score) => {
                (score, paths)
            }
            None => {(0, paths)}
        }
    }
}



fn parse_map(s: Vec<String>) -> (HashSet<(i32, i32)>, Reindeer) {
    let mut reindeer = Reindeer{r: 0, c: 0, dir: 0, score: 0, end: (0,0)};
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    for (r, row) in s.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            match ch {
                '#' => {rocks.insert((r as i32, c as i32));}
                'S' => {
                    reindeer.r = r as i32;
                    reindeer.c = c as i32;
                }
                'E' => {reindeer.end = (r as i32, c as i32)}
                _ => {}
            }
        }
    }
    (rocks, reindeer)
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let (rocks, reindeer) = parse_map(input);
    let (p1, paths) = Reindeer::shortest_paths(&reindeer, &rocks);
    println!("part1: {}", p1);
    println!("part2: {}", paths.len());
}
