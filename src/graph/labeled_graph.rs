use std::fmt::Display;
use std::{hash::Hash, iter::FromIterator};
use std::collections::HashMap;
use crate::graph::base::{Graph, Directed, Adjacency, AdjacencyInv};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct LabelNode<L: Label> {
    id: u64,
    label: L
}

impl<L: Label> Display for LabelNode<L> 
where L: Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[id: {}, label: {}]", self.id, self.label)
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct LabeledEdge<L: Label> {
    src: u64,
    dst: u64,
    label: L,
}

/// `L1` is the label of the nodes, `L2` is the label of the edges. All these labels needs to implement the `Label` trait.
/// 
/// The graph is directed.

pub struct LabeledGraph<L1: Label, L2: Label> {
    nodes: Vec<LabelNode<L1>>,
    edges: Vec<LabeledEdge<L2>>,
}


pub trait Label: Hash + Eq + Clone + Display {
    // type Value;    
}

pub trait Labeled<'a>: Graph<'a> {
    fn label_same(&self, node: &Self::Node, label: &Self::Node) -> bool;
    fn get_label(&'a self, node: &'a Self::Node) -> &'a impl Label;
}

#[derive(Hash, Eq, Clone)]
pub struct SingleLabel(());

impl Display for SingleLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Label for SingleLabel {}

impl PartialEq for SingleLabel {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

pub type StandardLabeledGraph = LabeledGraph<String, SingleLabel>;

impl Label for String {}

impl<'a> Graph<'a> for StandardLabeledGraph {
    type Node = LabelNode<String>;

    type Edge = LabeledEdge<SingleLabel>;

    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node> {
        self.nodes.iter()
    }

    fn edges(&'a self) -> impl Iterator<Item = &'a Self::Edge> {
        self.edges.iter()
    }

    fn get_edges_pair(&'a self) -> impl Iterator<Item = (&'a Self::Node, &'a Self::Node)> {
        let id_map: HashMap<_, _, std::collections::hash_map::RandomState> = HashMap::from_iter(self.nodes.iter().map(|node| (node.id, node)));
        self.edges.iter().map(|edge| (id_map.get(&edge.src).unwrap().clone(), id_map.get(&edge.dst).unwrap().clone()) ).collect::<Vec<_>>().into_iter()
    }

    fn add_node(&mut self, node: Self::Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, edge: Self::Edge) {
        self.edges.push(edge);
    }
}

impl<'a> Labeled<'a> for StandardLabeledGraph {
    fn label_same(&self, node: &Self::Node, label: &Self::Node) -> bool {
        node.label == label.label
    }

    fn get_label(&'a self, node: &'a Self::Node) -> &'a impl Label {
        &node.label
    }
}

impl Directed for StandardLabeledGraph {}

impl Adjacency<'_> for StandardLabeledGraph {}
    
impl AdjacencyInv<'_> for StandardLabeledGraph {}

impl StandardLabeledGraph {
    pub fn new() -> Self {
        return StandardLabeledGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: u64, label: String) {
        self.nodes.push(LabelNode {
            id,
            label
        });
    }

    pub fn add_edge(&mut self, src: u64, dst: u64) {
        self.edges.push(LabeledEdge {
            src,
            dst,
            label: SingleLabel(())
        });
    }
}

impl Display for StandardLabeledGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("nodes: ");
        for node in self.nodes.iter() {
            s.push_str(format!("{}, ", node).as_str());
        }
        s.push_str("\nedges: ");
        for edge in self.edges.iter() {
            s.push_str(format!("{} -> {}, ", edge.src, edge.dst).as_str());
        }
        write!(f, "{}", s)
    }
}

trait HyperLabeled<'a>: Labeled<'a> {
    type L: Label;
    fn set_same_label_fn(&mut self, f: Box<dyn Fn(&Self::L, &Self::L) -> bool>);
}

pub struct HyperLabelGraph<L: Label> {
    nodes: Vec<LabelNode<L>>,
    edges: Vec<LabeledEdge<SingleLabel>>,
    same_label_fn: Option<Box<dyn Fn(&L, &L) -> bool>>,
}

impl<'a, L: Label> Graph<'a> for HyperLabelGraph<L> {
    type Node = LabelNode<L>;
    type Edge = LabeledEdge<SingleLabel>;

    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node> {
        self.nodes.iter()
    }

    fn edges(&'a self) -> impl Iterator<Item = &'a Self::Edge> {
        self.edges.iter()
    }

    fn get_edges_pair(&'a self) -> impl Iterator<Item = (&'a Self::Node, &'a Self::Node)> {
        let id_map: HashMap<_, _, std::collections::hash_map::RandomState> = HashMap::from_iter(self.nodes.iter().map(|node| (node.id, node)));
        self.edges.iter().map(|edge| (id_map.get(&edge.src).unwrap().clone(), id_map.get(&edge.dst).unwrap().clone()) ).collect::<Vec<_>>().into_iter()
    }

    fn add_node(&mut self, node: Self::Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, edge: Self::Edge) {
        self.edges.push(edge);
    }
}

impl<'a, L: Label> Labeled<'a> for HyperLabelGraph<L> {
    fn label_same(&self, node: &Self::Node, label: &Self::Node) -> bool {
        (self.same_label_fn.as_ref().expect("hyper compare function is not set"))(&node.label, &label.label)
    }

    fn get_label(&'a self, node: &'a Self::Node) -> &'a impl Label {
        &node.label
    }
}

impl<L: Label> HyperLabeled<'_> for HyperLabelGraph<L> {
    type L = L;
    fn set_same_label_fn(&mut self, f: Box<dyn Fn(&Self::L, &Self::L) -> bool>) {
        self.same_label_fn = Some(f);
    }
}

impl Directed for HyperLabelGraph<SingleLabel> {}

impl Adjacency<'_> for HyperLabelGraph<SingleLabel> {}
    
impl AdjacencyInv<'_> for HyperLabelGraph<SingleLabel> {}

impl<L: Label> HyperLabelGraph<L> {
    pub fn new() -> Self {
        return HyperLabelGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
            same_label_fn: None,
        }
    }
    
}