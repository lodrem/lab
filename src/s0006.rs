pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut rows: Vec<String> = vec![String::with_capacity(s.capacity()); num_rows as usize];
        let mut go_down = false;
        let mut cur_row = 0;

        for (_, c) in s.chars().enumerate() {
            rows[cur_row].push(c);
            if cur_row == 0 || cur_row == num_rows as usize - 1 {
                go_down = !go_down
            }

            if go_down {
                cur_row += 1;
            } else {
                cur_row -= 1;
            }
        }

        rows.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_owned(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_owned(), 4),
            "PINALSIGYAHRPI"
        );
    }
}
