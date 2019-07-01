pub struct Solution;

impl Solution {

  #[allow(dead_code)]
  pub fn letter_combinations(digits: String) -> Vec<String> {
    use std::collections::HashMap;

    let mapping = {
      let mut mapping = HashMap::new();
      mapping.insert('2', "abc");
      mapping.insert('3', "def");
      mapping.insert('4', "ghi");
      mapping.insert('5', "jkl");
      mapping.insert('6', "mno");
      mapping.insert('7', "pqrs");
      mapping.insert('8', "tuv");
      mapping.insert('9', "wxyz");
      mapping
    };

    let mut result = vec![];

    for (_, digit) in digits.chars().enumerate() {
      let letters = mapping.get(&digit).unwrap();

      if result.is_empty() {
        for (_, letter) in letters.chars().enumerate() {
          result.push(letter.to_string());
        }
      } else {
        let mut next_result = Vec::with_capacity(letters.len() * result.len());

        for parent in result.drain(..) {
          for (_, letter) in letters.chars().enumerate() {
            let mut combination = parent.clone();
            combination.push(letter);
            next_result.push(combination);
          }
        }

        result = next_result;
      }
    }

    return result;
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    {
    }
  }
}