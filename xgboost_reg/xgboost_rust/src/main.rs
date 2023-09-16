extern crate xgboost;
extern crate sprs;
extern crate env_logger;

use std::io::{BufRead, BufReader};
use std::fs::File;
use xgboost::{parameters, DMatrix, Booster};

fn main() {
    let booster = Booster::load("/data/guangyuh/coding_env/xgboost_py_test/xgb_best_model.model").unwrap();
    println!("Booster loaded!");
}