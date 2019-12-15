pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        Self::solve_by_counting(s)
    }

    fn solve_by_counting(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut max_len = 0;

        {
            // from left to right
            let mut left = 0;
            let mut right = 0;

            for i in 0..bytes.len() {
                match bytes[i] {
                    b'(' => left += 1,
                    _ => right += 1,
                }

                if left == right {
                    max_len = std::cmp::max(max_len, left + right);
                } else if right > left {
                    left = 0;
                    right = 0;
                }
            }
        }

        {
            // from right to left
            let mut left = 0;
            let mut right = 0;

            for i in (0..bytes.len()).rev() {
                match bytes[i] {
                    b'(' => left += 1,
                    _ => right += 1,
                }

                if left == right {
                    max_len = std::cmp::max(max_len, left + right);
                } else if right < left {
                    left = 0;
                    right = 0;
                }
            }
        }

        max_len
    }

    fn solve_by_stack(s: String) -> i32 {
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
