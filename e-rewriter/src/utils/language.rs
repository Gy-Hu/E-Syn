use egg::*;
//use crate::{Analysis, EClass, EGraph, Id, Language, RecExpr};
define_language! {
    pub enum Prop {
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


pub fn make_rules() -> Vec<Rewrite<Prop, ConstantFold>> {
    let mut rws: Vec<Rewrite<Prop, ConstantFold>> = vec![
        // Boolean theorems of one variable (Table 2.2 pg 62)
        rewrite!("null-element1"; "(* ?b 0)" => "0"),
        rewrite!("null-element2"; "(+ ?b 1)" => "1"),
        rewrite!("complements1"; "(* ?b (! ?b))" => "0"),
        rewrite!("complements2"; "(+ ?b (! ?b))" => "1"),
        rewrite!("covering1"; "(* ?b (+ ?b ?c))" => "?b"),
        rewrite!("covering2"; "(+ ?b (* ?b ?c))" => "?b"),
        rewrite!("combining1"; "(+ (* ?b ?c) (* ?b (! ?c)))" => "?b"),
        rewrite!("combining2"; "(* (+ ?b ?c) (+ ?b (! ?c)))" => "?b"),
        //rewrite!("q"; "(+ ?a (! ?a))"   =>    "1"                   ) ,
        //rewrite!("null-element1"; "(* ?b 0)" => "0"),
        //rewrite!("null-element2"; "(+ ?b 1)" => "1"),
        //rewrite!("complements1"; "(* ?b (! ?b))" => "0"),
        //rewrite!("identity1"; "(* ?b 1)" => "?b"),
        //rewrite!("identity2'"; "(+ ?b 0)" => "?b"),
        //rewrite!("involution1"; "(! (! ?a))"      =>       "?a"                     ),
        //rewrite!("associativity2"; "(+ ?a (+ ?b ?c))"=> "(+ (+ ?a ?b) ?c)"       ),
        // rewrite!("d"; "(* ?a (+ ?b ?c))"=> "(+ (* ?a ?b) (* ?a ?c))"),
        // rewrite!("e"; "(+ ?a (* ?b ?c))"=> "(* (+ ?a ?b) (+ ?a ?c))"),
        // //rewrite!("f"; "(+ ?a ?b)"       =>        "(+ ?b ?a)"              ),
        // // rewrite!("r"; "(* ?a ?b)"       =>        "(* ?b ?a)"              ),
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
        // rewrite!("th8"; "(* (+ ?x ?y) (* (+ (! ?x) ?z) (+ ?y ?z)))" => "(* (+ ?x ?y) (+ (! ?x) ?z))"),
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




//test_rules_most
// pub fn make_rules() -> Vec<Rewrite<Prop, ConstantFold>> {
//     vec![
//         //version 1
//         //rewrite!("a"; "(-> ?a ?b)"      =>       "(+ (! ?a) ?b)"          ),
//         rewrite!("q"; "(+ ?a (! ?a))"   =>    "1"                   ) ,
//         rewrite!("null-element1"; "(* ?b 0)" => "0"),
//         rewrite!("null-element2"; "(+ ?b 1)" => "1"),
//         rewrite!("complements1"; "(* ?b (! ?b))" => "0"),
//         rewrite!("identity1"; "(* ?b 1)" => "?b"),
//         rewrite!("identity2'"; "(+ ?b 0)" => "?b"),


//         rewrite!("involution1"; "(! (! ?a))"      =>       "?a"                     ),
//         rewrite!("associativity2"; "(+ ?a (+ ?b ?c))"=> "(+ (+ ?a ?b) ?c)"       ),
//         rewrite!("d"; "(* ?a (+ ?b ?c))"=> "(+ (* ?a ?b) (* ?a ?c))"),
//         rewrite!("e"; "(+ ?a (* ?b ?c))"=> "(* (+ ?a ?b) (+ ?a ?c))"),
//         rewrite!("f"; "(+ ?a ?b)"       =>        "(+ ?b ?a)"              ),
//         rewrite!("r"; "(* ?a ?b)"       =>        "(* ?b ?a)"              ),


//         rewrite!("th1"; "(+ ?x (* ?x ?y))" => "?x"),
//         // Theorem 2: X + !X · Y = X + Y
//         rewrite!("th2"; "(+ ?x (* (! ?x) ?y))" => "(+ ?x ?y)"),
//         // Theorem 3: X · Y + !X · Z + Y · Z = X · Y + !X · Z
//         rewrite!("th3"; "(+ (* ?x ?y) (+ (* (! ?x) ?z) (* ?y ?z)))" => "(+ (* ?x ?y) (* (! ?x) ?z))"),
//         // Theorem 4: X(X + Y) = X
//         rewrite!("th4"; "(* ?x (+ ?x ?y))" => "?x"),
//         // Theorem 5: X(!X + Y) = X · Y
//         rewrite!("th5"; "(* ?x (+ (! ?x) ?y))" => "(* ?x ?y)"),
//         // Theorem 6: (X + Y)(X + !Y) = X
//         rewrite!("th6"; "(* (+ ?x ?y) (+ ?x (! ?y)))" => "?x"),
//         // Theorem 7: (X + Y)(!X + Z) = X · Z + !X · Y
//         rewrite!("th7"; "(* (+ ?x ?y) (+ (! ?x) ?z))" => "(+ (* ?x ?z) (* (! ?x) ?y))"),
//         // Theorem 8: (X + Y)(!X + Z)(Y + Z) = (X + Y)(!X + Z)
//         rewrite!("th8"; "(* (+ ?x ?y) (* (+ (! ?x) ?z) (+ ?y ?z)))" => "(* (+ ?x ?y) (+ (! ?x) ?z))"),
//         //version2
//     //     rewrite!("identity"; "(* ?b true)" => "?b"),
//     //     rewrite!("identity'"; "(+ ?b false)" => "?b"),
//     //     rewrite!("null-element"; "(* ?b false)" => "false"),
//     //   // rewrite!("null-element"; "false" => "(* ?b false)"),
//     //     rewrite!("null-element'"; "(+ ?b true)" => "true"),
//     //   // rewrite!("null-element'"; "true"=> "(+ ?b true)"),
//     //     rewrite!("idempotency"; "(* ?b ?b)" => "?b"),
//     //     rewrite!("idempotency'"; "(+ ?b ?b)" => "?b"),
//     //     rewrite!("involution"; "(! (! ?b))" => "?b"),
//     //     rewrite!("complements"; "(* ?b (! ?b))" => "false"),
//     //     rewrite!("complements'"; "(+ ?b (! ?b))" => "true"),
//     //     // Boolean theorems of several variables (Table 2.3 pg 63)
//     //     rewrite!("commutativity"; "(* ?b ?c)" => "(* ?c ?b)"),
//     //     rewrite!("commutativity'"; "(+ ?b ?c)" => "(+ ?c ?b)"),
//     //     rewrite!("associativity"; "(*(* ?b ?c) ?d)" => "(* ?b (* ?c ?d))"),
//     //     rewrite!("associativity'"; "(+(+ ?b ?c) ?d)" => "(+ ?b (+ ?c ?d))"),
//     //     rewrite!("distributivity"; "(+ (* ?b ?c) (* ?b ?d))" => "(* ?b (+ ?c ?d))"),
//     //     rewrite!("distributivity'"; "(* (+ ?b ?c) (+ ?b ?d))" => "(+ ?b (* ?c ?d))"),
//     //     rewrite!("covering"; "(* ?b (+ ?b ?c))" => "?b"),
//     //     rewrite!("covering'"; "(+ ?b (* ?b ?c))" => "?b"),
//     //     rewrite!("combining"; "(+ (* ?b ?c) (* ?b (! ?c)))" => "?b"),
//     //     rewrite!("combining'"; "(* (+ ?b ?c) (+ ?b (! ?c)))" => "?b"),
//     //     rewrite!("consensus"; "(+ (+ (* ?b ?c) (* (! ?b) ?d)) (* ?c ?d))" => "(+ (* ?b ?c) (* (! ?b) ?d))"),
//     //     rewrite!("consensus'"; "(* (* (+ ?b ?c) (+ (! ?b) ?d)) (+ ?c ?d))" => "(* (+ ?b ?c) (+ (! ?b) ?d))"),
//     //     rewrite!("de-morgan"; "(! (* ?b ?c))" => "(+ (! ?b) (! ?c))"),
//     //     rewrite!("de-morgan'"; "(! (+ ?b ?c))" => "(* (! ?b) (! ?c))"),
//     //  version3 
//     // rewrite!("a"; "(-> ?a ?b)"      =>       "(+ (! ?a) ?b)"          ),
//     // rewrite!("b"; "(! (! ?a))"      =>       "?a"                     ),
//     // rewrite!("c"; "(+ ?a (+ ?b ?c))"=> "(+ (+ ?a ?b) ?c)"       ),
//     // rewrite!("d"; "(* ?a (+ ?b ?c))"=> "(+ (* ?a ?b) (* ?a ?c))"),
//     // rewrite!("e"; "(+ ?a (* ?b ?c))"=> "(* (+ ?a ?b) (+ ?a ?c))"),
//     // rewrite!("f"; "(+ ?a ?b)"       =>        "(+ ?b ?a)"              ),
//     // rewrite!("r"; "(* ?a ?b)"       =>        "(* ?b ?a)"              ),
//     // //rewrite!("q"; "(+ ?a (! ?a))"   =>    "true"                   ) ,
//     // //rewrite!("s"; "(+ ?a true)"     =>         "true"                ) ,
//     // rewrite!("g"; "(* ?a true)"     =>         "?a"                  ),
//     // rewrite!("y"; "(-> ?a ?b)"      =>    "(-> (! ?b) (! ?a))"     ),
//     // rewrite!("th1"; "(+ ?x (* ?x ?y))" => "?x"),
//     // // Theorem 2: X + !X · Y = X + Y
//     // rewrite!("th2"; "(+ ?x (* (! ?x) ?y))" => "(+ ?x ?y)"),
//     // // Theorem 3: X · Y + !X · Z + Y · Z = X · Y + !X · Z
//     // rewrite!("th3"; "(+ (* ?x ?y) (+ (* (! ?x) ?z) (* ?y ?z)))" => "(+ (* ?x ?y) (* (! ?x) ?z))"),
//     // // Theorem 4: X(X + Y) = X
//     // rewrite!("th4"; "(* ?x (+ ?x ?y))" => "?x"),
//     // // Theorem 5: X(!X + Y) = X · Y
//     // rewrite!("th5"; "(* ?x (+ (! ?x) ?y))" => "(* ?x ?y)"),
//     // // Theorem 6: (X + Y)(X + !Y) = X
//     // rewrite!("th6"; "(* (+ ?x ?y) (+ ?x (! ?y)))" => "?x"),
//     // // Theorem 7: (X + Y)(!X + Z) = X · Z + !X · Y
//     // rewrite!("th7"; "(* (+ ?x ?y) (+ (! ?x) ?z))" => "(+ (* ?x ?z) (* (! ?x) ?y))"),
//     // // Theorem 8: (X + Y)(!X + Z)(Y + Z) = (X + Y)(!X + Z)
//     // rewrite!("th8"; "(* (+ ?x ?y) (* (+ (! ?x) ?z) (+ ?y ?z)))" => "(* (+ ?x ?y) (+ (! ?x) ?z))")


//     ]
    
// }
