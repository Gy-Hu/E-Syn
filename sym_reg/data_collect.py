import os

global file_count

file_count = 3

# run aigfuzz to collect data for symbolic regression
# commend : f"aigfuzz -c -s > aigfuzz/simple_circuit_{i}.aig", save the aig file in ./aigfuzz

for i in range(file_count):
    os.system(f"aigfuzz -c -s > aigfuzz/simple_circuit_{i}.aig")
    os.system(f"abc -c \"read_aiger aigfuzz/simple_circuit_{i}.aig; topo; trim ; write_aiger aigfuzz/simple_circuit_{i}.aig\"")

# load all circuits and using abc to convert to eqn
for i in range(file_count):
    os.system(f"abc -c \"read_aiger aigfuzz/simple_circuit_{i}.aig; write_eqn aigfuzz/simple_circuit_{i}.eqn\"")
    os.system(f"aigtoaig aigfuzz/simple_circuit_{i}.aig aigfuzz/simple_circuit_{i}.aag")

# convert all the eqn to egg readable format
# import library from parent directory ../test.py

import sys
sys.path.append("..")
import run
print(run.__file__)
import CircuitParser

for i in range(file_count):
    print(f"processing No.{i} circuit")
    parser =  CircuitParser.CircuitParser(f"aigfuzz/simple_circuit_{i}.eqn", f"aigfuzz/simple_circuit_{i}.eqn")
    parser.process()
    
    # load file to convert to s-expression (test)
    with open (f"aigfuzz/simple_circuit_{i}.eqn", "r") as myfile:
        # read line by line
        data=myfile.readlines()
        
    _ = run.conver_to_sexpr(data, multiple_output = True, output_file_path=f"aigfuzz/simple_circuit_{i}.sexpr")   

    os.system(f"analyzer/target/debug/analyzer aigfuzz/simple_circuit_{i}.sexpr > aigfuzz/simple_circuit_{i}.data")

# run abc to get the stime, power and area. e.g. read_eqn test_data/original_circuit.txt; balance; refactor; print_stats -p; read_lib asap7_clean.lib ; map ; stime;
for i in range(file_count): # add topo?
    os.system(f"abc -c \"read_eqn aigfuzz/simple_circuit_{i}.eqn; balance; refactor; print_stats -p; read_lib ../asap7_clean.lib ; map ; stime; \" > aigfuzz/simple_circuit_{i}.stats")
    

# parse .data and .stats to get the data, store in pandas dataframe
import pandas as pd
import re
def parser(i):
    # Parse the data file
    with open(f"aigfuzz/simple_circuit_{i}.data", "r") as f:
        data = f.read().split('\n')
        op_dict = {line.split(':')[0]: int(line.split(':')[1].strip()) for line in data if line}
    # Parse the stats file
    with open(f"aigfuzz/simple_circuit_{i}.stats", "r") as f:
        stats = f.read()
        print(stats)
        power = float(re.search(r"power =\s+(\d+\.\d+)", stats).group(1))
        lev = int(re.search(r"lev =\s+(\d+)", stats).group(1))
        area = float(re.search(r"Area =\s+(\d+\.\d+)", stats).group(1))
        delay = float(re.search(r"Delay =\s+(\d+\.\d+)", stats).group(1))
    # Combine the data into a dictionary
    result = {"power": power, "lev": lev, "area": area, "delay": delay, **op_dict}
    return result
# Create a DataFrame for each file and concatenate them
df = pd.DataFrame([parser(i) for i in range(file_count)])
# Display the DataFrame
print(df)
# Save the DataFrame to a CSV file
df.to_csv("simple_circuit_analysis.csv", index=False)