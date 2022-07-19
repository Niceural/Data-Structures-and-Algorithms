#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub to: usize,
    pub weight: f64,
}

impl Edge {
    pub fn new(to: usize, weight: f64) -> Self {
        Self { to, weight }
    }
}

#[derive(Debug)]
pub struct WeightedAdjacencyList {
    list: Vec<Vec<Edge>>,
}
impl WeightedAdjacencyList {
    /// Initializes an empty adjacency list that can hold up to `n` vertices.
    pub fn new_with_size(n: usize) -> Self {
        Self { list: vec![vec![]; n] }
    }
    /// Returns true if the graph has no vertices.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
    /// Add a directed edge from vertex `u` to vertex `v` with weight `weight`.
    pub fn add_directed_edge(&mut self, u: usize, v: usize, weight: f64) {
        self.list[u].push(Edge::new(v, weight))
    }
    /// Add an undirected edge between vertices `u` and `v`.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize, weight: f64) {
        self.add_directed_edge(u, v, weight);
        self.add_directed_edge(v, u, weight);
    }
    /// Creates a new directed graph with edges `edges`
    pub fn new_directed(size: usize, edges: &[(usize, usize, f64)]) -> Self {
        let mut graph = Self::new_with_size(size);
        for &(a, b, c) in edges.iter() {
            graph.add_directed_edge(a, b, c);
        }
        graph
    }
    /// Creates a new undirected graph with edges `edges`
    pub fn new_undirected(size: usize, edges: &[(usize, usize, f64)]) -> Self {
        let mut graph = Self::new_with_size(size);
        for &(a, b, c) in edges.iter() {
            graph.add_undirected_edge(a, b, c);
        }
        graph
    }
    pub fn new_directed_unweighted(size: usize, edges: &[[usize; 2]]) -> Self {
        let mut graph = Self::new_with_size(size);
        for &[a, b] in edges.iter() {
            graph.add_directed_edge(a, b, 1.);
        }
        graph
    }
    pub fn new_undirected_unweighted(size: usize, edges: &[[usize; 2]]) -> Self {
        let mut graph = Self::new_with_size(size);
        for &[a, b] in edges.iter() {
            graph.add_undirected_edge(a, b, 1.);
        }
        graph
    }
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize, f64)> + '_ {
        self.list
            .iter()
            .enumerate()
            .flat_map(|(a, edges)| edges.iter().map(move |b| (a, b.to, b.weight)))
    }
    pub fn edge_count(&self) -> usize {
        self.edges().count()
    }
    pub fn vertices(&self) -> impl Iterator<Item = (usize, &Vec<Edge>)> {
        self.list.iter().enumerate()
    }
    pub fn vertex_count(&self) -> usize {
        self.list.len()
    }
}

impl std::ops::Index<usize> for WeightedAdjacencyList {
    type Output = Vec<Edge>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}
