use crate::graph::{Edge, WeightedAdjacencyList};

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
            for &neighbour in &graph[vertex] {
                if !visited[neighbour] {
                    _dfs(graph, neighbour, visited, count);
                }
            }
        }

        let mut count = 0;
        let mut visited = vec![false; self.vertex_count()];
        _dfs(self, start, &mut visited, &mut count);

        count
    }
}