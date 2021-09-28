// You are given an array of positive integers nums and want to erase a subarray
// containing unique elements. The score you get by erasing the subarray is
// equal to the sum of its elements.

// Return the maximum score you can get by erasing exactly one subarray.

// An array b is called to be a subarray of a if it forms a contiguous
// subsequence of a, that is, if it is equal to a[l],a[l+1],...,a[r] for some
// (l,r).

use std::collections::HashSet;

// struct Solution {}

// pub fn main() {
//     println!("{}", Solution::maximum_unique_subarray(vec![0, 1, 2]));
// }

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        (0..nums.len()).map(|i| sum_at(i, &nums)).max().unwrap()
    }
}

fn sum_at(i: usize, nums: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    let mut sum = 0;
    for n in nums.iter().skip(i) {
        if seen.contains(&n) {
            return sum;
        } else {
            seen.insert(n);
            sum += n;
        }
    }
    sum
}
