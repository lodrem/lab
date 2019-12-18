pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn find(roots: &mut Vec<i32>, mut id: i32) -> i32 {
        while roots[id as usize] != id {
            id = roots[id as usize];
        }

        id
    }

    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut roots = vec![-1; n as usize];
        for i in 0..roots.len() {
            roots[i] = i as i32;
        }

        for (_, edge) in edges.iter().enumerate() {
            let root1 = Self::find(&mut roots, edge[0]);
            let root2 = Self::find(&mut roots, edge[1]);

            if root1 == root2 {
                return false;
            }

            roots[root1 as usize] = roots[root2 as usize];
        }

        edges.len() == (n - 1) as usize
    }
}
