// src/lib.rs
mod parser;


#[derive(Debug)]
pub enum Expr {
    VariableStr(String),
    Lisp(String, Box<Expr>, Box<Expr>), 
    Infix(String, Box<Expr>, Box<Expr>), 
    Not(Box<Expr>), // new
}

pub fn parse_expr<'input>(input: &'input str) -> Result<Expr, lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'input>, &'static str>> {
    let parser = parser::ExprParser::new();
    parser.parse(input)
}