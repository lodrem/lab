pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let total_tasks = tasks.len();
        let tasks = {
            let mut t = vec![0; 26];
            for task in tasks.iter() {
                t[(*task as u8 - b'A') as usize] += 1;
            }
            t.sort();
            t
        };

        let mut idx = 25;
        while idx > 0 && tasks[idx] == tasks[25] {
            idx -= 1;
        }

        std::cmp::max(
            total_tasks as i32,
            ((tasks[25] - 1) * (n + 1)) + (25 - idx) as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
    }
}
