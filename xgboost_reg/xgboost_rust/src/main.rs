use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xgboost::{DMatrix, Booster, parameters};
use csv;
use rand::seq::SliceRandom;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("/data/guangyuh/coding_env/E-Brush/xgboost_reg/data.csv")?;
    let mut reader = csv::Reader::from_reader(BufReader::new(file));

    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?;
        let features: Vec<f32> = record
            .iter()
            .take(8) // first 8 columns are features
            .map(|x| x.parse::<f32>().unwrap())
            .collect();
        let target = record[8].parse::<f32>()?; // 9th column is the target
        data.push((features, target));
    }

    let split_ratio = 0.8;
    let train_size = (data.len() as f32 * split_ratio).round() as usize;

    let mut rng = rand::thread_rng();
    data.shuffle(&mut rng);

    let (train_data, test_data) = data.split_at(train_size);

    let x_train: Vec<f32> = train_data.iter().flat_map(|(features, _)| features.clone()).collect();
    let y_train: Vec<f32> = train_data.iter().map(|(_, target)| *target).collect();
    let x_test: Vec<f32> = test_data.iter().flat_map(|(features, _)| features.clone()).collect();
    let y_test: Vec<f32> = test_data.iter().map(|(_, target)| *target).collect();

    let num_train_rows = y_train.len();
    let mut dtrain = DMatrix::from_dense(&x_train, num_train_rows).unwrap();
    dtrain.set_labels(&y_train).unwrap();

    let num_test_rows = y_test.len();
    let mut dtest = DMatrix::from_dense(&x_test, num_test_rows).unwrap();
    dtest.set_labels(&y_test).unwrap();

    let evaluation_sets = &[(&dtrain, "train"), (&dtest, "test")];

    // configure objectives, metrics, etc.
    let learning_params = parameters::learning::LearningTaskParametersBuilder::default()
        .objective(parameters::learning::Objective::RegLinear)
        .build().unwrap();

    // configure the tree-based learning model's parameters
    let tree_params = parameters::tree::TreeBoosterParametersBuilder::default()
            .max_depth(10)
            .eta(0.3)
            .build().unwrap();

    // overall configuration for Booster
    let booster_params = parameters::BoosterParametersBuilder::default()
        .booster_type(parameters::BoosterType::Tree(tree_params))
        .learning_params(learning_params)
        .verbose(true)
        .build().unwrap();


    let training_params = parameters::TrainingParametersBuilder::default()
        .dtrain(&dtrain)
        .boost_rounds(100)
        .booster_params(booster_params)          // model parameters
        .evaluation_sets(Some(evaluation_sets))
        .build()
        .unwrap();

    let bst = Booster::train(&training_params).unwrap();

    println!("{:?}", bst.predict(&dtest).unwrap());

    Ok(())
}