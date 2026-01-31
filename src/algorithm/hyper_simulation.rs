use std::collections::{HashMap, HashSet};
use log::{info, warn};
// use std::fs::File;
// use std::io::{self, Write};


use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::error::Error;


use graph_base::interfaces::{edge::{DirectedHyperedge, Hyperedge}, graph::SingleId, hypergraph::{ContainedDirectedHyperedge, ContainedHyperedge, DirectedHypergraph, Hypergraph}, typed::{Type, Typed}};

use crate::{algorithm::simulation, utils::logger::init_global_logger_once};
use crate::utils::logger::TraceLog;

pub trait LMatch {
    type Edge;
    // fn l_match(&'a self, e: &'a Self::Edge, e_prime: &'a Self::Edge) -> HashMap<&'a Self::Node, &'a HashSet<&'a Self::Node>>;
    fn new() -> Self;
    fn l_match_with_node_mut(&mut self, e: &Self::Edge, e_prime: &Self::Edge, u: usize) -> &HashSet<usize>;
    fn l_match_with_node(&self, e: &Self::Edge, e_prime: &Self::Edge, u: usize) -> &HashSet<usize>;
    fn dom(&self, e: &Self::Edge, e_prime: &Self::Edge) -> impl Iterator<Item = &usize>;
}

#[derive(Hash)]
pub struct SematicCluster<'a, E: Hyperedge> {
    id: usize,
    hyperedges: Vec<&'a E>,
}

impl<'a, E: Hyperedge> SematicCluster<'a, E> {

    pub fn new(id: usize, hyperedges: Vec<&'a E>) -> Self {
        Self {
            id,
            hyperedges,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn hyperedges(&self) -> &Vec<&'a E> {
        &self.hyperedges
    }
}

pub trait Delta<'a> {
    type Node;
    type Edge: Hyperedge;
    fn get_sematic_clusters(&'a self, u: &'a Self::Node, v: &'a Self::Node) -> &'a Vec<(SematicCluster<'a, Self::Edge>, SematicCluster<'a, Self::Edge>)>;
}

pub trait DMatch<'a> {
    type Edge: Hyperedge;
    // fn d_match_mut(&mut self, e: &SematicCluster<'a, Self::Edge>, e_prime: &SematicCluster<'a, Self::Edge>) -> &HashSet<(usize, usize)>;
    fn d_match(&self, e: &SematicCluster<'a, Self::Edge>, e_prime: &SematicCluster<'a, Self::Edge>) -> &HashSet<(usize, usize)>;
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
    fn get_soft_simulation_naive(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
    fn get_hyper_simulation_naive(&'a self, other: &'a Self, delta: &'a impl Delta<'a, Node = Self::Node, Edge = Self::Edge>, d_match: & impl DMatch<'a, Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
}

// struct MultiWriter<W1: Write, W2: Write> {
//     w1: W1,
//     w2: W2,
// }

// impl<W1: Write, W2: Write> Write for MultiWriter<W1, W2> {
//     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//         self.w1.write_all(buf)?;
//         self.w2.write_all(buf)?;
//         Ok(buf.len())
//     }
//     fn flush(&mut self) -> io::Result<()> {
//         self.w1.flush()?;
//         self.w2.flush()
//     }
// }


impl<'a, H> HyperSimulation<'a> for H 
where H: Hypergraph<'a> + Typed<'a> + LPredicate<'a> + ContainedHyperedge<'a> {
    fn get_simulation_fixpoint(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        todo!()
    }

    fn get_simulation_recursive(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        todo!()
    }

    fn get_simulation_naive(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        
        // let log_file = File::create("hyper-simulation.log")
        //     .expect("Failed to create log file");
        // let multi_writer = MultiWriter {
        //     w1: log_file,
        //     w2: io::stdout(),
        // };
        
        // env_logger::Builder::new()
        //     .target(env_logger::Target::Pipe(Box::new(multi_writer)))
        //     .init();

        init_global_logger_once("hyper-simulation.log");

        info!("Start Naive Hyper Simulation");

        let self_contained_hyperedge = self.get_hyperedges_list();
        let other_contained_hyperedge = other.get_hyperedges_list();

        let mut simulation: HashMap<&Self::Node, HashSet<&Self::Node>> = self.nodes().map(|u| {
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
                                let id_set = l_match.l_match_with_node(e, e_prime, u.id());
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

        info!("END Initial, sim: is ");
        for (u, v_set) in &simulation {
            info!("\tsim({}) = {:?}", u.id(), v_set.iter().map(|v| v.id()).collect::<Vec<_>>());
        }
        

        let mut changed = true;
        while changed {
            changed = false;
            for u in self.nodes() {
                let mut need_delete = Vec::new();
                for v in simulation.get(u).unwrap() {
                    info!("Checking {} -> {}", u.id(), v.id());
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
                                    info!("Keeping {} -> {}", u.id(), v.id());
                                    _delete = false;
                                    break;
                                }
                            }
                        }
                    }
                    if _delete {
                        info!("Deleting {} -> {}", u.id(), v.id());
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

    fn get_soft_simulation_naive(&'a self, other: &'a Self, l_match: &mut impl LMatch<Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        init_global_logger_once("hyper-simulation.log");

        info!("Start Naive Hyper Simulation");

        // let self_contained_hyperedge = self.get_hyperedges_list();
        // let other_contained_hyperedge = other.get_hyperedges_list();

        let mut l_predicate_edges: HashMap<(usize, usize), Vec<(&Self::Edge, &Self::Edge)>> = HashMap::new();
        for e in self.hyperedges() {
            for e_prime in other.hyperedges() {
                if self.l_predicate_edge(e, e_prime) {
                    for u in e.id_set() {
                        for v in e_prime.id_set() {
                            l_predicate_edges.entry((u, v)).or_default().push((e, e_prime));
                        }
                    }
                }
            }
        }

        let mut simulation: HashMap<&'a Self::Node, HashSet<&'a Self::Node>> = self.nodes().map(|u| {
            let res = other.nodes().filter(|v| {
                if self.type_same(u, *v) {
                    if let Some(edge_pairs) = l_predicate_edges.get(&(u.id(), v.id())) {
                        for (e, e_prime) in edge_pairs {
                            let id_set = l_match.l_match_with_node(e, e_prime, u.id());
                            if !id_set.contains(&v.id()) {
                                return false;
                            }
                        }
                        return true;
                    } else {
                        return true;
                    }
                }
                false
            }).collect();
            (u, res)
        }).collect();

        info!("END Initial, sim: is ");
        for (u, v_set) in &simulation {
            info!("\tsim({}) = {:?}", u.id(), v_set.iter().map(|v| v.id()).collect::<Vec<_>>());
        }
        

        let mut changed = true;
        while changed {
            changed = false;
            for u in self.nodes() {
                let mut need_delete = Vec::new();
                for v in simulation.get(u).unwrap() {
                    info!("Checking {} -> {}", u.id(), v.id());
                    let mut _delete = false;

                    if let Some(edge_pairs) = l_predicate_edges.get(&(u.id(), v.id())) {
                        for (e, e_prime) in edge_pairs {
                            if l_match.dom(e, e_prime).all(|u_prime| {
                                l_match.l_match_with_node(e, e_prime, u_prime.clone()).iter().map(|id| {other.get_node_by_id(*id)}).any(|v_prime| {
                                    if let Some(v_prime) = v_prime {
                                        return simulation.get(u).unwrap().contains(v_prime);
                                    } else {
                                        return false;
                                    }
                                })
                            }) {
                                info!("Keeping {} -> {}", u.id(), v.id());
                                _delete = true;
                                break;
                            }
                        }
                    }

                    if _delete {
                        info!("Deleting {} -> {}", u.id(), v.id());
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

    fn get_hyper_simulation_naive(&'a self, other: &'a Self, delta: &'a impl Delta<'a, Node = Self::Node, Edge = Self::Edge>, d_match: & impl DMatch<'a, Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        init_global_logger_once("hyper-simulation.log");
        let mut hs_trace = HyperSimulationTrace::new();
        let mut simulation: HashMap<&'a Self::Node, HashSet<&'a Self::Node>> = self.nodes().map(|u| {
            let res = other.nodes().filter(|v| {
                if self.type_same(u, *v) {
                    let sematic_clusters = delta.get_sematic_clusters(u, v);
                    for (cluster_u, cluster_v) in sematic_clusters {
                        let d_match_set = d_match.d_match(cluster_u, cluster_v);
                        if !d_match_set.contains(&(u.id(), v.id())) {
                            // Add the trace that nodes (u, v) are deleted by the `sematic_clusters`
                            hs_trace.add_base_event(cluster_u.id, d_match_set.clone());
                            return false;
                        }
                    }
                    return true;
                }
                false
            }).collect();
            (u, res)
        }).collect();

        info!("END Initial, sim: is ");
        for (u, v_set) in &simulation {
            info!("\tsim({}) = {:?}", u.id(), v_set.iter().map(|v| v.id()).collect::<Vec<_>>());
        }

        let mut simulation_by_id: HashSet<(usize, usize)> = simulation.iter().flat_map(|(u, v_set)| {
            v_set.iter().map(move |v| (u.id(), v.id()))
        }).collect();

        let mut changed = true;
        while changed {
            changed = false;
            for u in self.nodes() {
                let mut need_delete = Vec::new();
                for v in simulation.get(u).unwrap() {
                    info!("Checking {} -> {}", u.id(), v.id());
                    let mut _delete = false;

                    let sematic_clusters = delta.get_sematic_clusters(u, v);
                    for (cluster_u, cluster_v) in sematic_clusters {
                        let d_relation = d_match.d_match(cluster_u, cluster_v);
                        // Check if for all (u_id, v_id) in d_relation, (u_id, v_id) is in simulation, i.e., d_relation is a subset of simulation_by_id
                        if !d_relation.is_subset(&simulation_by_id) {
                            info!("Deleting {} -> {}", u.id(), v.id());
                            // Add the trace that nodes (u, v) are deleted by the `sematic_clusters`
                            let uncoverd: HashSet<(usize, usize)> = d_relation.difference(&simulation_by_id).copied().collect();
                            hs_trace.add_derivation_event(cluster_u.id, uncoverd);
                            _delete = true;
                            break;
                        }
                    }

                    if _delete {
                        need_delete.push(v.clone());
                    }
                }

                for v in need_delete {
                    simulation.get_mut(u).unwrap().remove(v);
                    simulation_by_id.remove(&(u.id(), v.id()));
                    changed = true;
                }
            }
        }

        hs_trace.store_trace_file("hyper_simulation.trace").unwrap();

        return simulation;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct HyperSimulationTrace {
    events: Vec<HSEvent>
}

impl HyperSimulationTrace {
    fn new() -> Self {
        HyperSimulationTrace {
            events: Vec::new()
        }
    }

    fn add_base_event(&mut self, sc_id: usize, d_match: HashSet<(usize, usize)>) {
        let event = HSEvent::Base(sc_id, d_match);
        self.events.push(event);
    }
    fn add_derivation_event(&mut self, sc_id: usize, uncoverd: HashSet<(usize, usize)>) {
        let event = HSEvent::Derivation(sc_id, uncoverd);
        self.events.push(event);
    }
}

impl TraceLog for HyperSimulationTrace {
    fn store_trace_file(self, filename: &'static str) -> Result<(), Box<dyn Error>> {
        // use bincode to save the HyperSimulationTrace.
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        bincode::serialize_into(&mut writer, &self)?;
        Ok(())
    }
    
    fn get_trace(filename: &'static str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let file_decoded: HyperSimulationTrace = bincode::deserialize_from(&mut reader)?;
        Ok(file_decoded)
    }

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum HSEvent {
    Base(usize, HashSet<(usize, usize)>), // current D-Match
    Derivation(usize, HashSet<(usize, usize)>) // D-Match \ Sim
}