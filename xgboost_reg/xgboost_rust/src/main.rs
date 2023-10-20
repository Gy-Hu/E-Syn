use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xgboost::{DMatrix, Booster, parameters};
use csv;
use rand::seq::SliceRandom;
use xgboost::parameters::learning::EvaluationMetric;



fn main() -> Result<(), Box<dyn Error>> {
    let mut best_rmse = f32::INFINITY;
    let mut best_mae = f32::INFINITY;
    let mut best_model: Option<Booster> = None;

    let file = File::open("/data/guangyuh/coding_env/E-Brush/xgboost_reg/fuzz_circuit_analysis_small_size.csv")?;
    let mut reader = csv::Reader::from_reader(BufReader::new(file));

    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?;
        let features: Vec<f32> = record
            .iter()
            .take(8) // first 8 columns are features
            .map(|x| x.parse::<f32>().unwrap())
            .collect();
        let target_area = record[10].parse::<f32>()?; // 12th column is the target
        let target_delay = record[11].parse::<f32>()?; // 13th column is the target
        let target = target_area * 0.4 + target_delay * 0.6;
        data.push((features, target));
    }

    let num_folds = 10;
    let fold_size = data.len() / num_folds;

    let mut rng = rand::thread_rng();
    data.shuffle(&mut rng);

    let mut avg_rmse = 0.0;
    let mut avg_mae = 0.0;

    for fold in 0..num_folds {
        let test_start = fold * fold_size;
        let test_end = test_start + fold_size;

        let (train_data, test_data) = {
            let (left, right) = data.split_at(test_start);
            let (middle, _) = right.split_at(fold_size);
            (left.iter().chain(middle).cloned().collect::<Vec<_>>(), right.to_vec())
        };

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
            .objective(parameters::learning::Objective::GpuRegLinear)
            .eval_metrics(parameters::learning::Metrics::Custom(vec![EvaluationMetric::MAE]))
            //.eval_metrics(parameters::learning::Metrics::Auto)
            .build().unwrap();

        // configure the tree-based learning model's parameters
        let tree_params = parameters::tree::TreeBoosterParametersBuilder::default()
                .max_depth(100)
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
            .boost_rounds(500)
            .booster_params(booster_params)          // model parameters
            .evaluation_sets(Some(evaluation_sets))
            .build()
            .unwrap();

        // let bst = Booster::train(&training_params).unwrap();

        // let preds = bst.predict(&dtest).unwrap();
        // let rmse = (y_test.iter().zip(preds.iter()).map(|(y, y_hat)| (y - y_hat).powi(2)).sum::<f32>() / y_test.len() as f32).sqrt();
        // println!("Fold {}: RMSE = {}", fold + 1, rmse);
        // avg_rmse += rmse;

        let bst = Booster::train(&training_params).unwrap();

        let preds = bst.predict(&dtest).unwrap();
        let rmse = (y_test.iter().zip(preds.iter()).map(|(y, y_hat)| (y - y_hat).powi(2)).sum::<f32>() / y_test.len() as f32).sqrt();
        let mae = (y_test.iter().zip(preds.iter()).map(|(y, y_hat)| (y - y_hat).abs()).sum::<f32>() / y_test.len() as f32);
        println!("Fold {}: RMSE = {}", fold + 1, rmse);
        println!("Fold {}: MAE = {}", fold + 1, mae);
        avg_rmse += rmse;
        avg_mae += mae;

        // if rmse < best_rmse {
        //     best_rmse = rmse;
        //     best_model = Some(bst);
        // }
        if mae < best_mae {
            best_mae = mae;
            best_model = Some(bst);
        }
    }

    avg_rmse /= num_folds as f32;
    avg_mae /= num_folds as f32;
    println!("Average RMSE: {}", avg_rmse);
    println!("Average MAE: {}", avg_mae);

    // Save the best model
    if let Some(best_model) = best_model {
        best_model.save("xgb.model")?;
    }

    // load model and predict
    let bst = Booster::load("xgb.model")?;
    //let x_test = &[0.0,73.0,79.0,232.0,14.0,2395.0,152.0,15.756578947368421]; //test delay
    //let x_test = &[7.0,101.0,88.0,316.0,15.0,3027.0,208.0,15.443877551020408];//test area
    let x_test = &[3.0,2500.0,2435.0,7871.0,268.0,5185.0,0.0013716248478967393,1781.0];
    let num_rows = 1;
   //let y_test = &[53.05]; //test delay
   // let y_test = &[67.65]; //test area
    let y_test = &[162.998];
    let mut dtest = DMatrix::from_dense(x_test, num_rows).unwrap();
    dtest.set_labels(y_test).unwrap();
    println!("Prediction: {:?}", bst.predict(&dtest).unwrap());
    println!("Ground truth: {:?}", y_test);

    Ok(())
}