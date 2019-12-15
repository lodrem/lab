pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut sorted_words = words.clone();

        let orders = {
            let mut orders = vec![0; 26];

            let bytes = order.as_bytes();
            for i in 0..bytes.len() {
                orders[(bytes[i] - b'a') as usize] = i;
            }

            orders
        };

        sorted_words.sort_by(|x, y| {
            let xs = x.as_bytes();
            let ys = y.as_bytes();

            let mut i = 0;

            loop {
                if xs.len() <= i && ys.len() <= i {
                    return std::cmp::Ordering::Equal;
                }

                if xs.len() <= i {
                    return std::cmp::Ordering::Less;
                }

                if ys.len() <= i {
                    return std::cmp::Ordering::Greater;
                }

                let x = orders[(xs[i] - b'a') as usize];
                let y = orders[(ys[i] - b'a') as usize];

                if x < y {
                    return std::cmp::Ordering::Less;
                } else if x > y {
                    return std::cmp::Ordering::Greater;
                }

                i += 1;
            }
        });

        words == sorted_words
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        assert!(Solution::is_alien_sorted(
            vec!["hello".to_owned(), "leetcode".to_owned()],
            "hlabcdefgijkmnopqrstuvwxyz".to_owned()
        ));

        assert!(!Solution::is_alien_sorted(
            vec!["word".to_owned(), "world".to_owned(), "row".to_owned()],
            "worldabcefghijkmnpqstuvxyz".to_owned()
        ));

        assert!(!Solution::is_alien_sorted(
            vec!["apple".to_owned(), "app".to_owned()],
            "abcdefghijklmnopqrstuvwxyz".to_owned()
        ));
    }
}
