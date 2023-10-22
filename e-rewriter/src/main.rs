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
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use csv::ReaderBuilder;
use std::process::Command;
use std::collections::BTreeMap;

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
    let mut results: BTreeMap<i32, RecExpr<Prop>> = BTreeMap::new();
    let mut res_cost: HashMap<i32, usize> = HashMap::new();
    let mut sym_cost_dict: BTreeMap<i32, f64> = BTreeMap::new();

    let root = runner.roots[0];
    let extractor_base_0  = Extractor::new(&runner.egraph, egg::AstDepth);
    let extractor_base_1  = Extractor::new(&runner.egraph, egg::AstSize);
    let mut extractor = Extractor1::new(&runner.egraph, egg::AstDepth);
    let mut extractor1 = Extractor1::new(&runner.egraph, egg::AstSize);
    let mut extractor2 = Extractor1::new(&runner.egraph, Mixcost);

    let (best_cost_base_0,best_base_0 )=extractor_base_0.find_best(root);
    let (best_cost_base_1,best_base_1 )=extractor_base_1.find_best(root);

    results.insert(0, best_base_0.clone());
    results.insert(1, best_base_1.clone()); 

   // let (sym_cost0,input_para0) =xgboost(&(best_base_0.to_string()));
  //  sym_cost_dict.insert(0,sym_cost0);
   // let (sym_cost1,input_para1) =xgboost(&(best_base_1.to_string()));
   // sym_cost_dict.insert(1,sym_cost1);







 
    
    //res_cost.insert(0,best_cost_base_0);    
    
    
   // res_cost.insert(1,best_cost_base_1);    

    for i in 2..iterations*3/2+2 {        
        let (best_cost,best0 )=extractor.find_best_random(root);
       // let ( sym_cost,input_para) =xgboost(&(best0.to_string()));
        results.insert(i, best0.clone());
      //  sym_cost_dict.insert(i,sym_cost);
       // res_cost.insert(i,best_cost);
    }
    for i in iterations*3/2+2..2*iterations+2 {
        let (best_cost,best1 )=extractor.find_best(root);
      //  let ( sym_cost,input_para) =xgboost(&(best1.to_string()));
        results.insert(i, best1.clone());
      //  sym_cost_dict.insert(i,sym_cost);
       // res_cost.insert(i,best_cost);
    }
    for i in 2*iterations+2..7*iterations/2+2 {
        let (best_cost,best2 )=extractor1.find_best_random(root);
      //  let ( sym_cost,input_para) =xgboost(&(best2.to_string()));
        results.insert(i, best2.clone());
      //  sym_cost_dict.insert(i,sym_cost);
        //res_cost.insert(i,best_cost);
    }
    for i in 7*iterations/2+2..4*iterations+2 {
        let (best_cost,best3 )=extractor1.find_best(root);
      //  let ( sym_cost,input_para) =xgboost(&(best3.to_string()));
        results.insert(i, best3.clone());
      //  sym_cost_dict.insert(i,sym_cost);
        //res_cost.insert(i,best_cost);
    }
    // for i in 4*iterations+2..11*iterations/2+2 {
    //     let (best_cost,best )=extractor2.find_best_random(root);
    //     let ( sym_cost,input_para) =xgboost(&(best.to_string()));
    //     results.insert(i, best);
    //     sym_cost_dict.insert(i,sym_cost);
    //     //res_cost.insert(i,best_cost);
    // }
    // for i in 11*iterations/2+2..6*iterations+2 {
    //     let (best_cost,best )=extractor2.find_best(root);
    //     let ( sym_cost,input_para) =xgboost(&(best.to_string()));
    //     results.insert(i, best);
    //     sym_cost_dict.insert(i,sym_cost);
    //     //res_cost.insert(i,best_cost);
    // }
    

    println!("sym_cost_dict");
    for (key, value) in &sym_cost_dict {
        
        println!("Key: {}, Value: {}", key, value);
    }


    let results_vec: Vec<(&i32, &RecExpr<Prop>)> = results.iter().collect();
    results_vec.par_iter().enumerate().for_each(|(count, (key, best))| {
        let result_string = best.to_string();
        let expr: RecExpr<Prop> = result_string.parse().unwrap();
        let mut egraphout = EGraph::new(ConstantFold {});
        egraphout.add_expr(&expr);
        print!("count: {}, key: {}", count, key);
        let output_directory1 = "out_dot/";
        let output_file_name1 = format!("out_graph_dot{}.dot", count);
        let output_file_path1 = Path::new(output_directory1).join(output_file_name1);
        let _ = egraphout.dot().to_dot(output_file_path1);
    });

    results_vec.par_iter().enumerate().for_each(|(count, (key, best))| {
        let result_string = best.to_string();

    
    });
    let output_cmd = Command::new("python")
        .arg("graph_info.py")
        .arg("42")
        .output()
        .expect("Failed to execute command");
    if output_cmd.status.success() {
        let stdout = String::from_utf8_lossy(&output_cmd.stdout);
        println!("Command executed successfully. Output:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output_cmd.stderr);
        println!("Command failed. Error:\n{}", stderr);
    }
    
    let mut results_graph_info: BTreeMap<i32, (f32, f32)> = BTreeMap::new();

    let file = File::open("graph_info/out_graph_info.csv")?;
    let mut reader = ReaderBuilder::new().from_reader(file);

    for result in reader.records() {
        let record = result?;
        let index = record.get(0).unwrap().parse::<i32>()?;
        let density = record.get(1).unwrap().parse::<f32>()?;
        let edge_count = record.get(2).unwrap().parse::<f32>()?;
        results_graph_info.insert(index, (density, edge_count));
    }

    for (key, rec_expr) in &results {
        let mut out_string= rec_expr.to_string();
        if let Some(&(graph_density, graph_edge)) = results_graph_info.get(key){
            let (sym_cost,input_para) =xgboost_new((&out_string),&graph_density,&graph_edge);
            sym_cost_dict.insert(*key,sym_cost);
    }  

       // println!("Key: {}, RecExpr: {:?}", key, rec_expr);
    }

    



   // let Some((density, edge_count)) = results.get(&desired_key)


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

    //non-par version
    

    
    //par versi
   


    let mut min_key = 0; 
    let mut min_value = INFINITY as f64;

    for (key, &value) in &sym_cost_dict {
         if value  < min_value { min_key = *key; min_value = value; } 
        } // min_key 和 min_value 分别为最小值对应的键和值
   //  println!("best_cost{}",min_value);
   let count =0;
   let output_directory = "test_data_beta_runner/";
   let output_file_name = format!("output_from_egg{}.txt",count); 
   let output_file_path = Path::new(output_directory).join(output_file_name);
   let output = results.get(&min_key).expect("Value not found");
   if let Ok(mut output_file) = File::create(output_file_path) {
           output_file.write_all((output.to_string()).as_bytes()).ok();
       }  
    
//     let mut key_value_pairs: Vec<(&i32, &f64)> = sym_cost_dict.iter().collect();
//     key_value_pairs.sort_by(|&(_, value1), &(_, value2)| value1.partial_cmp(value2).unwrap());
//     let min_keys: Vec<&i32> = key_value_pairs.iter().take(30).map(|&(key, _)| key).collect();
//     println!("done");

//    let mut count = 0;
//    let output_directory = "test_data_beta_runner/";
//    for min_key in min_keys.iter() {
//        let output = results
//            .get(min_key)
//            .map(|result| result.to_string())
//            .unwrap_or_default();
//        let output_file_name = format!("output_from_egg{}.txt", count);
//        let output_file_path = Path::new(output_directory).join(output_file_name);
//        if let Ok(mut output_file) = File::create(output_file_path) {
//            output_file.write_all(output.as_bytes()).ok();
//        }
       
//        let (sym_cost,input_para) =xgboost(&output);




//        let file = File::create("test_data_beta_runner/data.csv")?;
//         // let file = OpenOptions::new()
//         //     .create(true)
//         //     .append(true)
//         //     .open("test_data_beta_runner/data.csv")?;
//        let mut writer = WriterBuilder::new().from_writer(file);
        
//        let mut row_data: Vec<String> = input_para
//             .iter()
//             .map(|&value| value.to_string())
//             .collect();
//        let str_key=format!("op{}",count);
//        row_data.push(str_key); 
//        writer.write_record(&row_data)?;
        
//        writer.flush()?;
//        count += 1;
//    }
    Ok(())
}