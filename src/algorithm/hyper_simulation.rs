use std::collections::{HashMap, HashSet, VecDeque};
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
    fn get_hyper_simulation_effect(&'a self, other: &'a Self, delta: &'a impl Delta<'a, Node = Self::Node, Edge = Self::Edge>, d_match: & impl DMatch<'a, Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
    fn get_hyper_simulation_effect_pass_by(&'a self, other: &'a Self, delta: &'a impl Delta<'a, Node = Self::Node, Edge = Self::Edge>, d_match: & impl DMatch<'a, Edge = Self::Edge>, type_same_lookup: &HashMap<&'a Self::Node, HashSet<&'a Self::Node>>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
    fn get_hyper_simulation_effect_by_id(&'a self, hc_map: &HashMap<(usize, usize), Vec<((usize, usize), HashSet<(usize, usize)>)>>) -> HashSet<(usize, usize)>;
    fn get_hyper_simulation_strict(&'a self, other: &'a Self, delta: &'a impl Delta<'a, Node = Self::Node, Edge = Self::Edge>, d_match: & impl DMatch<'a, Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
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
        init_global_logger_once("logs/hyper-simulation.log");
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

        info!("END Initial, raw-sim: is ");
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

        hs_trace.store_trace_file("logs/hyper_simulation.trace").unwrap();

        return simulation;
    }

    fn get_hyper_simulation_strict(&'a self, other: &'a Self, delta: &'a impl Delta<'a, Node = Self::Node, Edge = Self::Edge>, d_match: & impl DMatch<'a, Edge = Self::Edge>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        init_global_logger_once("logs/hyper-simulation.log");
        let mut hs_trace = HyperSimulationTrace::new();
        let mut simulation: HashMap<&'a Self::Node, HashSet<&'a Self::Node>> = self.nodes().map(|u| {
            let res = other.nodes().filter(|v| {
                if self.type_same(u, *v) {
                    let sematic_clusters = delta.get_sematic_clusters(u, v);
                    // Highlight!
                    if sematic_clusters.len() == 0 {
                        info!("Deleting {} -> {} because no sematic cluster", u.id(), v.id());
                        return false;
                    }
                    info!("Checking {} -> {}, sematic clusters size: {}", u.id(), v.id(), sematic_clusters.len());
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

        info!("END Initial, raw-sim: is ");
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

        hs_trace.store_trace_file("logs/hyper_simulation.trace").unwrap();

        return simulation;
    }

    fn get_hyper_simulation_effect(
        &'a self,
        other: &'a Self,
        delta: &'a impl Delta<'a, Node = Self::Node, Edge = Self::Edge>,
        d_match: &impl DMatch<'a, Edge = Self::Edge>,
    ) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {

        init_global_logger_once("logs/hyper-simulation.log");

        // 建立 ID 到节点的映射，方便最后构造返回结果
        let mut id_to_u: HashMap<usize, &'a Self::Node> = HashMap::new();
        let mut id_to_v: HashMap<usize, &'a Self::Node> = HashMap::new();

        // 存储节点对与其对应的所有的 Semantic Clusters (HC) 及 D-match 结果
        let mut hc_map: HashMap<(usize, usize), Vec<((usize, usize), HashSet<(usize, usize)>)>> = HashMap::new();
        
        // Pi: 当前满足 Hyper Simulation 条件的 (u.id(), v.id()) 集合
        let mut pi: HashSet<(usize, usize)> = HashSet::new();

        // ==========================================
        // Phase 1: Declarative Initialization
        // ==========================================
        
        // 1. 初始化 Pi 并获取 HC 和 D-match
        for u in self.nodes() {
            id_to_u.insert(u.id(), u);
            for v in other.nodes() {
                id_to_v.insert(v.id(), v);

                if self.type_same(u, v) {
                    let sematic_clusters = delta.get_sematic_clusters(u, v);
                    let mut valid = true;
                    let mut local_clusters = Vec::new();

                    for (cluster_u, cluster_v) in sematic_clusters {
                        let cu_id = cluster_u.id;
                        let cv_id = cluster_v.id;
                        let d_match_set = d_match.d_match(cluster_u, cluster_v);

                        // 条件 2.a: (u, v) 必须在 D-match 中
                        if !d_match_set.contains(&(u.id(), v.id())) {
                            valid = false;
                            break; // 只要有一个 cluster 失败，(u,v) 就不可能在 Pi 中
                        }
                        local_clusters.push(((cu_id, cv_id), d_match_set.clone()));
                    }

                    if valid {
                        pi.insert((u.id(), v.id()));
                        hc_map.insert((u.id(), v.id()), local_clusters);
                    }
                }
            }
        }

        info!("完成了 Pi 的初始化和 HC、D-match 的获取，Pi 大小: {}", pi.len());

        // A_cluster 对应的 D-match 缓存，避免重复计算
        let mut a_cluster_d_match: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
        
        // 依赖索引构建:
        // D_cluster[(Cu, Cv)] -> { (u, v) \in Pi } 
        let mut d_cluster: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
        // D_pair[(u', v')] -> { (Cu, Cv) \in A_cluster }
        let mut d_pair: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

        for (&(u_id, v_id), clusters) in &hc_map {
            for ((cu_id, cv_id), d_match_set) in clusters {
                let c_pair = (*cu_id, *cv_id);
                
                // 填充 D_cluster
                d_cluster.entry(c_pair).or_default().insert((u_id, v_id));

                // 如果这是第一次遇到这个簇对，填充 D_pair
                if !a_cluster_d_match.contains_key(&c_pair) {
                    a_cluster_d_match.insert(c_pair, d_match_set.clone());

                    for &(up_id, vp_id) in d_match_set {
                        d_pair.entry((up_id, vp_id)).or_default().insert(c_pair);
                    }
                }
            }
        }

        info!("1. 初始化 Pi 并获取 HC 和 D-match");

        // 2. 初始化 V_C (Valid Clusters)
        let mut v_c: HashSet<(usize, usize)> = HashSet::new();
        for (c_pair, d_match_set) in &a_cluster_d_match {
            // 条件 2.b: D-match 的所有元素都必须在当前的 Pi 中
            if d_match_set.is_subset(&pi) {
                v_c.insert(*c_pair);
            }
        }

        info!("2. 初始化 V_C (Valid Clusters)");

        // 3. 找出失效的 (u, v) 加入队列 Q
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut pi_retained = pi.clone();

        for &(u_id, v_id) in &pi {
            let mut all_in_vc = true;
            if let Some(clusters) = hc_map.get(&(u_id, v_id)) {
                for (c_pair, _) in clusters {
                    if !v_c.contains(c_pair) {
                        all_in_vc = false;
                        break;
                    }
                }
            }

            if !all_in_vc {
                q.push_back((u_id, v_id));      // 加入 Worklist
                pi_retained.remove(&(u_id, v_id)); // Pi = Pi \ Q
            }
        }
        pi = pi_retained;

        info!("3. 找出失效的 (u, v) 加入队列 Q");

        // ==========================================
        // Phase 2: Cascade deletions via the queue
        // ==========================================
        while let Some((up_id, vp_id)) = q.pop_front() {
            // 获取所有依赖于已删除节点对 (u', v') 的簇对 (Cu, Cv)
            if let Some(dependent_clusters) = d_pair.get(&(up_id, vp_id)) {
                for c_pair in dependent_clusters {
                    // 如果簇对仍然被认为是有效的，现在它失效了
                    if v_c.contains(c_pair) {
                        v_c.remove(c_pair); // V_c = V_c \ {(Cu, Cv)}

                        // 级联使依赖这个失效簇对的 (u, v) 失效
                        if let Some(dependent_node_pairs) = d_cluster.get(c_pair) {
                            for node_pair in dependent_node_pairs {
                                if pi.contains(node_pair) {
                                    pi.remove(node_pair);
                                    q.push_back(*node_pair);
                                }
                            }
                        }
                    }
                }
            }
        }

        info!("结束了主调用");

        // ==========================================
        // Phase 3: Construct Output
        // ==========================================
        // 将基于 ID 的关系还原为引用的 HashMap
        let mut result: HashMap<&'a Self::Node, HashSet<&'a Self::Node>> = 
            self.nodes().map(|u| (u, HashSet::new())).collect();

        for (u_id, v_id) in pi {
            // 由于我们事先在字典中存过映射，这里可以安全取值
            let u_node = id_to_u[&u_id];
            let v_node = id_to_v[&v_id];
            if let Some(set) = result.get_mut(u_node) {
                set.insert(v_node);
            }
        }

        result
    }

    fn get_hyper_simulation_effect_pass_by(&'a self, _other: &'a Self, delta: &'a impl Delta<'a, Node = Self::Node, Edge = Self::Edge>, d_match: & impl DMatch<'a, Edge = Self::Edge>, type_same_lookup: &HashMap<&'a Self::Node, HashSet<&'a Self::Node>>) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        init_global_logger_once("logs/hyper-simulation.log");

        // 建立 ID 到节点的映射，方便最后构造返回结果
        let mut id_to_u: HashMap<usize, &'a Self::Node> = HashMap::new();
        let mut id_to_v: HashMap<usize, &'a Self::Node> = HashMap::new();

        // 存储节点对与其对应的所有的 Semantic Clusters (HC) 及 D-match 结果
        let mut hc_map: HashMap<(usize, usize), Vec<((usize, usize), HashSet<(usize, usize)>)>> = HashMap::new();
        
        // Pi: 当前满足 Hyper Simulation 条件的 (u.id(), v.id()) 集合
        let mut pi: HashSet<(usize, usize)> = HashSet::new();

        // ==========================================
        // Phase 1: Declarative Initialization
        // ==========================================
        
        // 1. 初始化 Pi 并获取 HC 和 D-match
        for u in self.nodes() {
            id_to_u.insert(u.id(), u);

            if let Some(type_same_vs) = type_same_lookup.get(u) {
                for v in type_same_vs {
                    id_to_v.insert(v.id(), v);

                    let sematic_clusters = delta.get_sematic_clusters(u, v);
                    let mut valid = true;
                    let mut local_clusters = Vec::new();

                    for (cluster_u, cluster_v) in sematic_clusters {
                        let cu_id = cluster_u.id;
                        let cv_id = cluster_v.id;
                        let d_match_set = d_match.d_match(cluster_u, cluster_v);

                        // 条件 2.a: (u, v) 必须在 D-match 中
                        if !d_match_set.contains(&(u.id(), v.id())) {
                            valid = false;
                            break; // 只要有一个 cluster 失败，(u,v) 就不可能在 Pi 中
                        }
                        local_clusters.push(((cu_id, cv_id), d_match_set.clone()));
                    }

                    if valid {
                        pi.insert((u.id(), v.id()));
                        hc_map.insert((u.id(), v.id()), local_clusters);
                    }
                }
            }
        }

        info!("完成了 Pi 的初始化和 HC、D-match 的获取，Pi 大小: {}", pi.len());

        // A_cluster 对应的 D-match 缓存，避免重复计算
        let mut a_cluster_d_match: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
        
        // 依赖索引构建:
        // D_cluster[(Cu, Cv)] -> { (u, v) \in Pi } 
        let mut d_cluster: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
        // D_pair[(u', v')] -> { (Cu, Cv) \in A_cluster }
        let mut d_pair: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

        for (&(u_id, v_id), clusters) in &hc_map {
            for ((cu_id, cv_id), d_match_set) in clusters {
                let c_pair = (*cu_id, *cv_id);
                
                // 填充 D_cluster
                d_cluster.entry(c_pair).or_default().insert((u_id, v_id));

                // 如果这是第一次遇到这个簇对，填充 D_pair
                if !a_cluster_d_match.contains_key(&c_pair) {
                    a_cluster_d_match.insert(c_pair, d_match_set.clone());

                    for &(up_id, vp_id) in d_match_set {
                        d_pair.entry((up_id, vp_id)).or_default().insert(c_pair);
                    }
                }
            }
        }

        info!("1. 初始化 Pi 并获取 HC 和 D-match");

        // 2. 初始化 V_C (Valid Clusters)
        let mut v_c: HashSet<(usize, usize)> = HashSet::new();
        for (c_pair, d_match_set) in &a_cluster_d_match {
            // 条件 2.b: D-match 的所有元素都必须在当前的 Pi 中
            if d_match_set.is_subset(&pi) {
                v_c.insert(*c_pair);
            }
        }

        info!("2. 初始化 V_C (Valid Clusters)");

        // 3. 找出失效的 (u, v) 加入队列 Q
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut pi_retained = pi.clone();

        for &(u_id, v_id) in &pi {
            let mut all_in_vc = true;
            if let Some(clusters) = hc_map.get(&(u_id, v_id)) {
                for (c_pair, _) in clusters {
                    if !v_c.contains(c_pair) {
                        all_in_vc = false;
                        break;
                    }
                }
            }

            if !all_in_vc {
                q.push_back((u_id, v_id));      // 加入 Worklist
                pi_retained.remove(&(u_id, v_id)); // Pi = Pi \ Q
            }
        }
        pi = pi_retained;

        info!("3. 找出失效的 (u, v) 加入队列 Q");

        // ==========================================
        // Phase 2: Cascade deletions via the queue
        // ==========================================
        while let Some((up_id, vp_id)) = q.pop_front() {
            // 获取所有依赖于已删除节点对 (u', v') 的簇对 (Cu, Cv)
            if let Some(dependent_clusters) = d_pair.get(&(up_id, vp_id)) {
                for c_pair in dependent_clusters {
                    // 如果簇对仍然被认为是有效的，现在它失效了
                    if v_c.contains(c_pair) {
                        v_c.remove(c_pair); // V_c = V_c \ {(Cu, Cv)}

                        // 级联使依赖这个失效簇对的 (u, v) 失效
                        if let Some(dependent_node_pairs) = d_cluster.get(c_pair) {
                            for node_pair in dependent_node_pairs {
                                if pi.contains(node_pair) {
                                    pi.remove(node_pair);
                                    q.push_back(*node_pair);
                                }
                            }
                        }
                    }
                }
            }
        }

        info!("结束了主调用");

        // ==========================================
        // Phase 3: Construct Output
        // ==========================================
        // 将基于 ID 的关系还原为引用的 HashMap
        let mut result: HashMap<&'a Self::Node, HashSet<&'a Self::Node>> = 
            self.nodes().map(|u| (u, HashSet::new())).collect();

        for (u_id, v_id) in pi {
            // 由于我们事先在字典中存过映射，这里可以安全取值
            let u_node = id_to_u[&u_id];
            let v_node = id_to_v[&v_id];
            if let Some(set) = result.get_mut(u_node) {
                set.insert(v_node);
            }
        }

        result
    }

    fn get_hyper_simulation_effect_by_id(&'a self, hc_map: &HashMap<(usize, usize), Vec<((usize, usize), HashSet<(usize, usize)>)>>) -> HashSet<(usize, usize)> {
        // ==========================================
        // hc_map 参数说明：
        // ==========================================
        // hc_map 是一个预计算的语义集群和 D-match 关系映射表，用于完全在 ID 空间内执行 Hyper Simulation 计算。
        // 
        // 结构：HashMap<(u_id, v_id), Vec<((cu_id, cv_id), D_match_set)>>
        // 
        // 键 (u_id, v_id)：
        //   - 候选节点对，其中 u_id 属于 self，v_id 属于 other
        //   - 代表可能参加 Hyper Simulation 的节点对
        // 
        // 值 Vec<((cu_id, cv_id), D_match_set)>：
        //   - 该节点对相关的所有语义集群及其 D-match 关系
        //   - (cu_id, cv_id)：语义集群对的 ID（u 侧和 v 侧）
        //   - D_match_set：HashSet<(usize, usize)>，该集群对的 D-match 结果
        //     即满足 d_match(cluster_u, cluster_v) 的所有节点对 (u'_id, v'_id)
        // 
        // 调用前的准备步骤（由调用者负责）：
        //   1. 计算 type_same 的两个图之间所有可兼容的节点对
        //   2. 对每个节点对，调用 delta.get_sematic_clusters() 获取语义集群
        //   3. 对每个集群对，调用 d_match.d_match() 计算 D-match 集合
        //   4. 构建此 hc_map 并传入
        //
        // 函数内执行：
        //   - 完全基于 ID-based 的数据结构，不涉及具体的节点类型
        //   - 执行三个阶段：初始化、V_C 构建、级联删除
        //   - 返回最终的 Hyper Simulation 结果集合
        
        init_global_logger_once("logs/hyper-simulation.log");

        // Pi: 当前满足 Hyper Simulation 条件的 (u.id(), v.id()) 集合
        let mut pi: HashSet<(usize, usize)> = HashSet::new();

        // ==========================================
        // Phase 1: Initialize Pi from hc_map
        // ==========================================
        for &(u_id, v_id) in hc_map.keys() {
            pi.insert((u_id, v_id));
        }

        info!("完成了 Pi 的初始化和 HC、D-match 的获取，Pi 大小: {}", pi.len());

        // A_cluster 对应的 D-match 缓存，避免重复计算
        let mut a_cluster_d_match: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
        
        // 依赖索引构建:
        // D_cluster[(Cu, Cv)] -> { (u, v) \in Pi } 
        let mut d_cluster: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
        // D_pair[(u', v')] -> { (Cu, Cv) \in A_cluster }
        let mut d_pair: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

        for (&(u_id, v_id), clusters) in hc_map.iter() {
            for ((cu_id, cv_id), d_match_set) in clusters {
                let c_pair = (*cu_id, *cv_id);
                
                // 填充 D_cluster
                d_cluster.entry(c_pair).or_default().insert((u_id, v_id));

                // 如果这是第一次遇到这个簇对，填充 D_pair
                if !a_cluster_d_match.contains_key(&c_pair) {
                    a_cluster_d_match.insert(c_pair, d_match_set.clone());

                    for &(up_id, vp_id) in d_match_set {
                        d_pair.entry((up_id, vp_id)).or_default().insert(c_pair);
                    }
                }
            }
        }

        info!("1. 初始化 Pi 并获取 HC 和 D-match");

        // 2. 初始化 V_C (Valid Clusters)
        let mut v_c: HashSet<(usize, usize)> = HashSet::new();
        for (c_pair, d_match_set) in &a_cluster_d_match {
            // 条件 2.b: D-match 的所有元素都必须在当前的 Pi 中
            if d_match_set.is_subset(&pi) {
                v_c.insert(*c_pair);
            }
        }

        info!("2. 初始化 V_C (Valid Clusters)");

        // 3. 找出失效的 (u, v) 加入队列 Q
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut pi_retained = pi.clone();

        for &(u_id, v_id) in &pi {
            let mut all_in_vc = true;
            if let Some(clusters) = hc_map.get(&(u_id, v_id)) {
                for (c_pair, _) in clusters {
                    if !v_c.contains(c_pair) {
                        all_in_vc = false;
                        break;
                    }
                }
            }

            if !all_in_vc {
                q.push_back((u_id, v_id));      // 加入 Worklist
                pi_retained.remove(&(u_id, v_id)); // Pi = Pi \ Q
            }
        }
        pi = pi_retained;

        info!("3. 找出失效的 (u, v) 加入队列 Q");

        // ==========================================
        // Phase 2: Cascade deletions via the queue
        // ==========================================
        while let Some((up_id, vp_id)) = q.pop_front() {
            // 获取所有依赖于已删除节点对 (u', v') 的簇对 (Cu, Cv)
            if let Some(dependent_clusters) = d_pair.get(&(up_id, vp_id)) {
                for c_pair in dependent_clusters {
                    // 如果簇对仍然被认为是有效的，现在它失效了
                    if v_c.contains(c_pair) {
                        v_c.remove(c_pair); // V_c = V_c \ {(Cu, Cv)}

                        // 级联使依赖这个失效簇对的 (u, v) 失效
                        if let Some(dependent_node_pairs) = d_cluster.get(c_pair) {
                            for node_pair in dependent_node_pairs {
                                if pi.contains(node_pair) {
                                    pi.remove(node_pair);
                                    q.push_back(*node_pair);
                                }
                            }
                        }
                    }
                }
            }
        }

        info!("结束了主调用");

        // ==========================================
        // Phase 3: Return ID-based Result
        // ==========================================
        pi
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HyperSimulationTrace {
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

impl IntoIterator for HyperSimulationTrace {
    type Item = HSEvent;
    type IntoIter = std::vec::IntoIter<HSEvent>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.events.into_iter()
    }
}

impl<'a> IntoIterator for &'a HyperSimulationTrace {
    type Item = &'a HSEvent;
    type IntoIter = std::slice::Iter<'a, HSEvent>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.events.iter()
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
pub enum HSEvent {
    Base(usize, HashSet<(usize, usize)>), // current D-Match
    Derivation(usize, HashSet<(usize, usize)>) // D-Match \ Sim
}