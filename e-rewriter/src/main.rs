use egg::*;
use std::env;
use std::f32::INFINITY;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::time::{Instant};
use std::collections::HashMap;
use std::path::Path;
mod utils;
use std::error::Error;
use std::fs::OpenOptions;
use csv::Writer;
use csv::WriterBuilder;
use utils::{language::*,cost::*,sym_eval::*,extractor::*,xgboost::*};
fn main() ->Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let output_path = &args[2];
    let mut input_file = File::open(input_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;
    
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
    let iterations = 30 as i32;
    let runner = Runner::default()
        .with_explanations_enabled()
        .with_expr(&expr)
        .with_time_limit(std::time::Duration::from_secs(300))
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
    

    let root = runner.roots[0];
    let extractor_base_0  = Extractor::new(&runner.egraph, egg::AstDepth);
    let extractor_base_1  = Extractor::new(&runner.egraph, egg::AstSize);
    let mut extractor = Extractor1::new(&runner.egraph, egg::AstDepth);
    let mut extractor1 = Extractor1::new(&runner.egraph, egg::AstSize);
    let mut extractor2 = Extractor1::new(&runner.egraph, Mixcost);

    let (best_cost_base_0,best_base_0 )=extractor_base_0.find_best(root);
    let (best_cost_base_1,best_base_1 )=extractor_base_1.find_best(root);
    results.insert(0, best_base_0);
    res_cost.insert(0,best_cost_base_0);    
    results.insert(1, best_base_1);
    res_cost.insert(1,best_cost_base_1);    

    for i in 2..iterations*3/2+2 {        
        let (best_cost,best )=extractor.find_best_random(root);
        results.insert(i, best);
        res_cost.insert(i,best_cost);
    }
    for i in iterations*3/2+2..2*iterations+2 {
        let (best_cost,best )=extractor.find_best(root);
        results.insert(i, best);
        res_cost.insert(i,best_cost);
    }
    for i in 2*iterations+2..3*iterations*3/2+2 {
        let (best_cost,best )=extractor1.find_best_random(root);
        results.insert(i, best);
        res_cost.insert(i,best_cost);
    }
    for i in 3*iterations*3/2+2..4*iterations+2 {
        let (best_cost,best )=extractor1.find_best(root);
        results.insert(i, best);
        res_cost.insert(i,best_cost);
    }
    for i in 4*iterations+2..5*iterations*3/2+2 {
        let (best_cost,best )=extractor2.find_best_random(root);
        results.insert(i, best);
        res_cost.insert(i,best_cost);
    }
    for i in 5*iterations*3/2+2..6*iterations+2 {
        let (best_cost,best )=extractor2.find_best(root);
        results.insert(i, best);
        res_cost.insert(i,best_cost);
    }

    let mut sym_cost_dict: HashMap<i32, f64> = HashMap::new();
    //version of symbolic_regression


    // for (key, best) in &results {
    //     let result_string =best.to_string();
    //     let (size, depth) = count_ast_size_and_depth(&result_string);
    //     let operator_counts = count_operators(&result_string);
    //     let x1 = operator_counts.get("+").copied().unwrap_or(0.0);
    //     let x2 = operator_counts.get("!").copied().unwrap_or(0.0);
    //     let x3 = operator_counts.get("*").copied().unwrap_or(0.0);
    //     let x4 = operator_counts.get("&").copied().unwrap_or(0.0);
    //     println!("+:{},!:{},*:{},&:{},astsize:{},astdepth:{}",x1,x2,x3,x4,size,depth);

    //     fn mean(data: &Vec<f64>) -> f64 {
    //         data.iter().sum::<f64>() / data.len() as f64
    //     }
        
    //     fn std_dev(data: &Vec<f64>, mean: f64) -> f64 {
    //         let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    //         variance.sqrt()
    //     }
        
    //     fn standardize(data: &Vec<f64>, mean: f64, std_dev: f64) -> Vec<f64> {
    //         data.iter().map(|&x| (x - mean) / std_dev).collect()
    //     }
    //     let x = vec![x1, x2, x3, x4, size, depth];

    //     let mean = mean(&x);
    //     let std_dev = std_dev(&x, mean);
    //     let scaled_data_vec = standardize(&x, mean, std_dev);

     
    //     let x1_new =scaled_data_vec[0];
    //     let x2_new =scaled_data_vec[1];
    //     let x3_new =scaled_data_vec[2];
    //     let x4_new =scaled_data_vec[3];
    //     let size_new =scaled_data_vec[4];
    //     let depth_new =scaled_data_vec[5];
    //     //println!("+:{},!:{},*:{},&:{},astsize:{},astdepth:{}",x1_new,x2_new,x3_new,x4_new,size_new,depth_new);
        
    //     let sym_cost = calculate_cost(x1_new,x2_new,x3_new,x4_new,size_new,depth_new);



    //     sym_cost_dict.insert(*key, sym_cost);
    // }
 for (key, best) in &results {
        let result_string =best.to_string();
        
       // let sym_cost = calculate_cost(x1_new,x2_new,x3_new,x4_new,size_new,depth_new);
        let (sym_cost,input_para) =xgboost(&result_string);
        
    
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
    let mut key_value_pairs: Vec<(&i32, &f64)> = sym_cost_dict.iter().collect();
    key_value_pairs.sort_by(|&(_, value1), &(_, value2)| value1.partial_cmp(value2).unwrap());
    let min_keys: Vec<&i32> = key_value_pairs.iter().take(30).map(|&(key, _)| key).collect();
    let output = results.get(&min_key).map(|result| result.to_string()).unwrap_or_default();
    let egraphout = EGraph::new(ConstantFold {});
   // egraphout.add_expr(&out_rec);
  //  egraphin.dot().to_png("/data/cchen/E-Brush/image/fooout.png").unwrap();
    println!("output node{}", egraphout.total_size());
    println!("output class{}", egraphout.number_of_classes());
    let mut output_file = File::create(output_path)?;
    output_file.write(output.as_bytes())?;
    println!("done");

   let mut count = 0;
   
   let output_directory = "test_data_beta_runner/";
   for min_key in min_keys.iter() {
       let output = results
           .get(min_key)
           .map(|result| result.to_string())
           .unwrap_or_default();
       let output_file_name = format!("output_from_egg{}.txt", count);
       let output_file_path = Path::new(output_directory).join(output_file_name);
       if let Ok(mut output_file) = File::create(output_file_path) {
           output_file.write_all(output.as_bytes()).ok();
       }
       
       let (sym_cost,input_para) =xgboost(&output);
       let file = File::create("test_data_beta_runner/data.csv")?;
        // let file = OpenOptions::new()
        //     .create(true)
        //     .append(true)
        //     .open("test_data_beta_runner/data.csv")?;
       let mut writer = WriterBuilder::new().from_writer(file);
        
       let mut row_data: Vec<String> = input_para
            .iter()
            .map(|&value| value.to_string())
            .collect();
       let str_key=format!("op{}",count);
       row_data.push(str_key); 
       writer.write_record(&row_data)?;
        
       writer.flush()?;
       count += 1;
   }
    Ok(())
}