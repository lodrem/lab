pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn find(roots: &mut Vec<usize>, mut id: usize) -> usize {
        while roots[id] != id {
            roots[id] = roots[roots[id]];
            id = roots[id];
        }

        id
    }

    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut num_of_components = n as usize;
        let mut roots = vec![0; num_of_components];

        for i in 0..roots.len() {
            roots[i] = i;
        }

        for (_, edge) in edges.iter().enumerate() {
            let root1 = Self::find(&mut roots, edge[0] as usize);
            let root2 = Self::find(&mut roots, edge[1] as usize);

            if root1 != root2 {
                roots[root1] = roots[root2];
                num_of_components -= 1;
            }
        }

        num_of_components as i32
    }
}
