use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use rayon::prelude::*;
use std::process;

struct CircuitParser {
    input_file_path: String,
    output_file_path: String,
    new_n_dict: HashMap<String, String>,
    max_iterations: usize,
}

impl CircuitParser {
    fn new(input_file_path: String, output_file_path: String, max_iterations: usize) -> Self {
        CircuitParser {
            input_file_path,
            output_file_path,
            new_n_dict: HashMap::new(),
            max_iterations,
        }
    }

    fn parse_circuit(&mut self) -> String {
        let mut file = File::open(&self.input_file_path).expect("Failed to open input file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read input file");

        let mut lines = contents.lines().collect::<Vec<&str>>();
        let comments = lines[0].trim();
        lines.remove(0);

        let mut output = Vec::new();
        let mut in_order = String::new();
        let mut out_order = String::new();
        let mut current_line = String::new();

        for line in lines {
            let line = line.trim();
            current_line.push_str(" ");
            current_line.push_str(line);

            if line.ends_with(";") {
                if current_line.starts_with(" INORDER") {
                    in_order.push_str(&current_line);
                } else if current_line.starts_with(" OUTORDER") {
                    out_order.push_str(&current_line);
                } else if current_line.starts_with(" new_") {
                    let parts = current_line.split(" = ").collect::<Vec<&str>>();
                    let new_n_name = parts[0].trim().to_string();
                    let new_n_expr = parts[1].trim().trim_end_matches(";").to_string();
                    self.new_n_dict.insert(new_n_name, new_n_expr);
                } else {
                    output.push(self.replace_new_n(&current_line));
                }
                current_line.clear();
            }
        }

        output.insert(0, in_order.trim().to_string());
        output.insert(1, out_order.trim().to_string());

        let mut new_dict = HashMap::new();
        for (new_n_name, new_n_expr) in &self.new_n_dict {
            new_dict.insert(new_n_name.clone(), self.replace_new_n(new_n_expr));
        }
        self.new_n_dict = new_dict;

        //output = output.par_iter().map(|line| self.replace_new_n(line).trim_start().to_string()).collect();
        
        output = output.iter().map(|line| self.replace_new_n(line).trim_start().to_string()).collect();

        output.par_iter_mut().skip(2).for_each(|expr| {
            let parts = expr.split('=').collect::<Vec<&str>>();
            let expr_name = parts[0].trim().to_string();
            let expr_value = parts[1].trim().trim_end_matches(';').to_string();
            let tokens = expr_value.split_whitespace().count();
            if tokens != 1 {
                *expr = format!("{} = ({});", expr_name, expr_value);
            }
        });

        // output.par_iter_mut().skip(2).for_each(|expr| {
        //     *expr = expr.replace("!", "! ");
        // });

        output.insert(0, comments.to_string());

        output.join("\n")
    }

    fn replace_new_n(&self, expr: &str) -> String {
        let mut replaced_expr = expr.to_string();
        for (new_n_name, new_n_expr) in &self.new_n_dict {
            replaced_expr = replaced_expr.replace(new_n_name, &format!("({})", new_n_expr));
        }
        replaced_expr
    }

    fn write_to_file(&self, content: &str) {
        let mut file = File::create(&self.output_file_path).expect("Failed to create output file");
        file.write_all(content.as_bytes()).expect("Failed to write to output file");
    }

    fn process(&mut self) {
        let mut parsed_content = self.parse_circuit();
        let mut last_parsed_content = String::new();
        let mut iterations = 0;
        while parsed_content != last_parsed_content && iterations < self.max_iterations {
            last_parsed_content = parsed_content.clone();
            parsed_content = self.replace_new_n(&parsed_content);
            iterations += 1;
        }

        if last_parsed_content != parsed_content {
        // print we will need to add new_n_dict to file
        println!("Not fully unfold, we will need to add tmp variable to file");
        // write self.new_n_dict to file
        let new_n_dict_str = self.new_n_dict.iter()
            .map(|(k, v)| format!("{} = ({});", k, v))
            .collect::<Vec<String>>()
            .join("\n");

        // append new_n_dict_str to parsed_content
        parsed_content = format!("{}\n{}", parsed_content, new_n_dict_str);
        }
        else {
            println!("EQN fully unfold");
        }
        
        self.write_to_file(&parsed_content);



    }
}

fn read_original_circuit(file_path: &str) -> (String, Vec<String>) {
    let data = std::fs::read_to_string(file_path).expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();
    let formula_list: Vec<String> = lines[3..]
        .iter()
        .map(|line| line.split("= ").collect::<Vec<&str>>()[1].trim_end_matches(';').to_string())
        .collect();
    let mut equations = formula_list.clone();

    let mut num_concat = 0;

    while equations.len() > 1 {
        equations[0] = format!("({} & {})", equations[0], equations[1]);
        num_concat += 1;
        equations.remove(1);
    }

    (equations[0].clone(), formula_list)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Add a new argument for max_iterations
    if args.len() < 4 {
        eprintln!("Usage: {} <input_file_path> <output_file_path> <concat_expr> <max_iterations>", args[0]);
        std::process::exit(1);
    }

    let input_file_path = args[1].clone();
    let output_file_path = args[2].clone();
    let input_for_s_converter = args[3].clone();
    let max_iterations = args[4].parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Error: max_iterations must be a positive integer");
        process::exit(1);
    });

    let mut parser = CircuitParser::new(input_file_path, output_file_path, max_iterations);
    parser.process();

    let (single_equation, formula_list) = read_original_circuit(args[2].as_str());
    let mut file = File::create(&args[3]).unwrap_or_else(|err| {
        eprintln!("Error creating file: {:?}", err);
        process::exit(1);
    });

    file.write_all(single_equation.as_bytes()).unwrap_or_else(|err| {
        eprintln!("Error writing to file: {:?}", err);
        process::exit(1);
    });
}