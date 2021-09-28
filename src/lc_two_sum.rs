use std::collections::HashMap;

impl Solution {
    // Passes tests
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for (idx, n) in nums.iter().enumerate() {
            match seen.get(&(target - n)) {
                Some(&complement_idx) => {
                    return vec![idx as i32, complement_idx as i32];
                }
                None => {
                    seen.insert(n, idx);
                }
            }
        }
        panic!("Impossible")
    }
}

struct Solution;

pub fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 15, 7, 11], 9));
}
