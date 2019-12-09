pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn brute_force(s: String) -> String {
        let rev: String = s.chars().rev().collect();

        // 1. find **prefix** longest palindrome string slice
        // => is_palindrome(s[0..i])
        // => s[0..i] == rev(s[0..i])
        // => s[0..i] == rev[n-i..]
        // 2. shortest palindrome
        // => rev(s[i..n]) + s
        // => rev[0..i] + s
        for i in 0..s.len() {
            if &s[0..s.len() - i] == &rev[i..] {
                let mut res = rev[0..i].to_owned();
                res.push_str(&s);
                return res;
            }
        }
        "".to_owned()
    }

    fn kmp(s: String) -> String {
        let rev: String = s.chars().rev().collect();

        // Build longest palindrome with magic char '#'
        let c = {
            let mut c = String::with_capacity(s.len() * 2 + 1);
            c.push_str(&s);
            c.push('#');
            c.push_str(&rev);
            c
        };
        let b = c.as_bytes();
        let mut f = vec![0; c.len()];

        for i in 1..c.len() {
            let mut t = f[i - 1];
            while t > 0 && b[i] != b[t] {
                t = f[t - 1];
            }
            if b[i] == b[t] {
                t += 1;
            }
            f[i] = t;
        }

        let mut res = rev[0..s.len() - f[c.len() - 1]].to_owned();
        res.push_str(&s);

        res
    }

    pub fn shortest_palindrome(s: String) -> String {
        // Self::brute_force(s)
        Self::kmp(s)
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
