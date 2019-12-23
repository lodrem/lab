pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let chars = {
            let mut chars = vec![0; 26];

            for b in s.as_bytes().iter() {
                chars[(b - b'a') as usize] += 100;
            }

            for i in 0..chars.len() {
                chars[i] += i;
            }

            chars.sort();

            chars
        };

        let mut res = vec![b'z'; s.len()];

        let mut idx = 1;
        for i in 0..chars.len() {
            let count = chars[i] / 100;
            let c = b'a' + (chars[i] % 100) as u8;

            if count > (s.len() + 1) / 2 {
                return "".to_owned();
            }

            for _ in 0..count {
                if idx >= s.len() {
                    idx = 0;
                }
                res[idx] = c;
                idx += 2;
            }
        }

        unsafe { std::str::from_utf8_unchecked(&res).to_owned() }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reorganize_string("aab".to_owned()),
            "aba".to_owned()
        );
        assert_eq!(
            Solution::reorganize_string("vvvlo".to_owned()),
            "vlvov".to_owned()
        );
        assert_eq!(
            Solution::reorganize_string("aaab".to_owned()),
            "".to_owned()
        );
    }
}
