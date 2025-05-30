use std::collections::{HashMap, HashSet};

use graph_base::interfaces::{edge::DirectedHyperedge, graph::SingleId, hypergraph::{ContainedDirectedHyperedge, ContainedHyperedge, DirectedHypergraph, Hypergraph}, typed::{Type, Typed}};

pub trait LMatch {
    type Edge;
    // fn l_match(&'a self, e: &'a Self::Edge, e_prime: &'a Self::Edge) -> HashMap<&'a Self::Node, &'a HashSet<&'a Self::Node>>;
    fn new() -> Self;
    fn l_match_with_node_mut(&mut self, e: &Self::Edge, e_prime: &Self::Edge, u: usize) -> &HashSet<usize>;
    fn l_match_with_node(&self, e: &Self::Edge, e_prime: &Self::Edge, u: usize) -> &HashSet<usize>;
    fn dom(&self, e: &Self::Edge, e_prime: &Self::Edge) -> impl Iterator<Item = &usize>;
}

pub trait LPredicate<'a>: Hypergraph<'a> {
    fn l_predicate_node(&'a self, u: &'a Self::Node, v: &'a Self::Node) -> bool;
    fn l_predicate_edge(&'a self, e: &'a Self::Edge, e_prime: &'a Self::Edge) -> bool;
    fn l_predicate_set(&'a self, x: &HashSet<&'a Self::Node>, y: &HashSet<&'a Self::Node>) -> bool;
}

pub trait HyperSimulation<'a>: Hypergraph<'a> {

    fn get_simulation_fixpoint(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
    fn get_simulation_recursive(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
    fn get_simulation_naive(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
}

impl<'a, H> HyperSimulation<'a> for H 
where H: Hypergraph<'a> + Typed<'a> + LPredicate<'a> + ContainedHyperedge<'a> {
    fn get_simulation_fixpoint(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        todo!()
    }

    fn get_simulation_recursive(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        todo!()
    }

    fn get_simulation_naive(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        let self_contained_hyperedge = self.get_hyperedges_list();
        let other_contained_hyperedge = other.get_hyperedges_list();

        let mut simulation: HashMap<&'a Self::Node, HashSet<&'a Self::Node>> = self.nodes().map(|u| {
            let res = other.nodes().filter(|v| {
                if self.type_same(u, *v) {
                    // For each e, compute the union of l_match(u) over all matching e_prime,
                    // then take the intersection across all e.
                    let mut l_match_intersection: Option<HashSet<usize>> = None;
                    for e in self.contained_hyperedges(&self_contained_hyperedge, u) {
                        let mut l_match_union: HashSet<usize> = HashSet::new();
                        for e_prime in other.contained_hyperedges(&other_contained_hyperedge, v) {
                            if self.l_predicate_edge(e, e_prime) {
                                // let l_match = self.l_match(e, e_prime);
                                let id_set = l_match.l_match_with_node_mut(e, e_prime, u.id());
                                l_match_union = l_match_union.union(&id_set).copied().collect();
                            }
                        }
                        l_match_intersection = match l_match_intersection {
                            Some(ref acc) => Some(acc.intersection(&l_match_union).copied().collect()),
                            None => Some(l_match_union),
                        };
                    }
                    if let Some(l_match_intersection) = l_match_intersection {
                        if l_match_intersection.contains(&v.id()){
                            return true;
                        }
                    }
                }
                false
            }).collect();
            (u, res)
        }).collect();

        let mut changed = false;
        while !changed {
            for u in self.nodes() {
                let mut need_delete = Vec::new();
                for v in simulation.get(u).unwrap() {
                    let mut _delete = true;
                    for e in self.contained_hyperedges(&self_contained_hyperedge, u) {
                        if !_delete {
                            break;
                        }
                        for e_prime in other.contained_hyperedges(&other_contained_hyperedge, v) {
                            if self.l_predicate_edge(e, e_prime) {
                                if l_match.dom(e, e_prime).all(|u_prime| {
                                    l_match.l_match_with_node(e, e_prime, u_prime.clone()).iter().map(|id| {other.get_node_by_id(*id)}).any(|v_prime| {
                                        if let Some(v_prime) = v_prime {
                                            return simulation.get(u).unwrap().contains(v_prime);
                                        } else {
                                            return false;
                                        }
                                    })
                                }) {
                                    _delete = false;
                                    break;
                                }
                            }
                        }
                    }
                    if _delete {
                        need_delete.push(v.clone());
                    }
                }
                for v in need_delete {
                    simulation.get_mut(u).unwrap().remove(v);
                    changed = true;
                }
            }
        }

        simulation
    }
}

// pub trait DiHyperSimulation<'a> {
//     type Node;

//     fn get_simulation_fixpoint(&'a self, other: &'a Self) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
//     fn get_simulation_recursive(&'a self, other: &'a Self) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
//     fn get_simulation_native(&'a self, other: &'a Self) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
// }

// impl<'a, H> DiHyperSimulation<'a> for H 
// where 
//     H: DirectedHypergraph<'a> + Typed<'a> + LMatch<'a> + LPredicate<'a> + ContainedDirectedHyperedge<'a>,
//     H::Node: Type, H::Edge: DirectedHyperedge {
//     type Node = H::Node;

//     fn get_simulation_fixpoint(&'a self, other: &'a Self) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
//         todo!()
//     }

//     fn get_simulation_recursive(&'a self, other: &'a Self) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
//         todo!()
//     }

//     fn get_simulation_native(&'a self, other: &'a Self) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
//         // let mut simulation: HashMap<&'a Self::Node, HashSet<&'a Self::Node>> = self.nodes().map(|u| {
//         //     let res = other.nodes().filter(|v| {
//         //         self.type_same(u, *v)
//         //     }).collect();
//         //     (u, res)
//         // }).collect();

//         // let self_contained_hyperedge = self.get_hyperedges_src();
//         // let other_contained_hyperedge = other.get_hyperedges_src();

//         // let mut changed = false;
//         // while !changed {
//         //     for u in self.nodes() {
//         //         let mut need_delete = Vec::new();
//         //         for v in simulation.get(u).unwrap() {
//         //             let mut _delete = true;
//         //             for e in self.contained_hyperedges(&self_contained_hyperedge, u) {
//         //                 if !_delete {
//         //                     break;
//         //                 }
//         //                 for e_prime in other.contained_hyperedges(&other_contained_hyperedge, v) {
//         //                     if self.l_predicate_edge(e, e_prime) {
//         //                         let l_match = self.l_match(e, e_prime);
//         //                         if l_match(u).id() == v.id() 
//         //                             && self.dom(l_match).all(|u_prime| {
//         //                                 simulation.get(u).unwrap().contains(&u_prime)
//         //                         }) {
//         //                             _delete = false;
//         //                             break;
//         //                         }
//         //                     }
//         //                 }
//         //             }
//         //             if _delete {
//         //                 need_delete.push(v.clone());
//         //             }
//         //         }
//         //         for v in need_delete {
//         //             simulation.get_mut(u).unwrap().remove(v);
//         //             changed = true;
//         //         }
//         //     }
//         // }

//         // simulation
//         todo!()
//     }
// }