use egg::*;
use std::collections::HashMap;
use crate::utils::{random_gen::*};
use rand::prelude::SliceRandom;
use rand::Rng;
pub struct Extractor1<'a, CF: CostFunction<L>, L: Language, N: Analysis<L>> {
    cost_function: CF,
    costs: HashMap<Id, (CF::Cost, L)>,
    egraph: &'a EGraph<L, N>,
}

impl<'a, CF, L, N> Extractor1<'a, CF, L, N>
where
    CF: CostFunction<L>,
    L: Language,
    N: Analysis<L>,
{
    /// Create a new `Extractor` given an `EGraph` and a
    /// `CostFunction`.
    ///
    /// The extraction does all the work on creation, so this function
    /// performs the greedy search for cheapest representative of each
    /// eclass.
    pub fn new(egraph: &'a EGraph<L, N>, cost_function: CF) ->  Self where <CF as CostFunction<L>>::Cost: Ord {
        let costs = HashMap::default();
        let mut extractor = Extractor1 {
            costs,
            egraph,
            cost_function,
        };
        extractor.find_costs();

        extractor
    }
    pub fn get_node(&self, id: Id) -> &L {
        let random_num = generate_random_float1();
        //println!("random_num{}",random_num);
        if random_num>(0.5 as f64) {
           let eclass=&self.egraph[id];
           let nodes: Vec<&L> = eclass.iter().collect();
           let mut rng = rand::thread_rng();
           let random_index = rng.gen_range(0..nodes.len());
           let random_node = nodes[random_index];
           random_node   
           }
             
          // get random node from class id
         else {
          self.find_best_node(id)
        }
    }
    pub fn find_best_random(&mut self, eclass: Id) -> (CF::Cost, RecExpr<L>) {
        let root = self.costs[&self.egraph.find(eclass)].clone().1;
        let expr = root.build_recexpr(|child| self.get_node(child).clone());
        
        let cost = self.cost_function.cost_rec(&expr);
        (cost,expr)
    }  
    // pub fn find_cost_best_random(&mut self,eclass: Id) ->CF::Cost{   
        
        
    // }
    /// Find the cheapest (lowest cost) represented `RecExpr` in the
    /// given eclass.
    pub fn find_best(&self, eclass: Id) -> (CF::Cost, RecExpr<L>) {
        let (cost, root) = self.costs[&self.egraph.find(eclass)].clone();
        let expr = root.build_recexpr(|id| self.find_best_node(id).clone());
        (cost, expr)
    }
    /// Find the cheapest e-node in the given e-class.
    pub fn find_best_node(&self, eclass: Id) -> &L {
        &self.costs[&self.egraph.find(eclass)].1
    }
    /// Find the cost of the term that would be extracted from this e-class.
    // pub fn find_best_cost(&self, eclass: Id) -> CF::Cost {
    //     let (cost, _) = &self.costs[&self.egraph.find(eclass)];
    //     cost.clone()
    // }
    fn node_total_cost(&mut self, node: &L) -> Option<CF::Cost> {
        let eg = &self.egraph;
        let has_cost = |id| self.costs.contains_key(&eg.find(id));
        if node.all(has_cost) {
            let costs = &self.costs;
            let cost_f = |id| costs[&eg.find(id)].0.clone();
            Some(self.cost_function.cost(node, cost_f))
        } else { 
            None
        }
    }

    fn find_costs(&mut self) where <CF as CostFunction<L>>::Cost: Ord {
        let mut did_something = true;
        while did_something {
            did_something = false;
            for class in self.egraph.classes() {
                let pass = self.make_pass(class);
                // if alpha<=0.8 {
                    match (self.costs.get(&class.id), pass) {
                        (None, Some(new)) => {
                            self.costs.insert(class.id, new);
                            did_something = true;
                        }
                        
    
    
                        (Some(old), Some(new)) if new.0 < old.0 => {
                            self.costs.insert(class.id, new);
                            did_something = true;
                        }
                        _ => (),
                    }
                // }
                // else{
                //     match (self.costs.get(&class.id), pass) {
                //         (None, Some(new)) => {
                //             self.costs.insert(class.id, new);
                //             did_something = true;
                //         }
                        
    
    
                //         (Some(old), Some(new)) if new.0 >= old.0 => {
                //             self.costs.insert(class.id, new);
                //             did_something = true;
                //         }
                //         _ => (),
                //     }
            }
        }
    }



   fn make_pass(&mut self, eclass: &EClass<L, N::Data>) -> Option<(CF::Cost, L)>  where <CF as CostFunction<L>>::Cost: Ord {
    let result: Vec<(CF::Cost, L)> = eclass
        .iter()
        .filter_map(|n| {
            match self.node_total_cost(n) {
                Some(cost) => Some((cost, n.clone())),
                None => None,
            }
        })
        .collect();
    
    let min_cost = result.iter().map(|(cost, _)| cost).cloned().min();

    
    if let Some(min_cost) = min_cost {
        let min_cost_tuples: Vec<(CF::Cost, L)> = result
            .iter()
            .filter(|(cost, _)| cost == &min_cost)
            .cloned()
            .collect();
        let mut rng = rand::thread_rng();
        if let Some(selected_tuple) = min_cost_tuples.choose(&mut rng) {
            return Some(selected_tuple.clone());
            
        }
    }
    
    None
}
        // for class in self.egraph.classes() {
        //     if !self.costs.contains_key(&class.id) {
        //         log::warn!(
        //             "Failed to compute cost for eclass {}: {:?}",
        //             class.id,
        //             class.nodes
        //         )
        //     }
        // }
   // }

}