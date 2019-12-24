pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|x, y| x[0].cmp(&y[0]));

        let mut count = 0;
        let mut prev_idx = 0;

        for i in 1..intervals.len() {
            let prev = &intervals[prev_idx];
            let cur = &intervals[i];
            if prev[1] > cur[0] {
                if prev[1] > cur[1] {
                    prev_idx = i;
                }
                count += 1;
            } else {
                prev_idx = i;
            }
        }
        count
    }
}
