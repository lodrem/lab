pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum_less_than_k(a: Vec<i32>, k: i32) -> i32 {
        let mut max = -1;
        for i in 0..a.len() {
            let x = a[i];

            if x > k {
                continue;
            }

            for j in i + 1..a.len() {
                let sum = x + a[j];
                if sum > max && sum < k {
                    max = sum;
                }
            }
        }
        max
    }
}
