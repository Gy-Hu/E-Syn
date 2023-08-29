use egg::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

use num::pow;
use std::f64::consts::PI;

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
type EGraph = egg::EGraph<Prop, ConstantFold>;
#[derive(Default)]
struct ConstantFold;
impl Analysis<Prop> for ConstantFold {
    type Data = Option<(bool, PatternAst<Prop>)>;
    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            assert_eq!(a.0, b.0, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }
    fn make(egraph: &EGraph, enode: &Prop) -> Self::Data {
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
        println!("Make: {:?} -> {:?}", enode, result);
        result
    }
    fn modify(egraph: &mut EGraph, id: Id) {
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

fn make_rules() -> Vec<Rewrite<Prop, ConstantFold>> {
    vec![
        rewrite!("a"; "(-> ?a ?b)"      =>       "(+ (! ?a) ?b)"          ),
        rewrite!("b"; "(! (! ?a))"      =>       "?a"                     ),
        rewrite!("c"; "(+ ?a (+ ?b ?c))"=> "(+ (+ ?a ?b) ?c)"       ),
        rewrite!("d"; "(* ?a (+ ?b ?c))"=> "(+ (* ?a ?b) (* ?a ?c))"),
        rewrite!("e"; "(+ ?a (* ?b ?c))"=> "(* (+ ?a ?b) (+ ?a ?c))"),
        rewrite!("f"; "(+ ?a ?b)"       =>        "(+ ?b ?a)"              ),
        rewrite!("r"; "(* ?a ?b)"       =>        "(* ?b ?a)"              ),
        //rewrite!("q"; "(+ ?a (! ?a))"   =>    "true"                   ) ,
        //rewrite!("s"; "(+ ?a true)"     =>         "true"                ) ,
        rewrite!("g"; "(* ?a true)"     =>         "?a"                  ),
        rewrite!("y"; "(-> ?a ?b)"      =>    "(-> (! ?b) (! ?a))"     ),
        rewrite!("th1"; "(+ ?x (* ?x ?y))" => "?x"),
        // Theorem 2: X + !X · Y = X + Y
        rewrite!("th2"; "(+ ?x (* (! ?x) ?y))" => "(+ ?x ?y)"),
        // Theorem 3: X · Y + !X · Z + Y · Z = X · Y + !X · Z
        rewrite!("th3"; "(+ (* ?x ?y) (+ (* (! ?x) ?z) (* ?y ?z)))" => "(+ (* ?x ?y) (* (! ?x) ?z))"),
        // Theorem 4: X(X + Y) = X
        rewrite!("th4"; "(* ?x (+ ?x ?y))" => "?x"),
        // Theorem 5: X(!X + Y) = X · Y
        rewrite!("th5"; "(* ?x (+ (! ?x) ?y))" => "(* ?x ?y)"),
        // Theorem 6: (X + Y)(X + !Y) = X
        rewrite!("th6"; "(* (+ ?x ?y) (+ ?x (! ?y)))" => "?x"),
        // Theorem 7: (X + Y)(!X + Z) = X · Z + !X · Y
        rewrite!("th7"; "(* (+ ?x ?y) (+ (! ?x) ?z))" => "(+ (* ?x ?z) (* (! ?x) ?y))"),
        // Theorem 8: (X + Y)(!X + Z)(Y + Z) = (X + Y)(!X + Z)
        rewrite!("th8"; "(* (+ ?x ?y) (* (+ (! ?x) ?z) (+ ?y ?z)))" => "(* (+ ?x ?y) (+ (! ?x) ?z))"),
    ]
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

pub struct OperatorCount;
impl CostFunction<Prop> for OperatorCount {
    type Cost = f64;
    fn cost<C>(&mut self, e: &Prop, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let x1 = if let Prop::Or(_) = e { 1.0 } else { 0.0 };
        let x2 = if let Prop::Not(_) = e { 1.0 } else { 0.0 };
        let x4 = if let Prop::Concat(_) = e { 1.0 } else { 0.0 };
        // get AstSize for x5
        let x5 = e.fold(1.0, |sum, id| sum + costs(id));
        // Implementing the formula
        let cost = ((cube(x1) * (((x4 * -60.6726861683498) - x5) + 128.3127559231758))
            + ((138.49679515109216 + x2) - ((x5 / 0.20209527706229327) / x2)))
            + (cos2(cube(x5 / (x2 + 0.024815471829498798)))
                * (square(square(-1.549131586967952)) + -1.4213765980328845));
        cost
    }
}
fn cube(n: f64) -> f64 {
    pow(n, 3)
}
fn square(n: f64) -> f64 {
    pow(n, 2)
}
fn cos2(n: f64) -> f64 {
    (2.0 * PI * n).cos().powi(2)
}

fn simplify(s: &str) -> String {
    let expr: RecExpr<Prop> = s.parse().unwrap();
    let mut egraphin = EGraph::new(ConstantFold {});
    egraphin.add_expr(&expr);
    let runner = Runner::default()
        .with_explanations_enabled()
        .with_expr(&expr)
        .with_time_limit(std::time::Duration::from_secs(1200))
        .with_iter_limit(1000000)
        .with_node_limit(1000000)
        .run(&make_rules());
    let root = runner.roots[0];
    runner.print_report();
    //let extractor = Extractor::new(&runner.egraph, AstDepth);
    let extractor = Extractor::new(&runner.egraph, AstSize);
    //let extractor = Extractor::new(&runner.egraph, OperatorCount);
    let (best_cost, best) = extractor.find_best(root);
    let mut egraphout = EGraph::new(ConstantFold {});
    egraphout.add_expr(&best);
    best.to_string()
}
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let output_path = &args[2];
    let mut input_file = File::open(input_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;
    let result = simplify(&contents);
    let mut output_file = File::create(output_path)?;
    output_file.write(result.as_bytes())?;
    Ok(())
}
