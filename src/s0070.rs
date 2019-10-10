pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut i = 1;
        let mut j = 2;

        for _ in 3..n {
            let x = i + j;
            i = j;
            j = x;
        }

        j
    }
}
