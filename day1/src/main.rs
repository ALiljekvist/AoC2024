use aoc_tools::input::input::read_lines;

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let mut l1: Vec::<i64> = Vec::new();
    let mut l2: Vec::<i64> = Vec::new();
    for l in input {
        let nums: Vec<i64> = l.split("   ").into_iter().map(|x| x.parse().unwrap()).collect();
        l1.push(nums[0]);
        l2.push(nums[1]);
    }
    l1.sort();
    l2.sort();
    let mut p1 = 0;
    for i in 0..l1.len() {
        p1 += (l1[i] - l2[i]).abs();
    }
    println!("part1: {}", p1);
    let mut p2 = 0;
    for i in 0..l1.len() {
        let mut cntr = 0;
        for j in 0..l2.len() {
            if l1[i] == l2[j] {
                cntr += 1;
            }
        }
        p2 += cntr * l1[i];
    }
    println!("part2: {}", p2);
}
