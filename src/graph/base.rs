use std::{collections::HashMap, hash::Hash};


pub trait Graph<'a> {
    type Node: Eq + Hash + Clone + Sized;
    type Edge: Eq + Hash + Clone;
    fn nodes(&'a self) -> impl Iterator<Item = &Self::Node>;
    fn edges(&'a self) -> impl Iterator<Item = &Self::Edge>;
    fn get_edges_pair(&'a self) -> impl Iterator<Item = (&Self::Node, &Self::Node)>;
    fn add_node(&mut self, node: Self::Node);
    fn add_edge(&mut self, edge: Self::Edge);
}

pub trait Directed {
    
}

type AdjacencyList<'a, T: Graph<'a>> = HashMap<&'a T::Node, Vec<&'a T::Node>>;

pub trait Adjacency<'a>: Graph<'a> + Directed {
    fn get_adj(&'a self) -> AdjacencyList<'a, Self> {
        let mut adj = HashMap::new();
        for node in self.nodes() {
            adj.insert(node, Vec::new());
        }
        for (src, dst) in self.get_edges_pair() {
            adj.get_mut(src).unwrap().push(dst);
        }
        adj
    }
    fn get_post(&'a self, adj: &AdjacencyList<'a, Self>, node: &Self::Node) -> impl Iterator<Item = &'a Self::Node> {
        adj.get(node).unwrap().iter().copied()
    }
}

pub trait AdjacencyInv<'a>: Graph<'a> + Directed {
    fn get_adj_inv(&'a self) -> AdjacencyList<'a, Self> {
        let mut adj_inv = HashMap::new();
        for node in self.nodes() {
            adj_inv.insert(node, Vec::new());
        }
        for (src, dst) in self.get_edges_pair() {
            adj_inv.get_mut(dst).unwrap().push(src);
        }
        adj_inv
    }
    fn get_pre(&'a self, adj_inv: &AdjacencyList<'a, Self>, node: &Self::Node) -> impl Iterator<Item = &'a Self::Node> {
        adj_inv.get(node).unwrap().iter().copied()
    }
}