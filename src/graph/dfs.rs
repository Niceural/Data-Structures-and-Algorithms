use crate::graph::weighted_adjacency_list::{Edge, WeightedAdjacencyList};

impl WeightedAdjacencyList {

    pub fn dfs_iterative(&self, start: usize) -> usize {
        let mut count = 0;
        let mut visited: Vec<bool> = vec![false; self.vertex_count()];
        let mut stack= Vec::new();

        stack.push(start);
        visited[start] = true;

        while let Some(vertex) = stack.pop() {
            count += 1;
            let neighbours = &self[vertex];
            for &Edge { to, weight: _ } in neighbours {
                if !visited[to] {
                    stack.push(to);
                    visited[to] = true;
                }
            }
        }
        count
    }

    pub fn dfs_recursive(&self, start: usize) -> usize {

        fn _dfs(
            graph: &WeightedAdjacencyList,
            vertex: usize,
            visited: &mut [bool],
            count: &mut usize,
        ) {
            *count += 1;
            visited[vertex] = true;
            for &Edge { to, weight: _ } in &graph[vertex] {
                if !visited[to] {
                    _dfs(graph, to, visited, count);
                }
            }
        }

        let mut count = 0;
        let mut visited = vec![false; self.vertex_count()];
        _dfs(self, start, &mut visited, &mut count);

        count
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_dfs_iterative() {
        let graph = WeightedAdjacencyList::new_directed(
            5,
            &[
                (0, 1, 4.),
                (0, 2, 5.),
                (1, 2, -2.),
                (1, 3, 6.),
                (2, 3, 1.),
                (2, 2, 10.),
            ],
        );

        let count = graph.dfs_iterative(0);
        assert_eq!(count, 4);

        let count = graph.dfs_iterative(4);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_dfs_recursive() {
        const N: usize = 5;
        let graph = WeightedAdjacencyList::new_directed_unweighted(
            N,
            &[
                [0, 1],
                [0, 2],
                [1, 2],
                [1, 3],
                [2, 3],
                [2, 2],
            ]
        );

        let count = graph.dfs_recursive(0);
        assert_eq!(count, 4);

        let count = graph.dfs_recursive(4);
        assert_eq!(count, 1);
    }
}