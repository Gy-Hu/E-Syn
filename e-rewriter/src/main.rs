use egg::*;
use num::Num;

use std::borrow::BorrowMut;
use std::cell::Cell;
use std::env;
use std::f32::INFINITY;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::time::{Duration,Instant};
use num::pow;
use std::fmt::Debug;
use std::f64::consts::PI;
use std::collections::HashSet;
use rand::Rng;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::SliceRandom;
use std::path::Path;

use crate::{Analysis, EClass, EGraph, Id, Language, RecExpr};
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
//type EGraph = egg::EGraph<Prop, ConstantFold>;
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


fn make_rules() -> Vec<Rewrite<Prop, ConstantFold>> {
    let mut rws: Vec<Rewrite<Prop, ConstantFold>> = vec![
        // Boolean theorems of one variable (Table 2.2 pg 62)
        rewrite!("null-element1"; "(* ?b 0)" => "0"),
        rewrite!("null-element2"; "(+ ?b 1)" => "1"),
        rewrite!("complements1"; "(* ?b (! ?b))" => "0"),
        rewrite!("complements2"; "(+ ?b (! ?b))" => "1"),
        rewrite!("covering1"; "(* ?b (+ ?b ?c))" => "?b"),
        rewrite!("covering2"; "(+ ?b (* ?b ?c))" => "?b"),
        rewrite!("combining1"; "(+ (* ?b ?c) (* ?b (! ?c)))" => "?b"),
        rewrite!("combining2"; "(* (+ ?b ?c) (+ ?b (! ?c)))" => "?b")
        // Boolean theorems of several variables (Table 2.3 pg 63)
    ];

    rws.extend(rewrite!("identity1"; "(* ?b 1)" <=> "?b"));
    rws.extend(rewrite!("identity2'"; "(+ ?b 0)" <=> "?b"));
    rws.extend(rewrite!("idempotency1"; "(* ?b ?b)" <=> "?b"));
    rws.extend(rewrite!("idempotency2"; "(+ ?b ?b)" <=> "?b"));
    rws.extend(rewrite!("involution1"; "(! (! ?b))" <=> "?b"));
    rws.extend(rewrite!("commutativity1"; "(* ?b ?c)" <=> "(* ?c ?b)"));
    rws.extend(rewrite!("commutativity2"; "(+ ?b ?c)" <=> "(+ ?c ?b)"));
    rws.extend(rewrite!("associativity1"; "(*(* ?b ?c) ?d)" <=> "(* ?b (* ?c ?d))"));
    rws.extend(rewrite!("associativity2"; "(+(+ ?b ?c) ?d)" <=> "(+ ?b (+ ?c ?d))"));
    rws.extend(rewrite!("distributivity1"; "(+ (* ?b ?c) (* ?b ?d))" <=> "(* ?b (+ ?c ?d))"));
    rws.extend(rewrite!("distributivity2"; "(* (+ ?b ?c) (+ ?b ?d))" <=> "(+ ?b (* ?c ?d))"));
    rws.extend(rewrite!("consensus1"; "(+ (+ (* ?b ?c) (* (! ?b) ?d)) (* ?c ?d))" <=> "(+ (* ?b ?c) (* (! ?b) ?d))"));
    rws.extend(rewrite!("consensus2"; "(* (* (+ ?b ?c) (+ (! ?b) ?d)) (+ ?c ?d))" <=> "(* (+ ?b ?c) (+ (! ?b) ?d))"));
    rws.extend(rewrite!("de-morgan1"; "(! (* ?b ?c))" <=> "(+ (! ?b) (! ?c))"));
    rws.extend(rewrite!("de-morgan2"; "(! (+ ?b ?c))" <=> "(* (! ?b) (! ?c))"));

    rws
}
/* 
fn make_rules() -> Vec<Rewrite<Prop, ConstantFold>> {
    vec![
        //version 1
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
        //version2
    //     rewrite!("identity"; "(* ?b true)" => "?b"),
    //     rewrite!("identity'"; "(+ ?b false)" => "?b"),
    //     rewrite!("null-element"; "(* ?b false)" => "false"),
    //   // rewrite!("null-element"; "false" => "(* ?b false)"),
    //     rewrite!("null-element'"; "(+ ?b true)" => "true"),
    //   // rewrite!("null-element'"; "true"=> "(+ ?b true)"),
    //     rewrite!("idempotency"; "(* ?b ?b)" => "?b"),
    //     rewrite!("idempotency'"; "(+ ?b ?b)" => "?b"),
    //     rewrite!("involution"; "(! (! ?b))" => "?b"),
    //     rewrite!("complements"; "(* ?b (! ?b))" => "false"),
    //     rewrite!("complements'"; "(+ ?b (! ?b))" => "true"),
    //     // Boolean theorems of several variables (Table 2.3 pg 63)
    //     rewrite!("commutativity"; "(* ?b ?c)" => "(* ?c ?b)"),
    //     rewrite!("commutativity'"; "(+ ?b ?c)" => "(+ ?c ?b)"),
    //     rewrite!("associativity"; "(*(* ?b ?c) ?d)" => "(* ?b (* ?c ?d))"),
    //     rewrite!("associativity'"; "(+(+ ?b ?c) ?d)" => "(+ ?b (+ ?c ?d))"),
    //     rewrite!("distributivity"; "(+ (* ?b ?c) (* ?b ?d))" => "(* ?b (+ ?c ?d))"),
    //     rewrite!("distributivity'"; "(* (+ ?b ?c) (+ ?b ?d))" => "(+ ?b (* ?c ?d))"),
    //     rewrite!("covering"; "(* ?b (+ ?b ?c))" => "?b"),
    //     rewrite!("covering'"; "(+ ?b (* ?b ?c))" => "?b"),
    //     rewrite!("combining"; "(+ (* ?b ?c) (* ?b (! ?c)))" => "?b"),
    //     rewrite!("combining'"; "(* (+ ?b ?c) (+ ?b (! ?c)))" => "?b"),
    //     rewrite!("consensus"; "(+ (+ (* ?b ?c) (* (! ?b) ?d)) (* ?c ?d))" => "(+ (* ?b ?c) (* (! ?b) ?d))"),
    //     rewrite!("consensus'"; "(* (* (+ ?b ?c) (+ (! ?b) ?d)) (+ ?c ?d))" => "(* (+ ?b ?c) (+ (! ?b) ?d))"),
    //     rewrite!("de-morgan"; "(! (* ?b ?c))" => "(+ (! ?b) (! ?c))"),
    //     rewrite!("de-morgan'"; "(! (+ ?b ?c))" => "(* (! ?b) (! ?c))"),
    //  version3 
    // rewrite!("a"; "(-> ?a ?b)"      =>       "(+ (! ?a) ?b)"          ),
    // rewrite!("b"; "(! (! ?a))"      =>       "?a"                     ),
    // rewrite!("c"; "(+ ?a (+ ?b ?c))"=> "(+ (+ ?a ?b) ?c)"       ),
    // rewrite!("d"; "(* ?a (+ ?b ?c))"=> "(+ (* ?a ?b) (* ?a ?c))"),
    // rewrite!("e"; "(+ ?a (* ?b ?c))"=> "(* (+ ?a ?b) (+ ?a ?c))"),
    // rewrite!("f"; "(+ ?a ?b)"       =>        "(+ ?b ?a)"              ),
    // rewrite!("r"; "(* ?a ?b)"       =>        "(* ?b ?a)"              ),
    // //rewrite!("q"; "(+ ?a (! ?a))"   =>    "true"                   ) ,
    // //rewrite!("s"; "(+ ?a true)"     =>         "true"                ) ,
    // rewrite!("g"; "(* ?a true)"     =>         "?a"                  ),
    // rewrite!("y"; "(-> ?a ?b)"      =>    "(-> (! ?b) (! ?a))"     ),
    // rewrite!("th1"; "(+ ?x (* ?x ?y))" => "?x"),
    // // Theorem 2: X + !X · Y = X + Y
    // rewrite!("th2"; "(+ ?x (* (! ?x) ?y))" => "(+ ?x ?y)"),
    // // Theorem 3: X · Y + !X · Z + Y · Z = X · Y + !X · Z
    // rewrite!("th3"; "(+ (* ?x ?y) (+ (* (! ?x) ?z) (* ?y ?z)))" => "(+ (* ?x ?y) (* (! ?x) ?z))"),
    // // Theorem 4: X(X + Y) = X
    // rewrite!("th4"; "(* ?x (+ ?x ?y))" => "?x"),
    // // Theorem 5: X(!X + Y) = X · Y
    // rewrite!("th5"; "(* ?x (+ (! ?x) ?y))" => "(* ?x ?y)"),
    // // Theorem 6: (X + Y)(X + !Y) = X
    // rewrite!("th6"; "(* (+ ?x ?y) (+ ?x (! ?y)))" => "?x"),
    // // Theorem 7: (X + Y)(!X + Z) = X · Z + !X · Y
    // rewrite!("th7"; "(* (+ ?x ?y) (+ (! ?x) ?z))" => "(+ (* ?x ?z) (* (! ?x) ?y))"),
    // // Theorem 8: (X + Y)(!X + Z)(Y + Z) = (X + Y)(!X + Z)
    // rewrite!("th8"; "(* (+ ?x ?y) (* (+ (! ?x) ?z) (+ ?y ?z)))" => "(* (+ ?x ?y) (+ (! ?x) ?z))")


    ]
    
}
*/
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

// pub struct OperatorCount;
// impl CostFunction<Prop> for OperatorCount {
//     type Cost = f64;
//     fn cost<C>(&mut self, e: &Prop, mut costs: C) -> Self::Cost
//     where
//         C: FnMut(Id) -> Self::Cost,
//     {
//         let x1 = if let Prop::Or(_) = e { 1.0 } else { 0.0 };
//         let x2 = if let Prop::Not(_) = e { 1.0 } else { 0.0 };
//         let x4 = if let Prop::Concat(_) = e { 1.0 } else { 0.0 };
//         // get AstSize for x5powf
//         let x5 = e.fold(1.0, |sum, id| sum + costs(id));
//         // Implementing the formula
        // let cost = ((cube(x1) * (((x4 * -60.6726861683498) - x5) + 128.3127559231758))
        //     + ((138.49679515109216 + x2) - ((x5 / 0.20209527706229327) / x2)))
        //     + (cos2(cube(x5 / (x2 + 0.024815471829498798)))
        //         * (square(square(-1.549131586967952)) + -1.4213765980328845));
       //  cost
//     }
// }

pub fn calculate_cost(x1: f64, x2: f64, x3:f64, x4: f64, x5: f64,x6:f64) -> f64 {
    //let cost =(((((1.4074399975676353 / (x2 + 0.2620202058844679)) * x1) + 147.29219656957378) + x1) - (E.powf(x2 - cube(x1 - 1.4614726807034428)) - (((x1 + 2.8289664189149817) * ((37.56786118979008 + (x5 - (x5 * x2))) + x5)) + (square(x5) + x5)))) + -1.597589119894574;
    let cost =36.109265171004246*x1 + 1.488470710765137*x1/(x2 + 0.2620202058844679) - (x2 - 3.1031664518592159*((0.68559079218687315*x4 - 1.0).powi(3)).exp()) + 249.43155897921006;
    //let cost =(7.544539131409524*((3.051915820861467*(0.83030548759182525*x2 + 1.0).powi(6)).powi(2)).cos() + 22.532787907770317)*(x1 - 1.335070352638029*(20.603356550299853*(-x1 - 0.7295426217671495*x2).powi(3)).exp() + 10.175872397596697);
    cost
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

// fn simplify(s: &str) -> String {
//     let expr: RecExpr<Prop> = s.parse().unwrap();
//     let mut egraphin = EGraph::new(ConstantFold {});
//     egraphin.add_expr(&expr);
//     //egraphin.dot().to_png("./image/fooin.png").u
//     println!("input node{}", egraphin.total_size());
//     println!("input class{}", egraphin.number_of_classes());

//     // ruuner configure
//     let runner_iteration_limit = 10000000;
//     let egraph_node_limit = 25000000000;
//     let start = Instant::now();



//     let runner = Runner::default()
//         .with_explanations_enabled()
//         .with_expr(&expr)
//         .with_time_limit(std::time::Duration::from_secs(100))
//         .with_iter_limit(runner_iteration_limit)
//         .with_node_limit(egraph_node_limit)
//         .run(&make_rules());
//     let duration = start.elapsed();
//     runner.print_report();
//     println!("Runner stopped: {:?}. Time take for runner: {:?}, Classes: {}, Nodes: {}, Size: {}\n\n",
//             runner.stop_reason, duration, runner.egraph.number_of_classes(),
//             runner.egraph.total_number_of_nodes(), runner.egraph.total_size());


//     let root = runner.roots[0];
//     runner.print_report();
//     let extractor = Extractor::new(&runner.egraph, AstSize);
//     //let extractor = Extractor::new(&runner.egraph, AstSize);
//     //let extractor = Extractor::new(&runner.egraph, OperatorCount);
//     let (best_cost, best) = extractor.find_best(root);
//     let mut egraphout = EGraph::new(ConstantFold {});
//     egraphout.add_expr(&best);
//     println!("output node{}", egraphout.total_size());
//     println!("output class{}", egraphout.number_of_classes());
//     //egraphout.dot().to_png("./image/fooout.png").unwrap();
//     best.to_string()
// }

pub fn generate_random_float() -> f64 {
    let mut rng = rand::thread_rng();
    let random_float: f64 = rng.gen_range(0.0..0.5);
    random_float
}

pub fn generate_random_float1() -> f64 {
    let mut rng = rand::thread_rng();
    let random_float: f64 = rng.gen_range(0.0..1.0);
    random_float
}
pub trait OrdRandom {
    fn max_random(self, other: i32) -> i32;
    fn min_random(self, other: i32) -> i32;
}

impl OrdRandom for i32 {
    fn max_random(self, other: i32) -> i32 {
        match self.cmp(&other) {
            Ordering::Equal => {
                let seed = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs() as u64;
                let mut rng = rand::thread_rng();

                if rng.gen::<bool>() {
                    self
                } else {
                    other
                }
            }
            Ordering::Less | Ordering::Greater => {
                if self < other {
                    other
                } else {
                    self
                }
            }
        }
    }
    fn min_random(self, other: i32) -> i32 {
        match self.cmp(&other) {
            Ordering::Equal => {
                let seed = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs() as u64;
                let mut rng = rand::thread_rng();

                if rng.gen::<bool>() {
                    self
                } else {
                    other
                }
            }
            Ordering::Less | Ordering::Greater => {
                if self < other {
                    self
                } else {
                    other
                }
            }
        }
    }
}

pub fn min_random_cmp<T, F>(v1: T, v2: T, compare: F) -> T
where
    F: FnOnce(&T, &T) -> Ordering,
{
    match compare(&v1, &v2) {
        Ordering::Less | Ordering::Equal => {
            let mut rng = rand::thread_rng();
            if rng.gen::<bool>() {
                v1
            } else {
                v2
            }
        }
        Ordering::Greater => v2,
    }
}




pub trait MyIteratorExt: Iterator {
    fn min_by_random<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering;
}


impl<I> MyIteratorExt for I
where
    I: Iterator,
{
    fn min_by_random<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        #[inline]
        fn fold<T>(mut compare: impl FnMut(&T, &T) -> Ordering) -> impl FnMut(T, T) -> T {
            move |x, y| min_random_cmp(x, y, &mut compare)
        }
    
        self.reduce(fold(compare))
    }
}

fn cmp<T: PartialOrd>(a: &Option<T>, b: &Option<T>) -> Ordering {
    // None is high
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(a), Some(b)) => a.partial_cmp(b).unwrap(),
    }
}

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
                    _=> 1 
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

pub fn count_operators(s: &str) -> HashMap<String, f64> {
    let mut operator_counts = HashMap::new();
    for c in s.chars() {
        match c {
                '*' | '!' | '+' | '-' | '>' | '&' => {
                 let entry = operator_counts.entry(c.to_string()).or_insert(0.0);
                        *entry += 1.0;
                    },
                    _ => {},
                }
            }
            operator_counts
        }


pub fn count_ast_size_and_depth(s: &str) -> (f64, f64) {
    let expr: RecExpr<Prop> = s.parse().unwrap();
    let mut ast_size = AstSize;
    let mut ast_depth = AstDepth;
    let size = ast_size.cost_rec(&expr) as f64;
    let depth = ast_depth.cost_rec(&expr) as f64;
    (size, depth)
}

pub struct Extractor1<'a, CF: CostFunction<L>, L: Language, N: Analysis<L>> {
    cost_function: CF,
    costs: HashMap<Id, (CF::Cost, L)>,
    egraph: &'a EGraph<L, N>,
}

pub trait CostFunction<L: Language> {
    /// The `Cost` type. It only requires `PartialOrd` so you can use
    /// floating point types, but failed comparisons (`NaN`s) will
    /// result in a panic.
    type Cost: PartialOrd + Debug + Clone;

    /// Calculates the cost of an enode whose children are `Cost`s.
    ///
    /// For this to work properly, your cost function should be
    /// _monotonic_, i.e. `cost` should return a `Cost` greater than
    /// any of the child costs of the given enode.
    fn cost<C>(&mut self, enode: &L, costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost;

    /// Calculates the total cost of a [`RecExpr`].
    ///
    /// As provided, this just recursively calls `cost` all the way
    /// down the [`RecExpr`].
    ///
    fn cost_rec(&mut self, expr: &RecExpr<L>) -> Self::Cost {
        let mut costs: HashMap<Id, Self::Cost> = HashMap::default();
        for (i, node) in expr.as_ref().iter().enumerate() {
            let cost = self.cost(node, |i| costs[&i].clone());
            costs.insert(Id::from(i), cost);
        }
        let last_id = Id::from(expr.as_ref().len() - 1);
        costs[&last_id].clone()
    }
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
        let (cost0, root) = self.costs[&self.egraph.find(eclass)].clone();
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
    pub fn find_best_cost(&self, eclass: Id) -> CF::Cost {
        let (cost, _) = &self.costs[&self.egraph.find(eclass)];
        cost.clone()
    }
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




                // }




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
impl<L: Language, N: Analysis<L>> LpCostFunction<L, N> for AstSize {
    fn node_cost(&mut self, _egraph: &EGraph<L, N>, _eclass: Id, _enode: &L) -> f64 {
        1.0
    }
}




pub struct Cellcost;
impl LpCostFunction<Prop, ConstantFold> for Cellcost {
    fn node_cost(&mut self, _egraph: &EGraph<Prop, ConstantFold>, _eclass: Id, _enode: &Prop) -> f64 {
        let op = (*_enode).to_string();
        let op_cost   = match op.as_str() {
            "!" => 9.00 ,
            "+" => 26.0 ,
            "*"=> 22.0 ,
            //"&" => 0.0 as  f32,
            _=> 1.0
        };
        op_cost
    }
   
}



fn main() ->Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let output_path = &args[2];
    let mut input_file = File::open(input_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;
    
    //contents="(! (* (! (* (! a[1]) (! b[1])))(* a[0] b[0])))".to_string();
    let expr: RecExpr<Prop> = contents.parse().unwrap();
    //let mut egraphin = EGraph::new(ConstantFold {});
   //eaphin.add_expr(&expr);
   // egraphin.dot().to_png("/data/cchen/E-Brush/image/fooin.png").unwrap();
   // println!("input node{}", egraphin.total_size());
   // println!("input class{}", egraphin.number_of_classes());

    // ruuner configure
    let runner_iteration_limit = 10000000;
    let egraph_node_limit = 25000000;
    let start = Instant::now();
    let iterations = 10 as i32;
    let runner = Runner::default()
        .with_explanations_enabled()
        .with_expr(&expr)
        .with_time_limit(std::time::Duration::from_secs(100))
        .with_iter_limit(runner_iteration_limit)
        .with_node_limit(egraph_node_limit)
        .run(&make_rules());
    let duration = start.elapsed();
    println!("Runner stopped: {:?}. Time take for runner: {:?}, Classes: {}, Nodes: {}, Size: {}\n\n",
            runner.stop_reason, duration, runner.egraph.number_of_classes(),
            runner.egraph.total_number_of_nodes(), runner.egraph.total_size());
    //let mut unique_solutions = HashSet::new();
   // runner.egraph.dot().to_png("/data/cchen/E-Brush/image/process.png").unwrap();
    let mut results: HashMap<i32, RecExpr<Prop>> = HashMap::new();
    let mut res_cost: HashMap<i32, usize> = HashMap::new();

    for i in 0..iterations+1 {
        let mut extractor = Extractor1::new(&runner.egraph, AstDepth);
        // let mut lp_extractor = LpExtractor::new(&runner.egraph,AstSize);
        // lp_extractor.timeout(500.0); 
        let root = runner.roots[0];
        //let best = lp_extractor.solve(root);
        //let root_node = extractor.get_node(root);
        let (best_cost,best )=extractor.find_best_random(root);
        //let best = root.build_recexpr(|child| extractor.get_node(child));
        
        //println!("best_cost{}", best_cost);
        results.insert(i, best);
        res_cost.insert(i,best_cost);

    }
    // for(key,value)in &res_cost{
    //     println!("Inserted key: {}, value: {}", key, value);
    // }
    // for(key,value)in &results{
    //     println!("Inserted key: {}, value: {}", key, value);
    // }



    // cost function for dealy :
    // ((((((1.4074399975676353 / (x2 + 0.2620202058844679)) * x1) + 147.29219656957378) + x1) - (exp(x2 - cube(x1 - 1.4614726807034428)) - (((x1 + 2.8289664189149817) * ((37.56786118979008 + (x5 - (x5 * x2))) + x5)) + (square(x5) + x5)))) + -1.597589119894574)

    let mut sym_cost_dict: HashMap<i32, f64> = HashMap::new();
    for (key, best) in &results {
        let result_string =best.to_string();
        let (size, depth) = count_ast_size_and_depth(&result_string);
        let operator_counts = count_operators(&result_string);
        let x1 = operator_counts.get("+").copied().unwrap_or(0.0);
        let x2 = operator_counts.get("!").copied().unwrap_or(0.0);
        let x3 = operator_counts.get("*").copied().unwrap_or(0.0);
        let x4 = operator_counts.get("&").copied().unwrap_or(0.0);
        println!("+:{},!:{},*:{},&:{},astsize:{},astdepth:{}",x1,x2,x3,x4,size,depth);

        fn mean(data: &Vec<f64>) -> f64 {
            data.iter().sum::<f64>() / data.len() as f64
        }
        
        fn std_dev(data: &Vec<f64>, mean: f64) -> f64 {
            let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
            variance.sqrt()
        }
        
        fn standardize(data: &Vec<f64>, mean: f64, std_dev: f64) -> Vec<f64> {
            data.iter().map(|&x| (x - mean) / std_dev).collect()
        }
        let x = vec![x1, x2, x3, x4, size, depth];

        let mean = mean(&x);
        let std_dev = std_dev(&x, mean);
        let scaled_data_vec = standardize(&x, mean, std_dev);

     
        let x1_new =scaled_data_vec[0];
        let x2_new =scaled_data_vec[1];
        let x3_new =scaled_data_vec[2];
        let x4_new =scaled_data_vec[3];
        let size_new =scaled_data_vec[4];
        let depth_new =scaled_data_vec[5];
        //println!("+:{},!:{},*:{},&:{},astsize:{},astdepth:{}",x1_new,x2_new,x3_new,x4_new,size_new,depth_new);
        
        let sym_cost = calculate_cost(x1_new,x2_new,x3_new,x4_new,size_new,depth_new);



        sym_cost_dict.insert(*key, sym_cost);
    }
    
    let mut min_key = 0; 
    let mut min_value = INFINITY as f64;
    for (key, &value) in &sym_cost_dict {
        println!("key{},value{}",key,value);
       } // min_key 和 min_value 分别为最小值对应的键和值




    for (key, &value) in &sym_cost_dict {
         if value  < min_value { min_key = *key; min_value = value; } 
        } // min_key 和 min_value 分别为最小值对应的键和值
     println!("best_cost{}",min_value);
    //let mut min_value = f64::INFINITY;
    //let mut min_key = 0; 

    let mut key_value_pairs: Vec<(&i32, &f64)> = sym_cost_dict.iter().collect();
    key_value_pairs.sort_by(|&(_, value1), &(_, value2)| value1.partial_cmp(value2).unwrap());
    // 随机选择 
    //let mut rng = rand::thread_rng();
    //key_value_pairs.shuffle(&mut rng);

    //let Some((min_key, min_value)) = key_value_pairs.first() else { todo!() };
    let min_keys: Vec<&i32> = key_value_pairs.iter().take(10).map(|&(key, _)| key).collect();
    
    // if let Some(min_key) = min_keys.iter().min_by_key(|&&key| sym_cost_dict[key] as i64) {
    let output = results.get(&min_key).map(|result| result.to_string()).unwrap_or_default();
    //let Some(out_rec) = results.get(&min_key)else { todo!() };
    let egraphout = EGraph::new(ConstantFold {});
   // egraphout.add_expr(&out_rec);
  //  egraphin.dot().to_png("/data/cchen/E-Brush/image/fooout.png").unwrap();
    println!("output node{}", egraphout.total_size());
    println!("output class{}", egraphout.number_of_classes());
    let mut output_file = File::create(output_path)?;
    output_file.write(output.as_bytes())?;
    println!("done");
   //}


   let mut count = 0;
   let output_directory = "test_data_beta_runner/";

   for min_key in min_keys.iter() {
       let output = results
           .get(min_key)
           .map(|result| result.to_string())
           .unwrap_or_default();

       // 创建文件名
       let output_file_name = format!("output_from_egg{}.txt", count);
       let output_file_path = Path::new(output_directory).join(output_file_name);

       // 创建并写入文件
       if let Ok(mut output_file) = File::create(output_file_path) {
           output_file.write_all(output.as_bytes()).ok();
       }
       count += 1;
   }
    Ok(())
}