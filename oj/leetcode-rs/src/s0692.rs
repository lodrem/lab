pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        use std::collections::HashMap;

        let mut frequency_by_word = HashMap::new();
        for word in words.into_iter() {
            *frequency_by_word.entry(word).or_insert(0) += 1;
        }

        let mut freq: Vec<(String, usize)> = frequency_by_word.into_iter().collect();

        freq.sort_by(|x, y| {
            if x.1 == y.1 {
                x.0.cmp(&y.0)
            } else {
                y.1.cmp(&x.1)
            }
        });

        freq.into_iter()
            .take(k as usize)
            .map(|(word, _)| word)
            .collect()
    }
}
