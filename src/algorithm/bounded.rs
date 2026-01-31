use graph_base::interfaces::graph::{Adjacency, AdjacencyInv, Degree, Directed, Graph};
use graph_base::interfaces::labeled::Labeled;

use std::collections::{HashSet, HashMap};

pub trait BoundedSimulation<'a> {
    type Node: 'a;

    fn get_bounded_simulation(&'a self, other: &'a Self) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;
}

pub trait Bounded<'a>: Graph<'a> {
    fn get_bound(&'a self, u: &'a Self::Node, v: &'a Self::Node) -> usize;
}

impl<'a, 'b, T> BoundedSimulation<'a> for T 
where
    T: Graph<'a> + Bounded<'a> + Degree<'a> + Labeled<'a> + Adjacency<'a> + Degree<'a> + AdjacencyInv<'a> + Directed + 'b,
    T::Node: 'a, T::Edge: 'a,
{
    type Node = T::Node;

    fn get_bounded_simulation(&'a self, other: &'a Self) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {

        let adj_self = self.get_adj();
        let adj_other = other.get_adj();
        let adj_self_inv = self.get_adj_inv();
        // let adj_other_inv = other.get_adj_inv();



        // anc(get_bound(u_prime, u), u_prime, v) that returns v_prime in anc if:
        // 1. label_same(u_prime, v_prime)
        // 2. len(v_prime/.../v) <= get_bound(u_prime, u)
        // dec(get_bound(u, u_prime), u_prime, v) that returns v_prime in dec if:
        // 1. label_same(u_prime, v_prime)
        // 2. len(v/.../v_prime) <= get_bound(u, u_prime)
        // they are HashMap<(usize, &Node, &Node), HashSet<&Node>>

        // We firstly compute the distance matrix M of (V_other, E_other)
        let mut distance: HashMap<(&T::Node, &T::Node), usize> = HashMap::new();
        for u in other.nodes() {
            let mut queue: Vec<(&T::Node, usize)> = vec![(u, 0)];
            let mut visited: HashSet<&T::Node> = HashSet::new();
            visited.insert(u);
            while !queue.is_empty() {
                let (current, dist) = queue.remove(0);
                distance.insert((u, current), dist);
                for neighbor in other.get_post(&adj_other, current) {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor);
                        queue.push((neighbor, dist + 1));
                    }
                }
            }
        }
        
        let mut anc: HashMap<(usize, &T::Node, &T::Node), HashSet<&T::Node>> = HashMap::new();
        let mut dec: HashMap<(usize, &T::Node, &T::Node), HashSet<&T::Node>> = HashMap::new();
        
        
        // Then we compute anc and dec based on distance matrix

        // compute anc
        // anc(bound, u_prime, v) := {v_prime | label_same(u_prime, v_prime) and distance(v_prime, v) <= bound}
        // where u_prime is node from self, v_prime and v are nodes from other
        for u_prime in self.nodes() {
            for u in self.get_post(&adj_self, u_prime) {
                let bound = self.get_bound(u_prime, u);
                for v in other.nodes() {
                    let mut anc_set: HashSet<&T::Node> = HashSet::new();
                    for v_prime in other.nodes() {
                        if self.label_same(u_prime, v_prime) {
                            if let Some(&dist) = distance.get(&(v_prime, v)) {
                                if dist <= bound {
                                    anc_set.insert(v_prime);
                                }
                            }
                        }
                    }
                    anc.insert((bound, u_prime, v), anc_set);
                }
            }
        }

        // compute dec
        // dec(bound, u_prime, v) := {v_prime | label_same(u_prime, v_prime) and distance(v, v_prime) <= bound}
        // where u_prime is node from self, v_prime and v are nodes from other
        // We need to compute dec for all possible (u', u) pairs to get all required bounds
        for u in self.nodes() {
            for u_prime in self.get_post(&adj_self, u) {
                let bound = self.get_bound(u, u_prime);
                for v in other.nodes() {
                    let mut dec_set: HashSet<&T::Node> = HashSet::new();
                    for v_prime in other.nodes() {
                        if self.label_same(u_prime, v_prime) {
                            if let Some(&dist) = distance.get(&(v, v_prime)) {  
                                if dist <= bound {
                                    dec_set.insert(v_prime);
                                }
                            }
                        }
                    }
                    dec.insert((bound, u_prime, v), dec_set);
                }
            }
        }

        let self_out_degree = self.get_out_degree();
        let other_out_degree = other.get_out_degree();

        // sim(u) := {v | v in V_other and label_same(u, v) and out_degree(v) != 0 if out_degree(u) != 0}
        let mut sim = HashMap::new();
        for u in self.nodes() {
            let mut candidates: HashSet<&'a T::Node> = HashSet::new();
            for v in other.nodes() {
                if self.label_same(&u, v) {
                    if self.out_degree(&self_out_degree,&u) != 0 {
                        if other.out_degree(&other_out_degree,&v) != 0 {
                            candidates.insert(v);
                        }
                    } else {
                        candidates.insert(v);
                    }
                }
            }
            sim.insert(u, candidates);
        }

        // presim(u) := {v | v in V_other and there NOT exists (u_prime, u) in E_self 
        // s.t. ((1) v_prime in sim(u), (2) label_same(u_prime, v), and (3) len(v/.../v_prime) <= get_bound(u_prime, u))}
        let mut presim = HashMap::new();
        for u in self.nodes() {
            let mut candidates: HashSet<&'a T::Node> = HashSet::new();
            'v_loop: for v in sim.get(&u).unwrap() {
                if other.out_degree(&other_out_degree, v) == 0 {
                    continue;
                }
                // v 在 presim(u) 中，当且仅当：对所有 u' 都不存在这样的 v'
                // 这里 u' 满足 (u_prime, u) in E_self，即 u' 是 u 的前驱
                for u_prime in self.get_pre(&adj_self_inv, u) {
                    // 首先检查条件 (2)：label_same(u_prime, v)
                    // 如果 v 的标签不等于 u_prime 的标签，则条件 (2) 不满足，v 不可能被排除
                    if !self.label_same(u_prime, v) {
                        continue;  // 这个 u_prime 无法排除 v，检查下一个 u_prime
                    }
                    
                    let bound = self.get_bound(&u_prime, &u);
                    // 检查：是否存在 v' 满足条件
                    // (1) v' in sim(u)
                    // (2) label_same(u_prime, v) - 已经满足（见上面的检查）
                    // (3) len(v/.../v') <= bound (对应 dec)
                    if let Some(dec_set) = dec.get(&(bound, &u_prime, v)) {
                        // dec_set 包含所有标签为 u_prime 且距离 v 在 bound 内的节点
                        // 检查这些节点是否在 sim(u) 中
                        let has_match = dec_set.iter().any(|v_prime| {
                            sim.get(&u).unwrap().contains(v_prime)
                        });
                        // 如果存在这样的 v'，则 v 不在 presim(u) 中
                        if has_match {
                            continue 'v_loop;
                        }
                    }
                    // 如果 dec_set 不存在或为空，则不存在满足条件的 v'，继续检查下一个 u'
                }
                // 所有 u' 都不存在满足条件的 v'，v 在 presim(u) 中
                candidates.insert(v);
            }
            presim.insert(u, candidates);
        }
        
        // while (there exists a node u ∈ V_self with premv(u) != ∅) do 
        //     for (each (u′, u) ∈ E_self and each z ∈ premv(u) ∩ sim(u′)) do 
        //         sim(u′) := sim(u′) \ {z};  
        //         if (sim(u′) = ∅) then return ∅; 
        //             for each u′′ with (u′′, u′) ∈ E_self do 
        //                 for (z′ ∈ anc(get_bound(u′′, u′), u′′, z) ∧ z′ /∈ premv(u′)) do 
        //                     if (dec(get_bound(u′′, u′), u′, z′) ∩ sim(u′) = ∅) 
        //                         then premv(u′) := premv(u′) ∪ {z′}; 
        //     premv(u) := ∅;

        loop {
            // 1. 找到一个 presim 非空的节点 u，如果没有则退出
            let Some(u) = self.nodes().find(|node| !presim.get(node).unwrap().is_empty()) else {
                break;
            };
            
            // 2. 提前复制 premv_u，避免后续借用冲突
            let premv_u = presim.get(&u).unwrap().clone();
            
            // 3. 收集所有 u 的前驱节点 u_prime，即边 (u_prime, u)
            let u_primes: Vec<_> = self.get_pre(&adj_self_inv, &u).collect();
            
            for u_prime in u_primes {
                // 4. 提前收集 sim(u_prime) 和 premv_u 的交集
                let sim_u_prime = sim.get(&u_prime).unwrap();
                let to_remove: Vec<_> = premv_u.intersection(sim_u_prime).cloned().collect();
                
                for z in to_remove {
                    // 5. 现在可以安全地修改 sim
                    sim.get_mut(&u_prime).unwrap().remove(&z);
                    
                    if sim.get(&u_prime).unwrap().is_empty() {
                        return HashMap::new();
                    }
                    
                    // 6. 收集 u_prime 的前驱节点 u_double_prime，即边 (u_double_prime, u_prime)
                    let u_double_primes: Vec<_> = self.get_pre(&adj_self_inv, &u_prime).collect();
                    
                    // 7. 收集所有需要更新的 (u_double_prime, z_prime) 对
                    let mut updates: Vec<(&T::Node, &T::Node)> = Vec::new();
                    
                    // 收集当前 presim(u_prime) 的值，用于检查 z' /∈ presim(u′)
                    let presim_u_prime = presim.get(&u_prime).unwrap().clone();
                    
                    for u_double_prime in u_double_primes {
                        let bound = self.get_bound(&u_double_prime, &u_prime);
                        
                        if let Some(anc_set) = anc.get(&(bound, &u_double_prime, &z)) {
                            // 收集 anc_set 到临时变量
                            let anc_vec: Vec<_> = anc_set.iter().cloned().collect();
                            
                            // 过滤出满足条件的 z_prime: z' ∈ anc(...) ∧ z' /∈ presim(u′)
                            for z_prime in anc_vec.iter() {
                                if !presim_u_prime.contains(z_prime) {
                                    // 检查 dec(...) ∩ sim(u') 是否为空
                                    if let Some(dec_set) = dec.get(&(bound, &u_prime, z_prime)) {
                                        let sim_u_prime_set = sim.get(&u_prime).unwrap();
                                        let has_intersection = dec_set.iter().any(|v| sim_u_prime_set.contains(v));
                                        
                                        if !has_intersection {
                                            updates.push((u_double_prime, *z_prime));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // 8. 批量更新 presim(u_double_prime)
                    for (u_double_prime, z_prime) in updates {
                        presim.get_mut(&u_double_prime).unwrap().insert(z_prime);
                    }
                }
            }
            
            // 9. 清空 presim(u)
            presim.get_mut(&u).unwrap().clear();
        }

        sim
    }
}