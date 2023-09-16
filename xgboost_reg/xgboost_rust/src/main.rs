extern crate xgboost;
extern crate sprs;
extern crate env_logger;

use std::io::{BufRead, BufReader};
use std::fs::File;
use xgboost::{parameters, DMatrix, Booster};

fn main() {
    let booster = Booster::load("/data/guangyuh/coding_env/E-Brush/xgboost_reg/xgb_best_model.model").unwrap();
    println!("Booster loaded!");
    // input a 8 dimensional vector
    let input = &[3.0 ,539.0,571.0,1698.0,21.0,17491.0,1118.0, 15.7151841868823 ];

    let num_rows = 1;
    let mut dtest = DMatrix::from_dense(input, num_rows).unwrap();
    //dtest.set_labels(y_test).unwrap();

    //println!("Test matrix rows: {}",  dtest.num_rows());
    // use the booster to predict the input vector
    let prediction = booster.predict(&dtest).unwrap();
    println!("Prediction: {}", prediction[0]);
}