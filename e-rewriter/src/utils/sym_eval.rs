//use std::f64::consts::PI;
//use num::pow;
use std::collections::HashMap;
use egg::*;
use crate::utils::{language::*};


// fn cube(n: f64) -> f64 {
//     pow(n, 3)
// }
// fn square(n: f64) -> f64 {
    
//     pow(n, 2)
// }

// fn cos2(n: f64) -> f64 {
//     (2.0 * PI * n).cos().powi(2)
// }
pub fn count_operators(s: &str) -> HashMap<String, i32> {
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


pub fn count_ast_size_and_depth(s: &str) -> (i32, i32) {
            let expr: RecExpr<Prop> = s.parse().unwrap();
            let mut ast_size = AstSize;
            let mut ast_depth = AstDepth;
            let size = ast_size.cost_rec(&expr) as i32;
            let depth = ast_depth.cost_rec(&expr)as i32;
            (size, depth)
}
        

pub fn sum_of_liberty_mutiplied_node_number(operator_counts: &HashMap<String, i32>) -> i32 {
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



pub fn sum_of_nodes(operator_counts: &HashMap<String, i32>) -> i32 {
    // sum up all the counts
    let mut sum = 0;
    for (operator, count) in operator_counts {
        sum += count;
    }
    sum
}


pub fn average_liberty_mutiplied_node_number(operator_counts: &HashMap<String, i32>) -> f64 {
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



pub fn calculate_cost(x1: f64, x2: f64, x3:f64, x4: f64, x5: f64,x6:f64) -> f64 {
    //let cost =(((((1.4074399975676353 / (x2 + 0.2620202058844679)) * x1) + 147.29219656957378) + x1) - (E.powf(x2 - cube(x1 - 1.4614726807034428)) - (((x1 + 2.8289664189149817) * ((37.56786118979008 + (x5 - (x5 * x2))) + x5)) + (square(x5) + x5)))) + -1.597589119894574;
    let cost =36.109265171004246*x1 + 1.488470710765137*x1/(x2 + 0.2620202058844679) - (x2 - 3.1031664518592159*((0.68559079218687315*x4 - 1.0).powi(3)).exp()) + 249.43155897921006;
    //let cost =(7.544539131409524*((3.051915820861467*(0.83030548759182525*x2 + 1.0).powi(6)).powi(2)).cos() + 22.532787907770317)*(x1 - 1.335070352638029*(20.603356550299853*(-x1 - 0.7295426217671495*x2).powi(3)).exp() + 10.175872397596697);
    cost
}