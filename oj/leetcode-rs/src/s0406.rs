pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by(|x, y| {
            if x[0] == y[0] {
                x[1].cmp(&y[1])
            } else {
                y[0].cmp(&x[0])
            }
        });

        let mut result = Vec::with_capacity(people.len());
        for (_, p) in people.into_iter().enumerate() {
            let idx = p[1] as usize;
            if idx < result.len() {
                result.insert(idx, p);
            } else  {
                result.push(p);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ]
        );
    }
}
