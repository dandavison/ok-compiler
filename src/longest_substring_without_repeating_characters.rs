use std::collections::HashSet;
use std::convert::TryInto;

// struct Solution {}

// pub fn main() {
//     println!(
//         "{}",
//         Solution::length_of_longest_substring("abcabcbb".to_owned())
//     );
// }

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        (0..s.len())
            .map(|i| length_at(i, &s))
            .max()
            .unwrap()
            .try_into()
            .unwrap()
    }
}

fn length_at(i: usize, s: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut n = 0;
    for c in s.chars().skip(i) {
        if seen.contains(&c) {
            return n;
        } else {
            seen.insert(c);
            n += 1;
        }
    }
    n
}
