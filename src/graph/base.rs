use std::{collections::HashMap, fmt::Display, hash::Hash};

pub trait Graph<'a> {
    type Node: Eq + Hash + Clone + Sized + Display + SingleId;
    type Edge: Eq + Hash + Clone + IdPair;
    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node>;
    fn edges(&'a self) -> impl Iterator<Item = &'a Self::Edge>;
    fn get_edges_pair(&'a self) -> impl Iterator<Item = (&'a Self::Node, &'a Self::Node)> {
        let id_map: HashMap<_, _> = HashMap::from_iter(self.nodes().map(|node| (node.id(), node)));
        self.edges().map(move |edge| (id_map.get(&edge.pair().0).unwrap().clone(), id_map.get(&edge.pair().1).unwrap().clone()) ).collect::<Vec<_>>().into_iter()
    }
    fn get_edges_pair_with_edge(&'a self) -> impl Iterator<Item = (&'a Self::Node, &'a Self::Edge, &'a Self::Node)> {
        let id_map: HashMap<_, _> = HashMap::from_iter(self.nodes().map(|node| (node.id(), node)));
        self.edges().map(move |edge| (id_map.get(&edge.pair().0).unwrap().clone(), edge, id_map.get(&edge.pair().1).unwrap().clone()) ).collect::<Vec<_>>().into_iter()
    }
    fn add_node(&mut self, node: Self::Node);
    fn add_edge(&mut self, edge: Self::Edge);
}

pub trait SingleId {
    fn id(&self) -> usize;
}
pub trait Label {
    fn label(&self) -> &str;
}

pub trait IdPair {
    fn pair(&self) -> (usize, usize);
}

pub trait Directed {}

pub struct AdjacencyList<'a, T: Graph<'a>>(HashMap<&'a T::Node, Vec<&'a T::Node>>);

impl<'a, T> Display for AdjacencyList<'a, T> 
where T: Graph<'a> {    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (node, adj) in self.0.iter() {
            
            let s1 = format!("{}", node);
            let mut s2 = String::new();
            for node in adj {
                s2.push_str(format!("{}, ", node).as_str());
            }

            s.push_str(format!("Node {} -> {{{}}}\n", s1, s2).as_str());
        }
        write!(f, "{}", s)
    }
}

pub trait Adjacency<'a>: Graph<'a> + Directed + Sized {
    fn get_adj(&'a self) -> AdjacencyList<'a, Self> {
        let mut adj = HashMap::new();
        for node in self.nodes() {
            adj.insert(node, Vec::new());
        }
        for (src, dst) in self.get_edges_pair() {
            adj.get_mut(src).unwrap().push(dst);
        }

        AdjacencyList(adj)
    }
    fn get_post(&'a self, adj: &AdjacencyList<'a, Self>, node: &Self::Node) -> impl Iterator<Item = &'a Self::Node> {
        adj.0.get(node).expect(format!("No node in adjacency table named {} \n adj is: {}", node, adj).as_str()).iter().copied()
    }
}

pub trait AdjacencyInv<'a>: Graph<'a> + Directed + Sized {
    fn get_adj_inv(&'a self) -> AdjacencyList<'a, Self> {
        let mut adj_inv = HashMap::new();
        for node in self.nodes() {
            adj_inv.insert(node, Vec::new());
        }
        for (src, dst) in self.get_edges_pair() {
            adj_inv.get_mut(dst).unwrap().push(src);
        }
        AdjacencyList(adj_inv)
    }
    fn get_pre(&'a self, adj_inv: &AdjacencyList<'a, Self>, node: &Self::Node) -> impl Iterator<Item = &'a Self::Node> {
        adj_inv.0.get(node).expect(format!("No node in adjacency table named {} \n adj is: {}", node, adj_inv).as_str()).iter().copied()
    }
}