pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        use std::collections::VecDeque;

        let bytes = s.as_bytes();
        let mut stack = VecDeque::new();

        let mut max_len = 0;

        stack.push_back(-1);

        for i in 0..bytes.len() {
            match bytes[i] {
                b'(' => {
                    stack.push_back(i as i32);
                }
                _ => {
                    stack.pop_back();

                    if stack.is_empty() {
                        stack.push_back(i as i32);
                    } else {
                        let start_idx = stack[stack.len() - 1];

                        max_len = std::cmp::max(max_len, i as i32 - start_idx);
                    }
                }
            }
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::longest_valid_parentheses("()()".to_owned()), 4);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_owned()), 2);
        assert_eq!(Solution::longest_valid_parentheses("(()".to_owned()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_owned()), 4);
    }
}
