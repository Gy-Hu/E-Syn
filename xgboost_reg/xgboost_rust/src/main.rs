use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xgboost::{DMatrix, Booster, parameters};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("/data/guangyuh/coding_env/E-Brush/xgboost_reg/data.csv")?;
    let mut reader = csv::Reader::from_reader(BufReader::new(file));

    let mut x_train = Vec::new();
    let mut y_train = Vec::new();

    for result in reader.records() {
        let record = result?;
        let features: Vec<f32> = record
            .iter()
            .take(8) // first 8 columns are features
            .map(|x| x.parse::<f32>().unwrap())
            .collect();
        x_train.extend(features);
        y_train.push(record[8].parse::<f32>()?); // 9th column is the target
    }

    let num_rows = y_train.len();
    let mut dtrain = DMatrix::from_dense(&x_train, num_rows).unwrap();
    dtrain.set_labels(&y_train).unwrap();

    // Your test data and evaluation_sets, training_params, and model training code should go here
    // test matrix with 1 row
    let x_test = &[0.0,73.0,79.0,232.0,14.0,2395.0,152.0,15.756578947368421];
    let num_rows = 1;
    let y_test = &[53.05];
    let mut dtest = DMatrix::from_dense(x_test, num_rows).unwrap();
    dtest.set_labels(y_test).unwrap();

    // specify datasets to evaluate against during training
    let evaluation_sets = &[(&dtrain, "train"), (&dtest, "test")];

    // specify overall training setup
    let training_params = parameters::TrainingParametersBuilder::default()
        .dtrain(&dtrain)
        .evaluation_sets(Some(evaluation_sets))
        .build()
        .unwrap();

    // train model, and print evaluation data
    let bst = Booster::train(&training_params).unwrap();

    println!("{:?}", bst.predict(&dtest).unwrap());

    Ok(())

}