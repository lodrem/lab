pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn is_palindrome(s: &str) -> bool {
        let bytes = s.as_bytes();
        for i in 0..s.len() {
            if bytes[i] != bytes[s.len() - i - 1] {
                return false;
            }
        }

        true
    }

    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        let mut map: HashMap<String, usize> = HashMap::default();
        let mut res = vec![];

        for i in 0..words.len() {
            map.insert(words[i].chars().rev().collect(), i);
        }

        if let Some(empty_idx) = map.get("") {
            for i in 0..words.len() {
                if *empty_idx == i {
                    continue;
                }
                if Self::is_palindrome(&words[i]) {
                    res.push(vec![*empty_idx as i32, i as i32]);
                }
            }
        }

        for i in 0..words.len() {
            for j in 0..words[i].len() {
                let (left, right) = words[i].split_at(j);

                if let Some(idx) = map.get(left) {
                    if *idx != i && Self::is_palindrome(right) {
                        res.push(vec![i as i32, *idx as i32]);
                    }
                }

                if let Some(idx) = map.get(right) {
                    if *idx != i && Self::is_palindrome(left) {
                        res.push(vec![*idx as i32, i as i32]);
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::palindrome_pairs(
                vec!["abcd", "dcba", "lls", "s", "sssll"]
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect(),
            ),
            vec![vec![1, 0], vec![0, 1], vec![3, 2], vec![2, 4]]
        );
        assert_eq!(
            Solution::palindrome_pairs(
                vec!["bat", "tab", "cat"]
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect(),
            ),
            vec![vec![1, 0], vec![0, 1]]
        );
    }
}
