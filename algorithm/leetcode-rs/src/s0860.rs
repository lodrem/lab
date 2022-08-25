pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut num_five = 0;
        let mut num_ten = 0;

        for bill in bills.iter() {
            match bill {
                5 => num_five += 1,
                10 => {
                    num_ten += 1;
                    if num_five > 0 {
                        num_five -= 1;
                    } else {
                        return false;
                    }
                }
                _ => {
                    if num_ten > 0 && num_five > 0 {
                        num_ten -= 1;
                        num_five -= 1;
                    } else if num_five > 2 {
                        num_five -= 3;
                    } else {
                        return false;
                    }
                }
            };
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert!(Solution::lemonade_change(vec![
            5, 5, 5, 5, 10, 5, 10, 10, 10, 20
        ]));

        assert!(Solution::lemonade_change(vec![
            5, 5, 10, 10, 5, 20, 5, 10, 5, 5
        ]));
    }
}
