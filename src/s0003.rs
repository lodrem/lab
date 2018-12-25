pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut chars = HashMap::new();
        let mut start_idx = 0;
        let mut max_len = 0;

        for (i, c) in s.chars().enumerate() {
            match chars.get(&c) {
                Some(idx) if *idx > start_idx => {
                    start_idx = *idx;
                }
                _ => {
                    let len = i - start_idx + 1;
                    if len > max_len {
                        max_len = len;
                    }
                }
            };

            chars.insert(c, i + 1);
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_longest_substring(String::from("")), 0);
        assert_eq!(Solution::length_of_longest_substring(String::from(" ")), 1);
        assert_eq!(Solution::length_of_longest_substring(String::from("au")), 2);
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
}
