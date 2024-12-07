use aoc_tools::input::input::read_lines;
use regex::Regex;

#[derive(Debug)]
struct Calibration {
    val: i64,
    nums: Vec<i64>
}

impl Calibration {
    fn calibrated(&self, all_operators: bool) -> Option<i64> {
        if !self.evalutate(0, 0, all_operators) {
            return None
        }
        Some(self.val)
    }

    fn evalutate(&self, curr_val: i64, i: usize, all_operators: bool) -> bool {
        if i == self.nums.len() {
            return curr_val == self.val
        }
        if curr_val > self.val {
            return false
        }
        if all_operators && self.evalutate(concat_nums(curr_val, self.nums[i]), i+1, all_operators) {
            return true
        }
        if self.evalutate(curr_val * self.nums[i], i+1, all_operators) {
            return true
        }
        if self.evalutate(curr_val + self.nums[i], i+1, all_operators) {
            return true
        }
        false
    }
}

fn concat_nums(a: i64, b: i64) -> i64 {
    return a * 10i64.pow(b.to_string().len() as u32) + b
}

fn main() {
    let calibrations: Vec<Calibration> = read_lines::<String>("input.txt")
        .unwrap()
        .into_iter()
        .map(|s| {
            let re = Regex::new(r"(\d+)").unwrap();
            let nums: Vec<i64> = re.captures_iter(&s)
                .into_iter()
                .map(|cap| {
                    let (_, [num]) = cap.extract();
                    num.parse::<i64>().unwrap()
                }).collect();
            Calibration{val: nums[0], nums: nums[1..].to_vec()}
        })
        .collect();
    let p1 = calibrations.iter()
        .filter_map(|c| c.calibrated(false))
        .sum::<i64>();
    println!("part1: {}", p1);
    let p2 = calibrations.iter()
        .filter_map(|c| c.calibrated(true))
        .sum::<i64>();
    println!("part2: {}", p2);
}
