pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut n = 0;
        let mut i = g.len() as i32 - 1;
        let mut j = s.len() as i32 - 1;

        while i >= 0 && j >= 0 {
            if s[j as usize] >= g[i as usize] {
                n += 1;
                j -= 1;
            }
            i -= 1;
        }

        n
    }
}
