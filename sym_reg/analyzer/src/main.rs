use std::collections::HashMap;
use egg::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::env;



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

fn count_ast_size_and_depth(s: &str) -> (usize, usize) {
    let expr: RecExpr<Prop> = s.parse().unwrap();
    let mut ast_size = AstSize;
    let mut ast_depth = AstDepth;
    let size = ast_size.cost_rec(&expr);
    let depth = ast_depth.cost_rec(&expr);
    (size, depth)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let mut input_file = File::open(input_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;
    let operator_counts = count_operators(&contents);
    for (operator, count) in operator_counts {
        //println!("{}: {}", operator, count);
        if count > 0 {
            println!("{}: {}", operator, count);
        }
        else {
            println!("{}: 0", operator);
        }
    }
    let (size, depth) = count_ast_size_and_depth(&contents);
    //println!("AST size: {}, AST depth: {}", size, depth);
    println!("ASTSize: {}", size);
    println!("ASTDepth: {}", depth);
    Ok(())
}