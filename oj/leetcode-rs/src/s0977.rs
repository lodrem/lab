pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        if a.is_empty() {
            return a;
        }
        let mut res = vec![0; a.len()];

        let mut i = 0;
        let mut j = a.len() - 1;
        let mut k = a.len() - 1;
        while i < j {
            let x = a[i] * a[i];
            let y = a[j] * a[j];

            res[k] = if x > y {
                i += 1;
                x
            } else {
                j -= 1;
                y
            };

            k -= 1;
        }

        res[k] = a[i] * a[i];

        res
    }
}
