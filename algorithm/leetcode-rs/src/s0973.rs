pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        Self::sort(&mut points, k as usize);

        points[..k as usize].to_vec()
    }

    fn sort(coll: &mut [Vec<i32>], k: usize) {
        let len = coll.len();

        if len < 2 {
            return;
        }

        // quick sort
        let p = Self::partitoin(coll);

        if p >= k {
            Self::sort(&mut coll[0..p], k);
        } else {
            Self::sort(&mut coll[0..p], p);
            Self::sort(&mut coll[p + 1..len], k - p);
        }
    }

    fn partitoin(coll: &mut [Vec<i32>]) -> usize {
        let len = coll.len();
        let pivot_idx = len / 2;
        coll.swap(pivot_idx, len - 1);

        let mut store_idx = 0;
        for i in 0..len - 1 {
            if Self::get_distance(&coll[i]) < Self::get_distance(&coll[len - 1]) {
                coll.swap(i, store_idx);
                store_idx += 1;
            }
        }

        coll.swap(store_idx, len - 1);

        store_idx
    }

    fn get_distance(point: &[i32]) -> i32 {
        point[0] * point[0] + point[1] * point[1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![3, 3], vec![-2, 4]]
        )
    }
}
