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
            Prop::Concat([a, b])=>Some((
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
        rewrite!("a"; "(-> ?a ?b)"      =>       "(+ (! ?a) ?b)"          ) ,  
        rewrite!("b"; "(! (! ?a))"      =>       "?a"                     ),
        rewrite!("c"; "(+ ?a (+ ?b ?c))"=> "(+ (+ ?a ?b) ?c)"       ),
        rewrite!("d"; "(* ?a (+ ?b ?c))"=> "(+ (* ?a ?b) (* ?a ?c))"),
        rewrite!("e"; "(+ ?a (* ?b ?c))"=> "(* (+ ?a ?b) (+ ?a ?c))"),
        rewrite!("f"; "(+ ?a ?b)"       =>        "(+ ?b ?a)"              ),
        rewrite!("r"; "(* ?a ?b)"       =>        "(* ?b ?a)"              ),
        //rewrite!("q"; "(+ ?a (! ?a))"   =>    "true"                   ) ,  
        //rewrite!("s"; "(+ ?a true)"     =>         "true"                ) ,     
        rewrite!("g"; "(* ?a true)"     =>         "?a"                  ) ,  
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

fn simplify(s: &str) -> String {
    let expr: RecExpr<Prop> = s.parse().unwrap();
    let mut egraphin =EGraph::new(ConstantFold  {});
    egraphin.add_expr(&expr);
    let runner = Runner::default().with_explanations_enabled().with_expr(&expr).run(&make_rules());
    let root = runner.roots[0];
    let extractor = Extractor::new(&runner.egraph, AstDepth);
    //let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    let mut egraphout =EGraph::new(ConstantFold {});
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