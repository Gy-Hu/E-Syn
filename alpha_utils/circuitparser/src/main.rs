use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use rayon::prelude::*;

struct CircuitParser {
    input_file_path: String,
    output_file_path: String,
    new_n_dict: HashMap<String, String>,
}

impl CircuitParser {
    fn new(input_file_path: String, output_file_path: String) -> Self {
        CircuitParser {
            input_file_path,
            output_file_path,
            new_n_dict: HashMap::new(),
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
                } else if current_line.starts_with(" new_n") {
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
            *expr = format!("{} = ({});", expr_name, expr_value);
        });

        output.par_iter_mut().skip(2).for_each(|expr| {
            *expr = expr.replace("!", "! ");
        });

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
        while parsed_content != last_parsed_content {
            last_parsed_content = parsed_content.clone();
            parsed_content = self.replace_new_n(&parsed_content);
        }
        self.write_to_file(&parsed_content);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file_path> <output_file_path>", args[0]);
        std::process::exit(1);
    }

    let input_file_path = args[1].clone();
    let output_file_path = args[2].clone();

    let mut parser = CircuitParser::new(input_file_path, output_file_path);
    parser.process();
}