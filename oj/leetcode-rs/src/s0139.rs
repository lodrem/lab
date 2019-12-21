pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // f[i] = min{f[i], f[i - word[j]] | j -> 0..word.len}

        let n = s.len();

        let mut f = vec![false; n];

        for i in 0..n {
            if i > 0 && !f[i - 1] {
                continue;
            }

            for (_, word) in word_dict.iter().enumerate() {
                let j = i + word.len();
                if j > n || f[j - 1] {
                    continue;
                }

                f[j - 1] = &s[i..j] == word
            }
        }

        f[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert!(Solution::word_break(
            "leetcode".to_owned(),
            vec!["leet", "code"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect()
        ));

        assert!(Solution::word_break(
            "applepenapple".to_owned(),
            vec!["apple", "pen"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect()
        ));

        assert!(!Solution::word_break(
            "catsandog".to_owned(),
            vec!["cats", "dog", "sand", "and", "cat"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect()
        ));
    }
}
