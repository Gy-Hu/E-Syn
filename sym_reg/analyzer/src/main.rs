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
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let mut input_file = File::open(input_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;
    let operator_counts = count_operators(&contents);
    for (operator, count) in operator_counts {
        println!("{}: {}", operator, count);
    }
    Ok(())
}