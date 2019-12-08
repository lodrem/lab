pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.is_empty() || t.is_empty() {
            return "".to_string();
        }

        let mut total = t.len();
        let mut counter = vec![0 as i32; 255];

        for c in t.chars().map(|c| c as usize) {
            counter[c] += 1;
        }

        let mut left = 0;
        let mut result = &s[..];
        let mut matched = false;
        let bytes = s.as_bytes();

        for (right, c) in s.bytes().enumerate() {
            let c = c as usize;

            if counter[c] > 0 {
                total -= 1;
            }
            counter[c] -= 1;

            while left < right + 1 && total == 0 {
                matched = true;
                if result.len() > right - left {
                    result = &s[left..right + 1];
                }

                let c = bytes[left] as usize;
                if counter[c] == 0 {
                    total += 1;
                }
                counter[c] += 1;

                left += 1;
            }
        }

        if matched {
            result.to_owned()
        } else {
            "".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_window("ab".to_owned(), "a".to_owned()),
            "a".to_owned()
        );

        assert_eq!(
            Solution::min_window("a".to_owned(), "a".to_owned()),
            "a".to_owned()
        );

        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned()),
            "BANC".to_owned()
        );

        assert_eq!(
            Solution::min_window("ABCDEFGHIJKLMN".to_owned(), "Z".to_owned()),
            "".to_owned()
        );
    }
}
