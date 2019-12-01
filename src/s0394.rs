pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn decode(s: &str, idx: &mut usize) -> String {
        let mut res = String::default();

        let b = s.as_bytes();

        while *idx < s.len() && b[*idx] != b']' {
            if b[*idx] < b'0' || b[*idx] > b'9' {
                res.push_str(&s[*idx..*idx + 1]);
                *idx += 1;
            } else {
                let mut n = 0;
                while *idx < s.len() && b[*idx] >= b'0' && b[*idx] <= b'9' {
                    n = n * 10 + ((b[*idx] - b'0') as usize);
                    *idx += 1;
                }

                *idx += 1; // '['
                let t = Self::decode(s, idx);
                *idx += 1; // ']'

                res.push_str(&t.repeat(n));
            }
        }

        res
    }

    pub fn decode_string(s: String) -> String {
        let mut i = 0;
        return Self::decode(&s, &mut i);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!("", Solution::decode_string("".to_owned()));
        assert_eq!("aaabcbc", Solution::decode_string("3[a]2[bc]".to_owned()));
        assert_eq!("accaccacc", Solution::decode_string("3[a2[c]]".to_owned()));
        assert_eq!(
            "abcabccdcdcdef",
            Solution::decode_string("2[abc]3[cd]ef".to_owned())
        );
    }
}
