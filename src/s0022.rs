pub struct Solution;

impl Solution {
  #[allow(dead_code)]
  pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = Vec::new();

    if n == 0 {
      res.push("".to_owned())
    } else {
      for i in 0..n {
        for left in Self::generate_parenthesis(i) {
          for right in Self::generate_parenthesis(n - 1 - i) {
            res.push(format!("({}){}", left, right))
          }
        }
      }
    }

    res
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn it_works() {
    use std::collections::HashSet;

    {
      let actual = Solution::generate_parenthesis(3);

      let expected: Vec<_> = vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        .into_iter()
        .map(|x| String::from(x))
        .collect();

      assert_eq!(actual.len(), 5);

      {
        let x: HashSet<_> = actual.into_iter().collect();
        let y: HashSet<_> = expected.into_iter().collect();

        assert_eq!(x, y);
      }
    }
  }
}
