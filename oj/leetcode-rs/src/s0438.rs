pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s = s.into_bytes();
        let p = p.into_bytes();

        if s.len() < p.len() {
            return vec![];
        }

        let required_len = p.len();
        let mut required = vec![0; 26];
        let mut total_required = 0;
        for c in p {
            required[(c - b'a') as usize] += 1;
            total_required += 1;
        }

        let mut res = vec![];
        let mut start_idx = 0;
        let mut end_idx = 0;

        while end_idx < s.len() {
            let r = &mut required[(s[end_idx] - b'a') as usize];
            *r -= 1;
            if *r >= 0 {
                total_required -= 1;
            }
            end_idx += 1;

            while total_required == 0 {
                let r = &mut required[(s[start_idx] - b'a') as usize];

                *r += 1;
                if *r > 0 {
                    total_required += 1;
                }

                if end_idx - start_idx == required_len {
                    res.push(start_idx as i32);
                }
                start_idx += 1;
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
            Solution::find_anagrams("baa".to_owned(), "aa".to_owned()),
            vec![1]
        );

        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_owned(), "abc".to_owned()),
            vec![0, 6]
        );

        assert_eq!(
            Solution::find_anagrams(
                "eklpyqrbgjdwtcaxzsnifvhmoueklpyqrbgjdw".to_owned(),
                "yqrbgjdwtcaxzsnifvhmou".to_owned()
            ),
            vec![4]
        );
    }
}
