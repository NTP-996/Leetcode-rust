use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        for i in 0..s.len() {
            let mut hash: HashSet<char> = HashSet::new();
            for (j, c) in s.chars().skip(i).enumerate() {
                if !hash.contains(&c) {
                    longest = longest.max(j + 1);
                    hash.insert(c);
                } else {
                    break;
                }
            }
        }
        longest as i32
    }
}

fn main() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abcabcbb")),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abba")),
        2
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("cdd")),
        2
    );
    assert_eq!(Solution::length_of_longest_substring(String::from(" ")), 1);
}
