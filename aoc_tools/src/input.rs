pub mod input {
    use std::{fs, str::FromStr};

    pub fn read_lines<T>(filepath: &str) -> Result<Vec<T>, T::Err>
    where T: FromStr, <T as FromStr>::Err : std::fmt::Debug {
        fs::read_to_string(filepath)
        .unwrap()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse())
        .collect()
    }

    pub fn ints_from_str(s: &str) -> Vec<i64> {
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
}