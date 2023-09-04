import os
import sys
import pandas as pd
import re
from tqdm import tqdm

# The current network is not in a topo order (run "topo").?


def run_aigfuzz(file_count):
    # check aigfuzz/ is esist, if not, create it
    if not os.path.exists("aigfuzz"): os.mkdir("aigfuzz")
    for i in tqdm(range(file_count), desc='Run circuit generator'):
        os.system(f"aigfuzz -c -s > aigfuzz/simple_circuit_{i}.aig")
        os.system(
            f"abc -c \"read_aiger aigfuzz/simple_circuit_{i}.aig; trim ; write_aiger aigfuzz/simple_circuit_{i}.aig\"")
        # os.system(
        #     f"abc -c \"read_aiger aigfuzz/simple_circuit_{i}.aig; topo; trim ; write_aiger aigfuzz/simple_circuit_{i}.aig\"")


def load_circuits(file_count):
    for i in tqdm(range(file_count), desc='Loding circuits and convert to eqn'):
        os.system(
            f"abc -c \"read_aiger aigfuzz/simple_circuit_{i}.aig; write_eqn aigfuzz/simple_circuit_{i}.eqn\"")
        os.system(
            f"aigtoaig aigfuzz/simple_circuit_{i}.aig aigfuzz/simple_circuit_{i}.aag")


def process_circuits(file_count):
    for i in tqdm(range(file_count), desc='Processing circuits for analyzer'):
        print(f"processing No.{i} circuit")
        parser = CircuitParser(
            f"aigfuzz/simple_circuit_{i}.eqn", f"aigfuzz/simple_circuit_{i}_processed.eqn")
        parser.process()
        with open(f"aigfuzz/simple_circuit_{i}_processed.eqn", "r") as myfile:
            data = myfile.readlines()
        _ = run.conver_to_sexpr(
            data, multiple_output=True, output_file_path=f"aigfuzz/simple_circuit_{i}.sexpr")
        os.system(
            f"analyzer/target/debug/analyzer aigfuzz/simple_circuit_{i}.sexpr > aigfuzz/simple_circuit_{i}.data")


def run_abc(file_count):
    for i in tqdm(range(file_count), desc='Running abc to extract stats'):
        os.system(
            f"abc -c \"read_eqn aigfuzz/simple_circuit_{i}_processed.eqn; balance; refactor; print_stats -p; read_lib ../asap7_clean.lib ; map ; stime; \" > aigfuzz/simple_circuit_{i}.stats")


def parse_data(file_count):
    print("---------------------Final Step: Parsing Data---------------------")
    def parser(i):
        with open(f"aigfuzz/simple_circuit_{i}.data", "r") as f:
            data = f.read().split('\n')
            op_dict = {line.split(':')[0]: int(line.split(
                ':')[1].strip()) for line in data if line}
        with open(f"aigfuzz/simple_circuit_{i}.stats", "r") as f:
            stats = f.read()
            power_match = re.search(r"power =\s+(\d+\.\d+)", stats)
            power = float(power_match[1]) if power_match else None
            lev_match = re.search(r"lev =\s+(\d+)", stats)
            lev = int(lev_match[1]) if lev_match else None
            area_match = re.search(r"Area =\s+(\d+\.\d+)", stats)
            area = float(area_match[1]) if area_match else None
            delay_match = re.search(r"Delay =\s+(\d+\.\d+)", stats)
            delay = float(delay_match[1]) if delay_match else None
        return {"power": power, "lev": lev, "area": area, "delay": delay, **op_dict}

    df = pd.DataFrame([parser(i) for i in range(file_count)])
    # fill na with 0
    df = df.fillna(0)
    # remove rows that `power` or `delay` or `lev` or `area` is 0
    df = df[(df.power != 0) & (df.delay != 0) & (df.lev != 0) & (df.area != 0)]
    # sort the columns as +,!,*,&,lev, ASTSize,ASTDepth, power, area , delay
    df = df.reindex(columns=['+', '!', '*', '&', 'ASTSize',
                             'ASTDepth', 'lev', 'power', 'area', 'delay'])
    
    
    df.to_csv("simple_circuit_analysis_large.csv", index=False)


if __name__ == "__main__":
    #print(sys.path)
    sys.path.append("..")
    #print(sys.path)
    import run 
    from CircuitParser import CircuitParser
    print(run.__file__)
    file_count = 100
    run_aigfuzz(file_count)
    load_circuits(file_count)
    process_circuits(file_count)
    run_abc(file_count)
    parse_data(file_count)
