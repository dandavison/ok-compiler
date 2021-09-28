use std::cmp::max_by_key;

// struct Solution {}

// pub fn main() {
//     println!("{}", Solution::longest_palindrome("babad".to_owned()));
// }

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        (0..s.len())
            .map(|i| get_max_pal_at(i, &s))
            .max_by_key(|s| s.len())
            .unwrap_or_else(|| "".to_owned())
    }
}

fn get_max_pal_at(i: usize, s: &str) -> String {
    max_by_key(get_max_even_pal_at(i, s), get_max_odd_pal_at(i, s), |s| {
        s.len()
    })
}

fn get_max_even_pal_at(i: usize, s: &str) -> String {
    let mut max_pal = "".to_owned();
    if i == 0 || i == s.len() {
        return max_pal;
    }
    let mut l = i - 1;
    let mut r = i;
    let s: Vec<char> = s.chars().collect();
    while s[l] == s[r] {
        max_pal = s[l..r].iter().collect();
        if l == 0 || r + 1 == s.len() {
            return max_pal;
        }
        l -= 1;
        r += 1;
    }
    max_pal
}

fn get_max_odd_pal_at(i: usize, s: &str) -> String {
    let s: Vec<char> = s.chars().collect();
    let mut max_pal = s[i].to_string();
    if i == 0 || i + 1 == s.len() {
        return max_pal;
    }
    let mut l = i - 1;
    let mut r = i + 1;
    while s[l] == s[r] {
        max_pal = s[l..r].iter().collect();
        if l == 0 || r + 1 == s.len() {
            return max_pal;
        }
        l -= 1;
        r += 1;
    }
    max_pal
}
