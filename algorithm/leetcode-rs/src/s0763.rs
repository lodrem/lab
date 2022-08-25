pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let bytes = s.as_bytes();

        let mut rightmost = vec![std::i32::MIN; 26];

        for i in 0..s.len() {
            let c = (bytes[i] - b'a') as usize;
            let i = i as i32;
            if rightmost[c] < i {
                rightmost[c] = i;
            }
        }

        let mut partitions = vec![];

        let mut last_idx = 0;
        let mut farest_idx = 0;

        for i in 0..s.len() {
            let c = (bytes[i] - b'a') as usize;

            farest_idx = std::cmp::max(farest_idx, rightmost[c] as usize);

            if i == farest_idx {
                partitions.push((1 + farest_idx - last_idx) as i32);
                last_idx = farest_idx + 1;
            }
        }

        partitions
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_owned()),
            vec![9, 7, 8]
        );
    }
}
