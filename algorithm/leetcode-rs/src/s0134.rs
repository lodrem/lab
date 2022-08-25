pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();

        let mut total = 0;
        let mut cur = 0;
        let mut idx = 0;

        for i in 0..n {
            total += gas[i] - cost[i];
            cur += gas[i] - cost[i];

            if cur < 0 {
                idx = i + 1;
                cur = 0;
            }
        }

        if total >= 0 {
            idx as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1]),
            4
        );
    }
}
