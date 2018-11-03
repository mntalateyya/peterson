use std::{
    collections::{
        HashMap,
        hash_map::RandomState,
    },
    hash::{Hash, BuildHasher},
};

#[cfg(test)]
mod test {
    #[test]
    fn test_undirected() {
        use super::*;
        let mut g: Graph<i32,(),_> = Graph::new(false);
        g.add_edge((0, 1), ());
        g.add_edge((1, 2), ());
        g.add_edge((2, 3), ());
        g.add_edge((3, 1), ());
        assert_eq!(g.find_edge((0, 1)), Some(&()));
        assert_eq!(g.find_edge((1, 2)), Some(&()));
        assert_eq!(g.find_edge((2, 3)), Some(&()));
        assert_eq!(g.find_edge((3, 1)), Some(&()));
        assert_eq!(g.find_edge((1, 0)), Some(&()));
        assert_eq!(g.find_edge((2, 1)), Some(&()));
        assert_eq!(g.find_edge((3, 2)), Some(&()));
        assert_eq!(g.find_edge((1, 3)), Some(&()));
        assert_eq!(g.find_edge((0, 2)), None);
        assert_eq!(g.find_edge((2, 0)), None);
        assert_eq!(g.find_edge((0, 3)), None);
        assert_eq!(g.find_edge((3, 0)), None);
        assert_eq!(g.find_edge((3, 4)), None);
        assert_eq!(g.find_edge((4, 3)), None);
        assert_eq!(g.find_edge((4, 5)), None);
    }

    #[test]
    fn test_directed() {
        use super::Graph;
        let mut g = Graph::new(true);
        g.add_edge((0, 1), ());
        g.add_edge((1, 2), ());
        g.add_edge((2, 3), ());
        g.add_edge((3, 1), ());
        assert_eq!(g.find_edge((0, 1)), Some(&()));
        assert_eq!(g.find_edge((1, 2)), Some(&()));
        assert_eq!(g.find_edge((2, 3)), Some(&()));
        assert_eq!(g.find_edge((3, 1)), Some(&()));
        assert_eq!(g.find_edge((1, 0)), None);
        assert_eq!(g.find_edge((2, 1)), None);
        assert_eq!(g.find_edge((3, 2)), None);
        assert_eq!(g.find_edge((1, 3)), None);
        assert_eq!(g.find_edge((0, 2)), None);
        assert_eq!(g.find_edge((2, 0)), None);
        assert_eq!(g.find_edge((0, 3)), None);
        assert_eq!(g.find_edge((3, 0)), None);
        assert_eq!(g.find_edge((3, 4)), None);
        assert_eq!(g.find_edge((4, 3)), None);
        assert_eq!(g.find_edge((4, 5)), None);
    }
}

/// The edge is stored in an option because if the edge is undirected,
/// it is stored twice, in that case the edge value is stored in u -> v
/// only where u < v and None is stored at v -> u.
type EdgeTable<V, E, S> = HashMap<V, Option<E>, S>;
type NodeTable<V, E, S> = HashMap<V, EdgeTable<V, E, S>, S>;

pub trait Edge {
    fn weight(&self) -> isize {
        1
    }
}

impl Edge for () {}

pub struct Graph<V,E,S>
    where V: Copy+Hash+Eq+Ord, 
          E: Edge, 
          S: BuildHasher, 
{
    node_tables: NodeTable<V, E, S>,
    directed: bool,
    hasher: Box<Fn()->S>,
}

impl<V,E,S> Graph<V,E,S> 
    where V: Copy+Hash+Eq+Ord, 
          E: Edge, 
          S: BuildHasher, 
{
    pub fn new_with_hasher(directed: bool, f: Box<Fn()->S>) -> Self {
        Graph { node_tables: HashMap::with_hasher(f()), directed, hasher: f }
    }

    pub fn add_vertex(&mut self, v: V) -> &mut EdgeTable<V, E, S> {
        let table = self.new_table();
        self.node_tables.entry(v)
            .or_insert(table)
    }

    pub fn add_edge(&mut self, (u, v): (V, V), e: E) -> &mut E {
        if u == v {
            panic!("self loops not allowed yet")
        }

        let (u, v) = if !self.directed && u > v { (v, u) } else { (u, v) }; 
        if !self.directed {
            self.add_vertex(v).entry(u).or_insert(None);
        }
        self.add_vertex(u)
            .entry(v).or_insert(Some(e))
            .as_mut().unwrap()
    }

    pub fn find_edge(&self, (u, v): (V, V)) -> Option<&E> {
        let (u, v) = if !self.directed && u > v { (v, u) } else { (u, v) }; 
        self.node_tables.get(&u)
            .and_then(|umap| umap.get(&v))
            .and_then(|e| e.as_ref())
    }
    
    fn new_table(&self) -> EdgeTable<V, E, S> {
        HashMap::with_hasher(self.hasher.as_ref()())
    }
}

impl<V,E> Graph<V,E,RandomState> 
    where V: Copy+Hash+Eq+Ord, 
          E: Edge, 
{
    pub fn new(directed: bool) -> Self {
        Graph { node_tables: HashMap::default(), directed, hasher: Box::new(|| RandomState::default()) }
    }
}