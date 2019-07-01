pub struct Solution;

impl Solution {
  #[allow(dead_code)]
  pub fn reverse(mut x: i32) -> i32 {
    if x == std::i32::MIN {
      return 0;
    } else if x < 0 {
      return -Self::reverse(-x);
    }

    let mut res = 0;

    while x != 0 {
      if res > std::i32::MAX / 10 || 10 * res > std::i32::MAX - x % 10 {
        return 0;
      }

      res = res * 10 + x % 10;

      x /= 10;
    }

    res
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(-120), -21);
    assert_eq!(Solution::reverse(std::i32::MAX), 0);
  }
}
