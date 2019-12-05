pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        use std::collections::VecDeque;

        let mut max_area = 0;

        let mut s = VecDeque::with_capacity(heights.len());

        s.push_back(-1);

        for i in 0..heights.len() {
            loop {
                let height_idx = match s.back() {
                    Some(idx) if *idx != -1 && heights[*idx as usize] > heights[i] => *idx as usize,
                    _ => break,
                };

                s.pop_back();

                let leftmost_idx = s.back().unwrap();
                let rightmost_idx = i as i32;
                let width = (rightmost_idx - leftmost_idx - 1) as usize;

                max_area = std::cmp::max(max_area, heights[height_idx] as usize * width);
            }
            s.push_back(i as i32);
        }

        while s.len() > 1 {
            let idx = s.pop_back().unwrap() as usize;
            let leftmost_idx = s.back().unwrap();
            let rightmost_idx = heights.len() as i32;
            let width = (rightmost_idx - leftmost_idx - 1) as usize;

            max_area = std::cmp::max(max_area, heights[idx] as usize * width);
        }

        max_area as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
        assert_eq!(1, Solution::largest_rectangle_area(vec![1]));
        assert_eq!(2, Solution::largest_rectangle_area(vec![1, 1]));
        assert_eq!(3, Solution::largest_rectangle_area(vec![2, 1, 2]));
    }
}
