pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() {
            return
        }
        let mut i = 0;
        let n = s.len();
        while i < n - i - 1 {
            s.swap(i, n - i - 1);
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        {
            let mut nums = vec!['h', 'e', 'l', 'l', 'o'];
            Solution::reverse_string(&mut nums);
            assert_eq!(nums, vec!['o', 'l', 'l', 'e', 'h']);
        }

        {
            let mut nums = vec!['H', 'a', 'n', 'n', 'a', 'h'];
            Solution::reverse_string(&mut nums);
            assert_eq!(nums, vec!['h', 'a', 'n', 'n', 'a', 'H']);
        }
    }
}
