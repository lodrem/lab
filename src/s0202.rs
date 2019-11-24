pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn square_of(digits: &[i32]) -> i32 {
        let mut result = 0;
        for digit in digits {
            result += digit * digit;
        }

        result
    }

    fn extract_digits(mut n: i32) -> Vec<i32> {
        let mut digits = vec![];

        while n > 0 {
            let s = n % 10;

            if s > 0 {
                digits.push(s as i32);
            }

            n /= 10;
        }

        digits
    }

    pub fn is_happy(mut n: i32) -> bool {
        use std::collections::HashSet;

        let mut squares = HashSet::new();

        while n > 0 {
            let digits = Self::extract_digits(n);
            let square = Self::square_of(&digits);
            if square == 1 {
                return true;
            }

            if squares.contains(&square) {
                return false;
            }
            n = square;
            squares.insert(square);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert!(Solution::is_happy(19));
        assert!(Solution::is_happy(1));
        assert!(!Solution::is_happy(2));
    }
}
