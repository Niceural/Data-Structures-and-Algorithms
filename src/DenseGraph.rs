extern crate queues;
use queues::*;

pub struct DenseGraph {
    number_of_vertices: usize,
    adjacency_matrix: Vec<Option<i32>>,
}

impl DenseGraph {

    // ================================================
    //                                  graph functions
    // ================================================

    pub fn new(num_vertices: usize) -> DenseGraph {
        DenseGraph {
            number_of_vertices: num_vertices,
            adjacency_matrix: vec![None; num_vertices * num_vertices]
        }
    }

    fn _2D_to_1D(&self, row: usize, col: usize) -> usize {
        self.number_of_vertices * row + col
    }

    pub fn connect(&mut self, start: usize, end: usize, weight: i32) {
        let id = self._2D_to_1D(start, end);
        self.adjacency_matrix[id] = Some(weight);
    }

    // ================================================
    //                             breadth first search
    // ================================================

    fn _reconstruct_path(&self, start: usize, end: usize, prev: Vec<Option<usize>>) {
        let mut path: Vec<usize> = Vec::new();
        let mut id = end;
        loop {
        }
    }

    pub fn bfs_connected(&self, start: usize, end: usize) -> bool {
        if start == end { return true; }

        let mut q: Queue<usize> = Queue::new();
        q.add(start).expect("Failed to add to queue while executing bfs");

        let mut visited: Vec<bool> = vec![false; self.number_of_vertices];
        visited[start] = true;

        loop {
            let node = match q.remove() {
                Err(_) => return false,
                Ok(n) => n,
            };
            for id in 0..self.number_of_vertices  {
                match self.adjacency_matrix[self._2D_to_1D(node, id)] {
                    Some(_) => {
                        if !visited[id] {
                            if id == end { return true; }
                            q.add(id).expect("Failed to add to queue while executing bfs");
                            visited[id] = true;
                        }
                    },
                    None => (),
                }
            } // neighbours
        } // queue

    }

    pub fn bfs_shortest_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut q: Queue<usize> = Queue::new();
        q.add(start).expect("Failed to add to the queue while executing bfs");

        let mut visited: Vec<bool> = vec![false; self.number_of_vertices];
        visited[start] = true;

        let mut prev: Vec<Option<usize>> = vec![None; self.number_of_vertices];
        loop {
            let node = match q.remove() {
                Err(_) => break,
                Ok(n) => n,
            };
            for id in 0..self.number_of_vertices  {
                let vec_id: usize = self._2D_to_1D(node, id);
                match self.adjacency_matrix[vec_id] {
                    Some(_) => {
                        if !visited[id] {
                            q.add(id).expect("Failed to add to the queue while executing bfs");
                            visited[id] = true;
                            prev[id] = Some(node);
                        }
                    },
                    None => (),
                }
            } // neighbours
        } // queue

        let mut path: Vec<usize> = Vec::new();
        let mut at = end;
        loop {
            path.push(at);
            at = match prev[at] {
                Some(n) => n,
                None => break,
            }
        }
        path.reverse();

        if path[0] == start { return Some(path); }
        return None;
    }
}
