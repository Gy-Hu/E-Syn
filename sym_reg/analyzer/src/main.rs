use std::collections::HashMap;
use egg::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::env;
// mod utils;
// use utils::{language::*};
use std::path::Path;

#[derive(Default)]
pub struct ConstantFold;
impl Analysis<Prop> for ConstantFold {
    type Data = Option<(bool, PatternAst<Prop>)>;
    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            assert_eq!(a.0, b.0, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }
    fn make(egraph: &egg::EGraph<Prop, ConstantFold>, enode: &Prop) -> Self::Data {
        let x = |i: &Id| egraph[*i].data.as_ref().map(|c| c.0);
        let result = match enode {
            Prop::Let([a, b]) => Some((
                x(a) == x(b),
                format!("(let {} {})", x(a)?, x(b)?).parse().unwrap(),
            )),
            Prop::Bool(c) => Some((*c, c.to_string().parse().unwrap())),
            Prop::And([a, b]) => Some((
                x(a)? && x(b)?,
                format!("(* {} {})", x(a)?, x(b)?).parse().unwrap(),
            )),
            Prop::Not(a) => Some((!x(a)?, format!("(not {})", x(a)?).parse().unwrap())),
            Prop::Or([a, b]) => Some((
                x(a)? || x(b)?,
                format!("(+ {} {})", x(a)?, x(b)?).parse().unwrap(),
            )),
            Prop::Implies([a, b]) => Some((
                !x(a)? || x(b)?,
                format!("(-> {} {})", x(a)?, x(b)?).parse().unwrap(),
            )),
            Prop::Concat([a, b]) => Some((
                x(a)? > x(b)?,
                format!("(& {} {})", x(a)?, x(b)?).parse().unwrap(),
            )),
            Prop::Symbol(_) => None,
        };
        //println!("Make: {:?} -> {:?}", enode, result);
        result
    }
    fn modify(egraph: &mut egg::EGraph<Prop, ConstantFold>, id: Id) {
        if let Some(c) = egraph[id].data.clone() {
            egraph.union_instantiations(
                &c.1,
                &c.0.to_string().parse().unwrap(),
                &Default::default(),
                "analysis".to_string(),
            );
        }
    }
}

define_language! {
    enum Prop {
        Bool(bool),
        "*" = And([Id; 2]),
        "!" = Not(Id),
        "+" = Or([Id; 2]),
        "->" = Implies([Id; 2]),
        "let" = Let([Id; 2]),
        "&" = Concat([Id; 2]),
        Symbol(Symbol),
    }
}
fn count_operators(s: &str) -> HashMap<String, i32> {
    let mut operator_counts = HashMap::new();
    for c in s.chars() {
        match c {
            '*' | '!' | '+' | '-' | '>' | '&' => {
                let entry = operator_counts.entry(c.to_string()).or_insert(0);
                *entry += 1;
            },
            _ => {},
        }
    }
    operator_counts
}

/*

- Sum of liberty * node number
- Average path length
- Average AST size
- Average liberty * node number
- Sum of fan-out number multiples delay info on each node
- Total number of nodes
*/

// get hashmap from count_operators and use it to calculate sum_of_liberty_mutiplied_node_number
// input: hashmap from count_operators
fn sum_of_liberty_mutiplied_node_number(operator_counts: &HashMap<String, i32>) -> i32 {
    let mut sum = 0;
    for (operator, count) in operator_counts {
        match operator.as_str() {
            // "!" => 9 ,
            // "+" => 26 ,
            // "*"=> 22 ,
            "!" => sum += 9 * count,
            "+" => sum += 26 * count,
            "*" => sum += 22 * count,
            _ => {},
        }
    }
    sum
}

fn sum_of_nodes(operator_counts: &HashMap<String, i32>) -> i32 {
    // sum up all the counts
    let mut sum = 0;
    for (operator, count) in operator_counts {
        sum += count;
    }
    sum
}

fn average_liberty_mutiplied_node_number(operator_counts: &HashMap<String, i32>) -> f64 {
    let mut sum = 0;
    let mut count = 0;
    for (operator, c) in operator_counts {
        match operator.as_str() {
            "!" => {
                sum += 9 * c;
                count += c;
            },
            "+" => {
                sum += 26 * c;
                count += c;
            },
            "*" => {
                sum += 22 * c;
                count += c;
            },
            _ => {},
        }
    }
    sum as f64 / count as f64
}


//fn sum_of_fan_out_number_multiples_delay_info_on_each_node(s: &str) -> i32 {
    // regex to the node that without (, ), +, !, *, &
    // count the number of the node
    
// pub struct Five_AstDepth;
// impl<L: Language> CostFunction<L> for Five_AstDepth {
//     type Cost = usize;
//     fn cost<C>(&mut self, enode: &L, mut costs: C) -> Self::Cost
//     where
//         C: FnMut(Id) -> Self::Cost,
//     {
//         1 + enode.fold(0, |max, id| max.max(costs(id)))
//     }
// }

pub struct AstSize;
impl<L: Language> CostFunction<L> for AstSize {
    type Cost = usize;
    fn cost<C>(&mut self, enode: &L, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        enode.fold(1, |sum, id| sum.saturating_add(costs(id)))
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

fn count_ast_size_and_depth(s: &str, dot_name: &str) -> (usize, usize) {
    let expr: RecExpr<Prop> = s.parse().unwrap();
    let mut ast_size = AstSize;
    let mut ast_depth = AstDepth;
    let size = ast_size.cost_rec(&expr);
    let depth = ast_depth.cost_rec(&expr);
    //let expr: RecExpr<Prop> = result_string.parse().unwrap();
    let mut egraphout = EGraph::new(ConstantFold {});
    egraphout.add_expr(&expr);
    let output_directory1 = "out_dot/";
    let output_file_name1 = format!("{}_graph_dot.dot",dot_name);
    let output_file_path1 = Path::new(output_directory1).join(output_file_name1);
    let _ = egraphout.dot().to_dot(output_file_path1);
    (size, depth)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let dot_name = &args[2];
    let mut input_file = File::open(input_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;
    let operator_counts = count_operators(&contents);
    for (operator, count) in &operator_counts {
        //println!("{}: {}", operator, count);
        if count > &0 {
            println!("{}: {}", operator, count);
        }
        else {
            println!("{}: 0", operator);
        }
    }
    let (size, depth) = count_ast_size_and_depth(&contents, &dot_name);
    //println!("AST size: {}, AST depth: {}", size, depth);
    println!("ASTSize: {}", size);
    println!("ASTDepth: {}", depth);
    let sum_of_liberty_mutiplied_node_number = sum_of_liberty_mutiplied_node_number(&operator_counts);
    println!("SUM_LIB: {}", sum_of_liberty_mutiplied_node_number);
    let sum_of_nodes = sum_of_nodes(&operator_counts);
    println!("SUM_NODE: {}", sum_of_nodes);
    let average_liberty_mutiplied_node_number = average_liberty_mutiplied_node_number(&operator_counts);
    println!("AVE_LIB: {}", average_liberty_mutiplied_node_number);
    Ok(())
}