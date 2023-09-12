// src/main.rs
mod parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process;

#[derive(Debug)]
pub enum Expr {
    VariableStr(String),
    Lisp(String, Box<Expr>, Box<Expr>), 
    Infix(String, Box<Expr>, Box<Expr>), 
    Not(Box<Expr>), // new
}

impl Expr {
    fn to_lisp(&self) -> String {
        match self {
            Expr::VariableStr(n) => n.to_string(),
            Expr::Lisp(op, e1, e2) => format!("({} {} {})", op, e1.to_lisp(), e2.to_lisp()),
            Expr::Infix(op, e1, e2) => format!("({} {} {})", e1.to_lisp(), op, e2.to_lisp()),
            Expr::Not(e) => format!("(! {})", e.to_lisp()), 
        }
    }

    fn to_infix(&self) -> String {
        match self {
            Expr::VariableStr(n) => n.to_string(),
            Expr::Lisp(op, e1, e2) => format!("({} {} {})", e1.to_infix(), op, e2.to_infix()), 
            Expr::Infix(op, e1, e2) => format!("({} {} {})", e1.to_infix(), op, e2.to_infix()),
            Expr::Not(e) => format!("(! {})", e.to_infix()),
        }
    }

    // fn gather_and_expressions(&self, expressions: &mut HashMap<usize, String>, id: &mut usize) {
    //     match self {
    //         Expr::VariableStr(_) => {},
    //         Expr::Lisp(op, e1, e2) => {
    //             if op == "&" {
    //                 e1.gather_and_expressions(expressions, id);
    //                 e2.gather_and_expressions(expressions, id);
    //             } else {
    //                 expressions.insert(*id, self.to_infix());
    //                 *id += 1;
    //             }
    //         },
    //         Expr::Infix(op, e1, e2) => {
    //             if op == "&" {
    //                 e1.gather_and_expressions(expressions, id);
    //                 e2.gather_and_expressions(expressions, id);
    //             } else {
    //                 expressions.insert(*id, self.to_infix());
    //                 *id += 1;
    //             }
    //         },
    //         Expr::Not(e) => e.gather_and_expressions(expressions, id),
    //     }
    // }

    fn gather_and_expressions(&self, expressions: &mut HashMap<usize, String>, id: &mut usize, nested: bool) {
        match self {
            Expr::VariableStr(_) => {
                if !nested {
                    expressions.insert(*id, self.to_infix());
                    *id += 1;
                }
            },
            Expr::Lisp(op, e1, e2) => {
                if op == "&" {
                    e1.gather_and_expressions(expressions, id, true);
                    e2.gather_and_expressions(expressions, id, true);
                } else {
                    expressions.insert(*id, self.to_infix());
                    *id += 1;
                }
            },
            Expr::Infix(op, e1, e2) => {
                if op == "&" {
                    e1.gather_and_expressions(expressions, id, true);
                    e2.gather_and_expressions(expressions, id, true);
                } else {
                    expressions.insert(*id, self.to_infix());
                    *id += 1;
                }
            },
            Expr::Not(e) => e.gather_and_expressions(expressions, id, true),
        }
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_file> <split_concat>/lisp", args[0]);
        process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        eprintln!("Error reading file: {:?}", err);
        process::exit(1);
    });

    let output_file = &args[2];
    //let split_concat = &args[3];

    match parser::ExprParser::new().parse(&input) {
        Ok(expr) => {
            let output = if args.len() > 3 && args[3] == "lisp" {
            expr.to_lisp()
            
            } else {
                expr.to_lisp()
            };

            let mut file = File::create(output_file).unwrap_or_else(|err| {
                eprintln!("Error creating file: {:?}", err);
                process::exit(1);
            });
            file.write_all(output.as_bytes()).unwrap_or_else(|err| {
                eprintln!("Error writing to file: {:?}", err);
                process::exit(1);
            });

            // if args.len() == 3 and args[3] is not "lisp"
        //     if args.len() > 3 && args[3] != "lisp" {
        //     let split_concat = &args[3];
        //     let mut map = HashMap::new();
        //     let mut id = 0;
        //     expr.gather_and_expressions(&mut map, &mut id, false);
        //     //expr.gather_and_expressions(&mut id, &mut map);
        //     let mut file = File::create(split_concat).unwrap_or_else(|err| {
        //         eprintln!("Error creating file: {:?}", err);
        //         process::exit(1);
        //     });
        //     // write line by line (for every key-value pair, one line)
        //     // sort the map by id, 1, 2, 3, ...
        //     let mut map: Vec<_> = map.into_iter().collect();
        //     map.sort_by_key(|k| k.0);
        //     for (key, value) in &map {
        //         file.write_all(format!("{} {}\n", key, value).as_bytes()).unwrap_or_else(|err| {
        //             eprintln!("Error writing to file: {:?}", err);
        //             process::exit(1);
        //         });
        //     }

        // }
        },
        Err(err) => {eprintln!("Error: {:?}", err);
        //print the error expression
       // println!("{:?}", err.0);
    },
    }
}