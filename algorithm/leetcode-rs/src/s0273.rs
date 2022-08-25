pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn convert_units(n: usize) -> String {
        match n {
            1 => "One",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eight",
            9 => "Nine",
            x => panic!("invalid number: {}", x),
        }
        .to_string()
    }

    fn covert_tens(n: usize) -> Vec<String> {
        let tens = n / 10;
        let units = n % 10;

        match tens {
            0 => vec![Self::convert_units(units)],
            1 => vec![match n {
                10 => "Ten",
                11 => "Eleven",
                12 => "Twelve",
                13 => "Thirteen",
                14 => "Fourteen",
                15 => "Fifteen",
                16 => "Sixteen",
                17 => "Seventeen",
                18 => "Eighteen",
                19 => "Nineteen",
                x => panic!("invalid number {}", x),
            }
            .to_string()],
            _ => {
                let tens = match tens {
                    2 => "Twenty",
                    3 => "Thirty",
                    4 => "Forty",
                    5 => "Fifty",
                    6 => "Sixty",
                    7 => "Seventy",
                    8 => "Eighty",
                    9 => "Ninety",
                    x => panic!("invalid number {}", x),
                }
                .to_string();

                if units == 0 {
                    vec![tens]
                } else {
                    let units = Self::convert_units(units);

                    vec![tens, units]
                }
            }
        }
    }

    fn convert_hundreds(n: usize) -> Vec<String> {
        let hundreds = n / 100;
        let tens = n % 100;

        let mut hundreds = match hundreds {
            x if x > 0 && x <= 9 => vec![Self::convert_units(x), "Hundred".to_string()],
            _ => vec![],
        };

        if tens > 0 {
            let tens = Self::covert_tens(tens);

            hundreds.extend(tens);
        }

        hundreds
    }

    pub fn number_to_words(mut num: i32) -> String {
        // 100 => one hundred
        // 1, 000 => one thousand
        // 1, 000, 000 => one million
        // 1, 000, 000, 000 => one billion

        if num == 0 {
            return "Zero".to_owned();
        }

        enum Level {
            Zero,
            Thousand,
            Million,
            Billion,
        }

        let mut level = Level::Zero;

        let mut words = vec![];

        while num > 0 {
            let n = num % 1000;
            num /= 1000;

            let mut hunreds = Self::convert_hundreds(n as usize);

            match level {
                Level::Zero => {
                    level = Level::Thousand;
                }
                Level::Thousand => {
                    if !hunreds.is_empty() {
                        hunreds.push("Thousand".to_string());
                    }
                    level = Level::Million;
                }
                Level::Million => {
                    if !hunreds.is_empty() {
                        hunreds.push("Million".to_string());
                    }
                    level = Level::Billion;
                }
                Level::Billion => {
                    if !hunreds.is_empty() {
                        hunreds.push("Billion".to_string());
                    }
                }
            }

            hunreds.extend(words);
            words = hunreds;
        }

        words.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::number_to_words(0), "Zero");

        assert_eq!(Solution::number_to_words(10), "Ten");

        assert_eq!(Solution::number_to_words(20), "Twenty");

        assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three");

        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five"
        );

        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );

        assert_eq!(
            Solution::number_to_words(1234567891),
            "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
        );
    }
}
