pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn make_hash(s: &str) -> usize {
        const PRIMES: [usize; 26] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101,
        ];

        let mut hash = 1;

        for c in s.as_bytes().into_iter() {
            hash *= PRIMES[(c - b'a') as usize];
        }

        hash
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut hash: HashMap<usize, Vec<String>> = HashMap::default();

        for s in strs.into_iter() {
            let h = Self::make_hash(&s);

            hash.entry(h).or_insert(Vec::new()).push(s);
        }

        hash.into_iter().map(|(_, v)| v).collect()
    }
}
