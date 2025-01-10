use std::hash::Hash;
use std::collections::HashMap;
use crate::graph::base::{Graph, Directed, Adjacency, AdjacencyInv};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct LabelNode<L: Label> {
    id: u64,
    label: L
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


pub trait Label: Hash + Eq + Clone {
    // type Value;    
}

pub trait Labeled<'a>: Graph<'a> {
    fn label_same(node: &Self::Node, label: &Self::Node) -> bool;
    fn get_label(&'a self, node: &'a Self::Node) -> &'a impl Label;
}

#[derive(Hash, Eq, Clone)]
pub struct SingleLabel(());

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

    fn nodes(&'a self) -> impl Iterator<Item = &Self::Node> {
        self.nodes.iter()
    }

    fn edges(&'a self) -> impl Iterator<Item = &Self::Edge> {
        self.edges.iter()
    }

    fn get_edges_pair(&'a self) -> impl Iterator<Item = (&Self::Node, &Self::Node)> {
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
    fn label_same(node: &Self::Node, label: &Self::Node) -> bool {
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
}
