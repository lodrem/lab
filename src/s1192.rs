pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![vec![]; n as usize];
        for edge in connections {
            let (i, j) = (edge[0], edge[1]);
            graph[i as usize].push(j as usize);
            graph[j as usize].push(i as usize);
        }
        let mut disc = vec![-1; n as usize];
        let mut low = vec![-1; n as usize];
        let mut bridge = vec![];
        Self::dfs(
            0,
            0,
            0,
            &graph,
            &mut disc,
            &mut low,
            &mut bridge,
        );
        bridge
    }

    fn dfs(
        mut count: i32,
        v: usize,
        u: usize,
        graph: &Vec<Vec<usize>>,
        disc: &mut Vec<i32>,
        low: &mut Vec<i32>,
        bridge: &mut Vec<Vec<i32>>,
    ) {
        count += 1;
        disc[u] = count;
        low[u] = count;
        for &w in graph[u].iter() {
            if disc[w] == -1 {
                Self::dfs(count, u, w, graph, disc, low, bridge);
                if disc[w] == low[w] {
                    bridge.push(vec![u as i32, w as i32]);
                }
            }
            if w != v {
                low[u] = std::cmp::min(low[u], low[w]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![1, 3]],
            Solution::critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]])
        );
    }
}
