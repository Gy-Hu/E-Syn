use egg::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;



define_language! {
    enum Prop {
        Bool(bool),
        // character verison
        // "and" = And([Id; 2]),
        // "not" = Not(Id),
        // "or" = Or([Id; 2]),

        //symbolic version
        // "&" = And([Id; 2]),
        // "~" = Not(Id),
        // "|" = Or([Id; 2]),
        // "->" = Implies([Id; 2]),
        // "let" = Let([Id; 2]),
        "*" = And([Id; 2]),
        "!" = Not(Id),
        "+" = Or([Id; 2]),
        "->" = Implies([Id; 2]),
        "let" = Let([Id; 2]),
        "concat" = Concat([Id; 2]),
        //"concat" = Concat(Vec<Id>),
        Symbol(Symbol),
        
    }
}

type EGraph = egg::EGraph<Prop, ConstantFold>;
//type Rewrite = egg::Rewrite<Prop, ConstantFold>;

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
                //format!("(and {} {})", x(a)?, x(b)?).parse().unwrap(),
                format!("(* {} {})", x(a)?, x(b)?).parse().unwrap(),
            )),
            Prop::Not(a) => Some((!x(a)?, format!("(not {})", x(a)?).parse().unwrap())),
            Prop::Or([a, b]) => Some((
                x(a)? || x(b)?,
                //format!("(or {} {})", x(a)?, x(b)?).parse().unwrap(),
                format!("(+ {} {})", x(a)?, x(b)?).parse().unwrap(),
            )),
            Prop::Implies([a, b]) => Some((
                !x(a)? || x(b)?,
                format!("(-> {} {})", x(a)?, x(b)?).parse().unwrap(),
            )),
            Prop::Concat([a, b])=>Some((
                x(a)? > x(b)?,
                format!("(concat {} {})", x(a)?, x(b)?).parse().unwrap(),
            )),
            // Prop::Concat(_)=>one((
            //     enode.clone(),
            //     format!("(concat {}",enode.clone()).parse().unwrap(),
            // )),
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
        // not and or verision 
        // rewrite!("a"; "(-> ?a ?b)"      =>       "(or (not ?a) ?b)"          ) ,  
        // rewrite!("b"; "(not (not ?a))"      =>       "?a"                     ),
        // rewrite!("c"; "(or ?a (or ?b ?c))"=> "(or (or ?a ?b) ?c)"       ),
        // rewrite!("d"; "(and ?a (or ?b ?c))"=> "(or (and ?a ?b) (and ?a ?c))"),
        // rewrite!("e"; "(or ?a (and ?b ?c))"=> "(and (or ?a ?b) (or ?a ?c))"),
        // rewrite!("f"; "(or ?a ?b)"       =>        "(or ?b ?a)"              ),
        // rewrite!("r"; "(and ?a ?b)"       =>        "(and ?b ?a)"              ),
        // rewrite!("q"; "(or ?a (not ?a))"   =>    "true"                   ) ,  
        // rewrite!("s"; "(or ?a true)"     =>         "true"                ) ,     
        // rewrite!("g"; "(and ?a true)"     =>         "?a"                  ) ,  
        // rewrite!("y"; "(-> ?a ?b)"      =>    "(-> (not ?b) (not ?a))"     ),
        // rewrite!("z"; "(or ?a ?a)" => "(?a)"),
        // rewrite!("h"; "(and ?a false)"    =>         "false"                ) , 
        // rewrite!("i"; "(or ?a false)"    =>         "?a"                   ) , 
        //symbolic version
        
        // rewrite!("a"; "(-> ?a ?b)"      =>       "(| (~ ?a) ?b)"          ) ,  
        // rewrite!("b"; "(~ (~ ?a))"      =>       "?a"                     ),
        // rewrite!("c"; "(| ?a (| ?b ?c))"=> "(| (| ?a ?b) ?c)"       ),
        // rewrite!("d"; "(& ?a (| ?b ?c))"=> "(| (& ?a ?b) (& ?a ?c))"),
        // rewrite!("e"; "(| ?a (& ?b ?c))"=> "(& (| ?a ?b) (| ?a ?c))"),
        // rewrite!("f"; "(| ?a ?b)"       =>        "(| ?b ?a)"              ),
        // rewrite!("r"; "(& ?a ?b)"       =>        "(& ?b ?a)"              ),
        // rewrite!("q"; "(| ?a (~ ?a))"   =>    "true"                   ) ,  
        // rewrite!("s"; "(| ?a true)"     =>         "true"                ) ,     
        // rewrite!("g"; "(& ?a true)"     =>         "?a"                  ) ,  
        // rewrite!("y"; "(-> ?a ?b)"      =>    "(-> (~ ?b) (~ ?a))"     ),
        // rewrite!("z"; "(| ?a ?a)" => "(?a)"),
        // rewrite!("h"; "(& ?a false)"    =>         "false"                ) , 
        // rewrite!("i"; "(| ?a false)"    =>         "?a"                   ) , 
        
        //original version
        // rewrite!("a"; "(-> ?a ?b)"      =>       "(| (~ ?a) ?b)"          ) ,  
        // rewrite!("b"; "(~ (~ ?a))"      =>       "?a"                     ),
        // rewrite!("c"; "(| ?a (| ?b ?c))"=> "(| (| ?a ?b) ?c)"       ),
        // rewrite!("d"; "(& ?a (| ?b ?c))"=> "(| (& ?a ?b) (& ?a ?c))"),
        // rewrite!("e"; "(| ?a (& ?b ?c))"=> "(& (| ?a ?b) (| ?a ?c))"),
        // rewrite!("f"; "(| ?a ?b)"       =>        "(| ?b ?a)"              ),
        // rewrite!("r"; "(& ?a ?b)"       =>        "(& ?b ?a)"              ),
        // rewrite!("q"; "(| ?a (~ ?a))"   =>    "true"                   ) ,  
        // rewrite!("s"; "(| ?a true)"     =>         "true"                ) ,     
        // rewrite!("g"; "(& ?a true)"     =>         "?a"                  ) ,  
        // rewrite!("y"; "(-> ?a ?b)"      =>    "(-> (~ ?b) (~ ?a))"     ),


        rewrite!("a"; "(-> ?a ?b)"      =>       "(+ (! ?a) ?b)"          ) ,  
        rewrite!("b"; "(! (! ?a))"      =>       "?a"                     ),
        rewrite!("c"; "(+ ?a (+ ?b ?c))"=> "(+ (+ ?a ?b) ?c)"       ),
        rewrite!("d"; "(* ?a (+ ?b ?c))"=> "(+ (* ?a ?b) (* ?a ?c))"),
        rewrite!("e"; "(+ ?a (* ?b ?c))"=> "(* (+ ?a ?b) (+ ?a ?c))"),
        rewrite!("f"; "(+ ?a ?b)"       =>        "(+ ?b ?a)"              ),
        rewrite!("r"; "(* ?a ?b)"       =>        "(* ?b ?a)"              ),
        rewrite!("q"; "(+ ?a (! ?a))"   =>    "true"                   ) ,  
        rewrite!("s"; "(+ ?a true)"     =>         "true"                ) ,     
        rewrite!("g"; "(* ?a true)"     =>         "?a"                  ) ,  
        rewrite!("y"; "(-> ?a ?b)"      =>    "(-> (! ?b) (! ?a))"     ),


        // rewrite!("j"; "(and ?a (and ?b ?c))"=> "(and (or ?a ?b) (or ?a ?c))"),
        // rewrite!("k"; "(not (or ?a ?b))"   =>    "(and (not ?a) (not ?b))"       ),  
        // rewrite!("l"; "(not (and ?a ?b))"   =>    "(or (not ?a) (not ?b))"       ),  
        // rewrite!("m"; "(and ?a ?a)" => "(?a)"),
        // rewrite!("t"; "(not true)"        =>       "false"                  ),
        // rewrite!("u"; "(not false)"       =>       "true"                   ),
        // multi_rewrite!("inline-let-and-left"; "?a = (let ?x ?y), ?b = (and ?x ?z)" => "?b = (and ?y ?z)"),
        // multi_rewrite!("inline-let-or-left"; "?a = (let ?x ?y), ?b = (or ?x ?z)" => "?b = (or ?y ?z)"),
        // multi_rewrite!("inline-let-and-right"; "?a = (let ?x ?y), ?b = (and ?z ?x)" => "?b = (and ?z ?y)"),
        // multi_rewrite!("inline-let-or-right"; "?a = (let ?x ?y), ?b = (or ?z ?x)" => "?b = (or ?z ?y)"),
        // multi_rewrite!("inline-let-not"; "?a = (let ?x ?y), ?b = (not ?x)" => "?b = (not ?y)"),
        // Theorem 1: X + X · Y = X
        // rewrite!("th1"; "(| ?x (& ?x ?y))" => "?x"),

        // // Theorem 2: X + ~X · Y = X + Y
        // rewrite!("th2"; "(| ?x (& (~ ?x) ?y))" => "(| ?x ?y)"),
        
        // // Theorem 3: X · Y + ~X · Z + Y · Z = X · Y + ~X · Z
        // rewrite!("th3"; "(| (& ?x ?y) (| (& (~ ?x) ?z) (& ?y ?z)))" => "(| (& ?x ?y) (& (~ ?x) ?z))"),
        
        // // Theorem 4: X(X + Y) = X
        // rewrite!("th4"; "(& ?x (| ?x ?y))" => "?x"),
        
        // // Theorem 5: X(~X + Y) = X · Y
        // rewrite!("th5"; "(& ?x (| (~ ?x) ?y))" => "(& ?x ?y)"),
        
        // // Theorem 6: (X + Y)(X + ~Y) = X
        // rewrite!("th6"; "(& (| ?x ?y) (| ?x (~ ?y)))" => "?x"),
        
        // // Theorem 7: (X + Y)(~X + Z) = X · Z + ~X · Y
        // rewrite!("th7"; "(& (| ?x ?y) (| (~ ?x) ?z))" => "(| (& ?x ?z) (& (~ ?x) ?y))"),
        
        // // Theorem 8: (X + Y)(~X + Z)(Y + Z) = (X + Y)(~X + Z)
        // rewrite!("th8"; "(& (| ?x ?y) (& (| (~ ?x) ?z) (| ?y ?z)))" => "(& (| ?x ?y) (| (~ ?x) ?z))"),
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

/// parse an expression, simplify it using egg, and pretty print it back out
fn simplify(s: &str) -> String {
    // parse the expression, the type annotation tells it which Language to use
    let expr: RecExpr<Prop> = s.parse().unwrap();
    let mut egraphin =EGraph::new(ConstantFold  {});
    egraphin.add_expr(&expr);
    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    egraphin.dot().to_png("fooin.png").unwrap();
  //  let runner = Runner::default().with_egraph(egraphin).run(&make_rules());
    println!("input node{}", egraphin.total_size());
    println!("input class{}", egraphin.number_of_classes());
    
    let runner = Runner::default().with_explanations_enabled().with_expr(&expr).run(&make_rules());
    
 //   println!("{:?}", runner); 
    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0];
    runner.print_report();
    runner.egraph.dot().to_png("fooegraph.png").unwrap();
    // use an Extractor to pick the best element of the root eclass
    let extractor = Extractor::new(&runner.egraph, AstDepth);
    let (best_cost, best) = extractor.find_best(root);
    let mut egraphout =EGraph::new(ConstantFold {});
    egraphout.add_expr(&best);
    egraphout.dot().to_png("fooout.png").unwrap();
    println!("output node{}", egraphout.total_size());
    println!("output class{}", egraphout.number_of_classes());
   // println!("Simplified {} to {} with cost {}", expr, best, best_cost);
    best.to_string()
}


fn main() -> std::io::Result<()> {
    let mut file = File::open("/data/cchen/egg/test_1.txt")?;
    let mut contents = String::new();
    
    file.read_to_string(&mut contents)?;
    println!("content{}", contents);
    
    let result = simplify(&contents);
    println!("result{}",result);
    let mut file1 = File::create("output.txt")?;
    
    // 将数据写入文件
    file1.write(result.as_bytes())?;

    // let before = contents.parse().unwrap();
    // let after =  result.parse().unwrap();
    // assert_eq!(before,after);
    // let mut eg = EGraph::default();
    //eg.add_expr(&before);
   // eg.rebuild();

  //  assert!(!eg.equivs(&before, &after).is_empty());
    Ok(())
}


















