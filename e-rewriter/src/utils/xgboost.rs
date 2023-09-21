use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xgboost::{DMatrix, Booster, parameters};
use csv;
use rand::seq::SliceRandom;
use crate::utils::{sym_eval::*};


pub fn xgboost(input_string: &str) -> (f64, Vec<f32>) {
    // load model and predict
    let bst = Booster::load("/data/cchen/E-Brush/e-rewriter/src/model/xgb_delay.model");

    let operator_counts = count_operators(&input_string);
    let x1 = operator_counts.get("+").copied().unwrap_or(0);
    let x2 = operator_counts.get("!").copied().unwrap_or(0);
    let x3 = operator_counts.get("*").copied().unwrap_or(0);
    let (size, depth) = count_ast_size_and_depth(&input_string);
    let sum_lib = sum_of_liberty_mutiplied_node_number(&operator_counts);
    let sum_node = sum_of_nodes(&operator_counts);
    let ave_lib = average_liberty_mutiplied_node_number(&operator_counts);
    let x_test = vec![
        x1 as f32,
        x2 as f32,
        x3 as f32,
        size as f32,
        depth as f32,
        sum_lib as f32,
        sum_node as f32,
        ave_lib as f32,
    ];
    let num_rows = 1;
    let y_test = &[53.05];
    let mut dtest = DMatrix::from_dense(x_test.as_slice(), num_rows).unwrap();

    let predict = bst.expect("REASON").predict(&dtest).unwrap()[0];
    let predict_f64 = predict as f64;

    (predict_f64, x_test)
}
