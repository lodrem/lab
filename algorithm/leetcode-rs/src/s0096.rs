pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut c: u128 = 1;

        for i in 0..n as u128 {
            c = c * 2 * (2 * i + 1) / (i + 2);
        }

        c as i32
    }
}
