
use egg::*;
use crate::utils::{language::*};
pub struct Mixcost;
impl CostFunction<Prop> for Mixcost{        
            type Cost = usize;
            fn cost<C>(&mut self, enode: &Prop, mut costs: C) -> Self::Cost
            where
                C: FnMut(Id) -> Self::Cost,
            {   
                let op = enode.to_string();
                let op_cost   = match op.as_str() {
                    "!" => 9 ,
                    "+" => 26 ,
                    "*"=> 22 ,
                    //"&" => 0.0 as  f32,
                    _=> 0 
                };
                //let alpha = generate_random_float();
                //let costsize = (enode.fold(1, |sum, id: Id| sum + f64::from(costs(id))));
                //let costdepth = enode.fold(op_cost, |sum, id: Id| sum + costs(id));
                //let costdepth = 1 + enode.fold(0, |max, id| max.max(costs(id) ));
                let costsize =enode.fold(op_cost, |sum, id| sum + costs(id));
                //println!("enode{},costdepth{}",enode.to_string(),costdepth);
                //let costdepth = op_cost + enode.fold(0, |max, id| max.max_random(costs(id) ));
                //let result = 1.2*costsize+(costdepth*5)as f64;
                let result =costsize;
                //let result =costdepth;
                // println!("cost{}", result);
                //let result = alpha * costsize + (((1.0-alpha)*costdepth as f64) as f64);
                result as usize
            }
            
        }
pub struct AstSize;
impl<L: Language> CostFunction<L> for AstSize {
    type Cost = usize;
    fn cost<C>(&mut self, enode: &L, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        enode.fold(1, |sum, id: Id| sum.saturating_add(costs(id)))
    }
}

pub struct AstDepth;
impl<L: Language> CostFunction<L> for AstDepth {
    type Cost = usize;
    fn cost<C>(&mut self, enode: &L, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        1 + enode.fold(0, |max, id| max.max(costs(id)))
    }
}