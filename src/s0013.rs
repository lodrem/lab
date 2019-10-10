pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut skipped = false;

        for (i, c) in s.chars().enumerate() {
            if skipped {
                skipped = false;
                continue;
            }

            let next = s.chars().nth(i + 1);

            match c {
                'I' if next == Some('V') => {
                    result += 4;
                    skipped = true;
                }
                'I' if next == Some('X') => {
                    result += 9;
                    skipped = true;
                }
                'I' => {
                    result += 1;
                }
                'V' => {
                    result += 5;
                }
                'X' if next == Some('L') => {
                    result += 40;
                    skipped = true;
                }
                'X' if next == Some('C') => {
                    result += 90;
                    skipped = true;
                }
                'X' => {
                    result += 10;
                }
                'L' => {
                    result += 50;
                }
                'C' if next == Some('D') => {
                    result += 400;
                    skipped = true;
                }
                'C' if next == Some('M') => {
                    result += 900;
                    skipped = true;
                }
                'C' => {
                    result += 100;
                }
                'D' => {
                    result += 500;
                }
                'M' => {
                    result += 1000;
                }
                _ => {}
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::roman_to_int("III".to_owned()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_owned()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_owned()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_owned()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_owned()), 1994);
    }
}
