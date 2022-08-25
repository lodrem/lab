pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();

        let mut i = 0;
        let mut j = 0;

        let mut matched_idx = 0;
        let mut wildcard_idx = -1;

        while i < s.len() {
            if j < p.len() && (p[j] == b'?' || s[i] == p[j]) {
                i += 1;
                j += 1;
            } else if j < p.len() && p[j] == b'*' {
                wildcard_idx = j as i32;
                matched_idx = i;
                j += 1;
            } else if wildcard_idx != -1 {
                j = (wildcard_idx as usize) + 1;
                matched_idx += 1;
                i = matched_idx;
            } else {
                return false;
            }
        }

        while j < p.len() && p[j] == b'*' {
            j += 1;
        }

        j == p.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_match("aa".to_owned(), "a".to_owned()), false);
    }
}
