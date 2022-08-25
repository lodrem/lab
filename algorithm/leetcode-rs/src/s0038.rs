pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = vec!['1'];

        for _ in 1..n as usize {
            let mut nums = String::default();
            let mut count = 0;
            let mut num = '1';

            for i in 0..s.len() {
                if s[i] == num {
                    count += 1;
                } else {
                    if count > 0 {
                        nums.push_str(&format!("{}{}", count, num));
                    }
                    count = 1;
                    num = s[i];
                }
            }
            if count > 0 {
                nums.push_str(&format!("{}{}", count, num));
            }

            s = nums.chars().collect()
        }

        s.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(2), "11");
        assert_eq!(Solution::count_and_say(3), "21");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
        assert_eq!(Solution::count_and_say(6), "312211");
    }
}
