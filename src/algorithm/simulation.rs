use crate::graph::base::{Adjacency, AdjacencyInv, Graph, Directed};
use crate::graph::labeled_graph::Labeled;

use std::cell::RefCell;
use std::collections::{HashSet, HashMap};
pub trait Simulation<'a> {
    type Node: 'a;

    fn get_simulation(&'a self) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>>;

}

impl<'a, 'b, T> Simulation<'a> for T
where 
    T: Graph<'a> + Adjacency<'a> + AdjacencyInv<'a> + Labeled<'a> + Directed,
    T::Node: 'a,
    'b: 'a
{
    type Node = T::Node;

    fn get_simulation(&'a self) -> HashMap<&'a Self::Node, HashSet<&'a Self::Node>> {
        let mut simulation: HashMap<&'a <T as Graph<'_>>::Node, HashSet<&'a <T as Graph<'_>>::Node>> = HashMap::new();
        let remove = RefCell::new(HashMap::new());
        let (adj, adj_inv) = (self.get_adj(), self.get_adj_inv());

        let pre_V = self.nodes().map(|v| self.get_post(&adj, v).collect::<HashSet<_>>()).reduce(|acc, x| acc.union(&x).cloned().collect()).unwrap();

        for v in self.nodes() {
            let sim_v: HashSet<_> = if self.get_post(&adj, v).count() != 0 {
                self.nodes().filter(|u| self.label_same(v, u)).collect()
            } else {
                self.nodes().filter(|u| self.label_same(v, u) && self.get_post(&adj,u).count() != 0).collect()
            };
            simulation.insert(v, sim_v.clone());

            let pre_sim_v = sim_v.into_iter().map(|u| self.get_pre(&adj_inv, u).collect::<HashSet<_>>()).reduce(|acc, x| acc.union(&x).cloned().collect()).unwrap();
            let res: HashSet<_> = pre_V.clone().into_iter().filter(|u| !pre_sim_v.contains(u)).collect();
            remove.borrow_mut().insert(v, res);
        }

        let legal_v = || {
            for v in self.nodes() {
                if remove.borrow().get(v).unwrap().len() != 0 {
                    return Some(v);
                }
            }
            None
        };

        while let Some(v) = legal_v() {
            for u in self.get_pre(&adj_inv,v) {
                for w in remove.borrow().get(v).unwrap() {
                    if simulation.get(u).unwrap().contains(w) {
                        simulation.get_mut(u).unwrap().remove(w);
                        for w_prime in self.get_pre(&adj_inv, w) {
                            if self.get_post(&adj, w_prime).collect::<HashSet<_>>().intersection(simulation.get(u).unwrap()).count() == 0 {
                                remove.borrow_mut().get_mut(u).unwrap().insert(w_prime);
                            }
                        }
                    }

                }
            }
            remove.borrow_mut().get_mut(v).unwrap().clear();
        }

        simulation
    }
}