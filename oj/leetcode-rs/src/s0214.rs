pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn fix_palindrome(s: &str, mut i: i32, mut j: i32) -> Option<String> {
        let bytes = s.as_bytes();
        let mut res = String::new();

        if i == j {
            res.push_str(&s[i as usize..(i + 1) as usize]);
            i -= 1;
            j += 1;
        }

        while i >= 0 || j < s.len() as i32 {
            if i >= 0 && j < s.len() as i32 {
                if bytes[i as usize] != bytes[j as usize] {
                    return None;
                }
                let i = i as usize;
                let j = j as usize;
                res.insert_str(0, &s[i..i + 1]);
                res.push_str(&s[j..j + 1]);
            } else if i >= 0 {
                let i = i as usize;
                res.insert_str(0, &s[i..i + 1]);
            } else {
                let j = j as usize;
                res.push_str(&s[j..j + 1]);
            }
            i -= 1;
            j += 1;
        }

        Some(res)
    }

    pub fn shortest_palindrome(s: String) -> String {
        // p = 2 / 2 = 1 [0, 1]
        // p = 3 / 2 = 1 [0, 1, 2]
        let p = s.len() / 2;
        let mut offset = 0;

        while p + (offset as usize) < s.len() {
            let p = p as i32;
            if let Some(res) = Self::fix_palindrome(&s, p - offset, p - offset) {
                return res;
            }
            if offset > 0 {
                if let Some(res) = Self::fix_palindrome(&s, p + offset, p + offset) {
                    return res;
                }
            }
            if p - offset - 1 >= 0 {
                if let Some(res) = Self::fix_palindrome(&s, p - offset - 1, p - offset) {
                    return res;
                }
            }

            if p + offset + 1 < s.len() as i32 {
                if let Some(res) = Self::fix_palindrome(&s, p + offset, p + offset + 1) {
                    return res;
                }
            }
            offset += 1;
        }

        "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_owned()),
            "aaacecaaa".to_owned()
        );
        assert_eq!(
            Solution::shortest_palindrome("abcd".to_owned()),
            "dcbabcd".to_owned()
        );
    }
}
