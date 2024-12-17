use std::iter::zip;

use aoc_tools::input::input::*;

fn combo(b: i64, registers: &Vec<i64>) -> i64 {
    match b {
        4 => {registers[0]}
        5 => {registers[1]}
        6 => {registers[2]}
        _ => {b}
    }
}

fn operate(a: i64, b: i64, registers: &mut Vec<i64>) -> Option<i64> {
    match a {
        // adv
        0 => {
            registers[0] = registers[0] / 2i64.pow(combo(b, registers) as u32);
            None
        }
        // bxl
        1 => {
            registers[1] = registers[1] ^ b;
            None
        }
        // bst
        2 => {
            registers[1] = combo(b, registers) % 8;
            None
        }
        // jnz
        3 => {
            // Do nothing, handled outside
            None
        }
        // bxc
        4 => {
            registers[1] = registers[1] ^ registers[2];
            None
        }
        // out
        5 => {
            Some(combo(b, registers) % 8)
        }
        // bdv
        6 => {
            registers[1] = registers[0] / 2i64.pow(combo(b, registers) as u32);
            None
        }
        // cdv
        7 => {
            registers[2] = registers[0] / 2i64.pow(combo(b, registers) as u32);
            None
        }
        _ => {None}
    }
}

fn run(program: &Vec<i64>, registers: &mut Vec<i64>, match_in: bool) -> (Vec<i64>, bool) {
    let mut op_pointer = 0;
    let mut out: Vec<i64> = Vec::new();
    while op_pointer < program.len() {
        if let Some(val) = operate(program[op_pointer], program[op_pointer+1], registers) {
            out.push(val);
            if match_in {
                if out.len() > program.len() {
                    return (out, false)
                }
                if val != program[out.len()-1] {
                    return (out, false)
                }
            }
        }
        if program[op_pointer] == 3 && registers[0] != 0 {
            op_pointer = program[op_pointer+1] as usize;
            continue;
        }
        op_pointer += 2;
    }
    let matched = out.len() == program.len();
    (out, matched)
}

// fn check_for_periodicity(a: &Vec<i64>) -> Option<Vec<i64>> {
//     let diffs: Vec<i64> = zip(&a[0..], &a[1..]).into_iter().map(|b| b.1-b.0).collect();
//     for t in 1..diffs.len()/2 {
//         if zip(&diffs[0..], &diffs[t..]).map(|b| *b.1-*b.0).all(|x| x == 0) {
//             println!("period on diffs: {}, total period: {}", t, diffs.iter().sum::<i64>());
//             let periods: Vec<i64> = diffs[..t].iter().map(|d| *d).collect();
//             println!("diffs: {:?}", diffs);
//             println!("periods: {:?}", periods);
//             return Some(periods);
//         }
//     }
//     None
// }

fn find_lowest_autoprogram(program: &Vec<i64>) -> i64 {
    // Used the commented code to find the start and pattern on when the produced output was correct up to the
    // second last digit. Then I used that starting point and the found increases to do a selected brute-force.
    let mut a = 3287450;
    let increases = [98304, 3432193, 255, 1383, 8, 662161, 98304, 3530752, 1383, 8, 531089, 32768, 98304, 3432193, 255, 1383, 8, 662161, 98304, 901120, 499457, 255, 1383, 8, 23185, 262144, 1310720, 499457, 255, 1383, 8, 23185, 262144, 245760, 16384, 49152, 98304, 3432193, 255, 1383, 8, 662161, 98304, 3530752, 1383, 8, 531089, 32768, 98304, 3432193, 255, 1383, 8, 662161, 98304, 4096000, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4030464, 16384, 49152, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4096000, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 901120, 2097152, 1032192, 16384, 49152, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4096000, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4030464, 16384, 49152, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4096000, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 901120, 786432, 1310720, 786432, 245760, 16384, 49152, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4096000, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4030464, 16384, 49152, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4096000, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 901120, 2097152, 1032192, 16384, 49152, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4096000, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4030464, 16384, 49152, 98304, 4096000, 98304, 4063232, 32768, 98304, 4096000, 98304, 4096000];
    let mut p = 0usize;
    loop {
        let mut registers = vec![a, 0, 0];
        let (_out, matched) = run(program, &mut registers, true);
        if matched {
            return a
        }
        a += increases[p];
        p = (p + 1) % increases.len();
        // if _out.len() > level && _out[level] == program[level] && period.len() == 0 {
        //     hits.push(a);
        //     if let Some(_found_period) = check_for_periodicity(&hits) {
        //         // println!("{:?}", found_period);
        //         // println!("{:?}", period);
        //         period = _found_period;
        //         // return 0
        //     }
        //     if period.len() == 0 {
        //         p += 1;
        //     }
        // }
        // if _out.len() > level+1 && _out[level+1] == program[level+1] && period2.len() == 0 {
        //     if start2 == 0 {
        //         start2 = a;
        //     }
        //     hits2.push(a);
        //     if let Some(_found_period) = check_for_periodicity(&hits2) {
        //         // println!("{:?}", found_period);
        //         // println!("{:?}", period);
        //         period2 = _found_period;
        //         println!("start from: {}", start2);
        //         return 0
        //     }
        //     // if period.len() == 0 {
        //     //     p += 1;
        //     // }
        // }
        // if period.len() > 0 {
        //     println!("{}", a);
        //     p = p % period.len();
        //     a += period[p];
        //     p += 1;
        //     continue;
        // }
        // a += 1;
    }
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let mut registers: Vec<i64> = (0..input.len()-1).map(|x| ints_from_str(&input[x])[0]).collect();
    let program = ints_from_str(&input[input.len()-1]);
    let (out, _) = run(&program, &mut registers, false);
    println!("part1: {}", out.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(","));

    // Reset register
    println!("part2: {}", find_lowest_autoprogram(&program));
}