#!/bin/bash
# Path to the directory contains files to be processed
source_dir="/data/guangyuh/coding_env/E-Brush/benchmark/AIO"
# Path to the output directory
output_dir="/data/guangyuh/coding_env/E-Brush/result"
# Path to the raw_circuit.txt
raw_circuit_file="/data/guangyuh/coding_env/E-Brush/test_data/raw_circuit.txt"
# Path to the run.py script
run_script="/data/guangyuh/coding_env/E-Brush/run.py"
# Iterate over each file in the source directory
for file in "$source_dir"/*
do
  # Get the base name of the file (without path)
  base_name=$(basename "$file")
  
  # Get the prefix of the file (without extension)
  prefix="${base_name%.*}"
  # Replace the contents of raw_circuit.txt with the contents of the current file
  cp "$file" "$raw_circuit_file"
  # Run the script and redirect the output to a file in the output directory with the same prefix as the processed file
  python "$run_script" > "$output_dir/$prefix.txt"
done