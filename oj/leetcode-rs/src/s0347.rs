pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut frequency_by_num = HashMap::new();

        for num in nums.into_iter() {
            *frequency_by_num.entry(num).or_insert(0) += 1;
        }

        let mut freq: Vec<(i32, usize)> =
            frequency_by_num.into_iter().map(|(k, v)| (k, v)).collect();

        freq.sort_by_key(|(_, f)| -(*f as i32));
        // Self::sort_top_k(&mut freq, k as usize);

        freq.into_iter()
            .take(k as usize)
            .map(|(num, _)| num)
            .collect()
    }

    fn sort_top_k(coll: &mut [(i32, usize)], k: usize) {
        let len = coll.len();
        if len < 2 {
            return;
        }

        let p = Self::partition(coll);

        if k > p {
            Self::sort_top_k(&mut coll[..p], p);
            Self::sort_top_k(&mut coll[p + 1..], k - p);
        } else {
            Self::sort_top_k(&mut coll[..p], p);
        }
    }

    fn partition(coll: &mut [(i32, usize)]) -> usize {
        let len = coll.len();
        let pivot_idx = len / 2;
        coll.swap(pivot_idx, len - 1);

        let mut store_idx = 0;
        for i in 0..len - 1 {
            if coll[i].1 > coll[len - 1].1 {
                coll.swap(i, store_idx);
                store_idx += 1;
            }
        }

        coll.swap(store_idx, len - 1);

        store_idx
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );

        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);

        assert_eq!(
            Solution::top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2),
            vec![2, -1]
        );
    }
}
