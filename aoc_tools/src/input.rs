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
}