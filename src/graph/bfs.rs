use crate::graph::weighted_adjacency_list::{Edge, WeightedAdjacencyList};
use std::collections::VecDeque;

impl WeightedAdjacencyList {
    pub fn bfs(&self, start: usize, end: usize) -> (usize, Option<Vec<usize>>) {
        let (depth, prev) = self._bfs(start);

        let path = self._path_to(end, prev);

        if path[0] == start {
            return (depth, Some(path));
        }
        (depth, None)
    }

    fn _bfs(&self, start: usize) -> (usize, Vec<Option<usize>>) {
        let n = self.vertex_count();
        let mut depth = 0;
        let mut queue = VecDeque::with_capacity(n);
        let mut visited = vec![false; n];
        let mut prev = vec![None; n];
        const DEPTH_TOKEN: usize = usize::MAX;

        queue.push_back(start);
        queue.push_back(DEPTH_TOKEN);
        visited[start] = true;

        while let Some(vertex) = queue.pop_front() {
            if queue.is_empty() {
                break;
            }
            if vertex == DEPTH_TOKEN {
                queue.push_back(DEPTH_TOKEN);
                depth += 1;
                continue;
            } else {
                let neighbours = &self[vertex];
                for &Edge { to, weight: _ } in neighbours {
                    if !visited[to] {
                        visited[to] = true;
                        prev[to] = Some(vertex);
                        queue.push_back(to);
                    }
                }
            }
        }
        (depth, prev)
    }

    fn _path_to(&self, end: usize, prev: Vec<Option<usize>>) -> Vec<usize> {
        let mut path = Vec::new();
        let mut at = end;

        while let Some(parent) = prev[at] {
            at = parent;
            path.push(at);
        }

        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bfs() {
        let graph = WeightedAdjacencyList::new_undirected(
            15, 
            &[
                (0, 7, 1.),
                (0, 9, 1.),
                (0, 11, 1.),
                (7, 11, 1.),
                (7, 6, 1.),
                (7, 3, 1.),
                (6, 5, 1.),
                (3, 4, 1.),
                (2, 3, 1.),
                (2, 12, 1.),
                (12, 8, 1.),
                (8, 1, 1.),
                (1, 10, 1.),
                (10, 9, 1.),
                (9, 8, 1.),
            ],
        );

        let (start, end) = (10, 5);
        let (depth, path) = graph.bfs(start, end);
        assert_eq!(depth, 5);
        assert_ne!(path, None);
    }
}