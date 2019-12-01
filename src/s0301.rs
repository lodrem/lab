pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn search(
        s: &str,
        res: &mut Vec<String>,
        start_to_count_idx: usize,
        start_to_remove_idx: usize,
        pair: (char, char),
    ) {
        let mut stack = 0;

        for i in start_to_count_idx..s.len() {
            match s.chars().nth(i).unwrap() {
                c if c == pair.0 => stack += 1,
                c if c == pair.1 => stack -= 1,
                _ => {}
            }

            if stack >= 0 {
                continue;
            }

            // with extra pair.1

            for j in start_to_remove_idx..i+1 {
                if s.chars().nth(j).unwrap() == pair.1
                    && (j == start_to_remove_idx || s.chars().nth(j - 1).unwrap() != pair.1)
                {
                    Self::search(&[&s[0..j], &s[j + 1..s.len()]].concat(), res, i, j, pair);
                }
            }

            return;
        }

        let reversed = s.chars().rev().collect::<String>();
        if pair.0 == '(' {
            Self::search(&reversed, res, 0, 0, (')', '('));
        } else {
            res.push(reversed);
        }
    }

    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut res = vec![];

        Self::search(&s, &mut res, 0, 0, ('(', ')'));

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![""],
            Solution::remove_invalid_parentheses("(".to_owned())
        );
        assert_eq!(
            vec!["(())()", "()()()"],
            Solution::remove_invalid_parentheses("()())()".to_owned())
        );
        assert_eq!(
            vec!["(a())()", "(a)()()"],
            Solution::remove_invalid_parentheses("(a)())()".to_owned())
        );
        assert_eq!(
            vec!["x"],
            Solution::remove_invalid_parentheses("x".to_owned())
        );
        assert_eq!(
            vec!["x"],
            Solution::remove_invalid_parentheses("(x".to_owned())
        );
        assert_eq!(
            vec!["x"],
            Solution::remove_invalid_parentheses(")x".to_owned())
        );
        assert_eq!(
            vec!["x"],
            Solution::remove_invalid_parentheses("x)".to_owned())
        );
        assert_eq!(
            vec!["x"],
            Solution::remove_invalid_parentheses("x(".to_owned())
        );
        assert_eq!(
            vec!["(x)"],
            Solution::remove_invalid_parentheses("(x)".to_owned())
        );
        assert_eq!(
            vec!["()"],
            Solution::remove_invalid_parentheses("()".to_owned())
        );
        assert_eq!(
            vec!["()"],
            Solution::remove_invalid_parentheses("())".to_owned())
        );

        assert_eq!(
            vec![""],
            Solution::remove_invalid_parentheses(")(".to_owned())
        );
    }
}
