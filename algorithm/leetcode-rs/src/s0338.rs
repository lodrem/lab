pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        // f[i] = f[i >> 1] + i & 1

        let mut res = vec![0; 1 + num as usize];

        for i in 0..res.len() {
            res[i] = res[i >> 1] + (i & 1) as i32;
        }

        res
    }
}
