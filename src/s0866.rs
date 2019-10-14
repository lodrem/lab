pub struct Solution;

impl Solution {
    fn is_prime(num: i32) -> bool {
        if num <= 3 {
            return num > 1;
        } else if num % 2 == 0 || num % 3 == 0 {
            return false;
        }

        let mut i = 5;

        while i * i <= num {
            if num % i == 0 || num % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }

        return true;
    }

    fn is_palindrome(num: i32) -> bool {
        let mut n = num;
        let mut reverse = 0;

        while n > 0 {
            reverse = reverse * 10 + n % 10;
            n /= 10;
        }
        return reverse == num;
    }

    #[allow(dead_code)]
    pub fn prime_palindrome(mut n: i32) -> i32 {
        loop {
            if Self::is_palindrome(n) && Self::is_prime(n) {
                return n;
            }

            if n < 10 {
                n += 1;
            } else if n % 2 == 0 {
                n += 1;
            } else {
                n += 2;
            }

            if n > 10000000 && n < 100000000 {
                n = 100000001;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(7, Solution::prime_palindrome(6));
        assert_eq!(7, Solution::prime_palindrome(7));
        assert_eq!(11, Solution::prime_palindrome(8));
        assert_eq!(101, Solution::prime_palindrome(13));
    }
}
