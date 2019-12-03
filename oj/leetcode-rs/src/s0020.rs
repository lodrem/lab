pub struct Solution;

impl Solution {
    fn is_open_bracket(c: char) -> bool {
        c == '(' || c == '{' || c == '['
    }

    fn get_opposite_bracket(c: char) -> char {
        match c {
            ')' => '(',
            '(' => ')',
            ']' => '[',
            '[' => ']',
            '{' => '}',
            '}' => '{',
            _ => panic!("unknown bracket"),
        }
    }

    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        let mut open_stack = Vec::with_capacity(s.len());

        for (_, bracket) in s.chars().enumerate() {
            if Self::is_open_bracket(bracket) {
                open_stack.push(bracket);
            } else {
                match open_stack.pop() {
                    Some(open_bracket) => {
                        if open_bracket != Self::get_opposite_bracket(bracket) {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
        }

        open_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        assert!(Solution::is_valid("()".to_string()));
        assert!(Solution::is_valid("()[]{}".to_string()));
        assert!(!Solution::is_valid("(]".to_string()));
        assert!(!Solution::is_valid("([)]".to_string()));
        assert!(Solution::is_valid("{[]}".to_string()));
        assert!(!Solution::is_valid("{".to_string()))
    }
}
