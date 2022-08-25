pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let bytes = s.as_bytes();
        let mut last_idx = vec![0; 26];
        for i in 0..bytes.len() {
            last_idx[(bytes[i] - b'a') as usize] = i;
        }

        let mut s = Vec::default();
        let mut is_contained = vec![false; 26];

        for i in 0..bytes.len() {
            let c = bytes[i];

            let idx = (c - b'a') as usize;

            if !is_contained[idx] {
                // remove stack peek which
                // 1. greater than current character
                // 2. will show later on
                while !s.is_empty()
                    && c < s[s.len() - 1]
                    && last_idx[(s[s.len() - 1] - b'a') as usize] > i
                {
                    let peek = s.pop().unwrap();
                    is_contained[(peek - b'a') as usize] = false;
                }

                is_contained[idx] = true;
                s.push(c);
            }
        }

        unsafe { std::str::from_utf8_unchecked(&s).to_owned() }
    }
}
