pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {

        if s.is_empty() {
            return true
        }

        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut s_idx = 0;

        for i in 0..t.len() {
            if t[i] == s[s_idx] {
                s_idx += 1;

                if s_idx >= s.len() {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert!(Solution::is_subsequence(
            "abc".to_owned(),
            "ahbgdc".to_owned()
        ));
        assert!(!Solution::is_subsequence(
            "axc".to_owned(),
            "ahbgdc".to_owned()
        ));
    }
}
