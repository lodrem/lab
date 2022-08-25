pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_by(|x, y| (x[0] - x[1]).cmp(&(y[0] - y[1])));

        let mut total = 0;

        let mid = costs.len() / 2;

        for i in 0..mid {
            total += costs[i][0] + costs[i + mid][1];
        }

        total
    }
}
