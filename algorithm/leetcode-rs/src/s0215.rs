pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        Self::find(&mut nums, k as usize)
    }

    fn find(coll: &mut [i32], k: usize) -> i32 {
        let len = coll.len();
        if len < 2 {
            return coll[0];
        }

        let p = Self::partition(coll);

        if k == p + 1 {
            coll[p]
        } else if k > p {
            Self::find(&mut coll[p + 1..], k - p - 1)
        } else {
            Self::find(&mut coll[..p], k)
        }
    }

    fn partition(coll: &mut [i32]) -> usize {
        let len = coll.len();
        let pivot_idx = len / 2;
        coll.swap(pivot_idx, len - 1);

        let mut store_idx = 0;

        for i in 0..len - 1 {
            if coll[i] > coll[len - 1] {
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
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }
}
