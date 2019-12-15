pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
        let is_digit = |c: u8| -> bool { c >= b'0' && c <= b'9' };

        logs.sort_by(|x: &String, y: &String| {
            let xs: Vec<&str> = x.splitn(2, " ").collect();
            let ys: Vec<&str> = y.splitn(2, " ").collect();

            let is_x_digit = is_digit(xs[1].as_bytes()[0]);
            let is_y_digit = is_digit(ys[1].as_bytes()[0]);

            if is_x_digit && is_y_digit {
                std::cmp::Ordering::Equal
            } else if is_x_digit {
                std::cmp::Ordering::Greater
            } else if is_y_digit {
                std::cmp::Ordering::Less
            } else {
                match xs[1].cmp(ys[1]) {
                    std::cmp::Ordering::Equal => xs[0].cmp(ys[0]),
                    order => order,
                }
            }
        });

        logs
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reorder_log_files(vec![
                "dig1 8 1 5 1".to_owned(),
                "let1 art can".to_owned(),
                "dig2 3 6".to_owned(),
                "let2 own kit dig".to_owned(),
                "let3 art zero".to_owned(),
            ]),
            vec![
                "let1 art can".to_owned(),
                "let3 art zero".to_owned(),
                "let2 own kit dig".to_owned(),
                "dig1 8 1 5 1".to_owned(),
                "dig2 3 6".to_owned(),
            ]
        );

        assert_eq!(
            Solution::reorder_log_files(vec![
                "a1 9 2 3 1".to_owned(),
                "g1 act car".to_owned(),
                "zo4 4 7".to_owned(),
                "ab1 off key dog".to_owned(),
                "a8 act zoo".to_owned(),
                "a2 act car".to_owned(),
            ]),
            vec![
                "a2 act car".to_owned(),
                "g1 act car".to_owned(),
                "a8 act zoo".to_owned(),
                "ab1 off key dog".to_owned(),
                "a1 9 2 3 1".to_owned(),
                "zo4 4 7".to_owned(),
            ]
        );
    }
}
