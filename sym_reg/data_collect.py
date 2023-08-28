import os
import sys
import pandas as pd
import re

# The current network is not in a topo order (run "topo").?


def run_aigfuzz(file_count):
    for i in range(file_count):
        os.system(f"aigfuzz -c -s > aigfuzz/simple_circuit_{i}.aig")
        os.system(
            f"abc -c \"read_aiger aigfuzz/simple_circuit_{i}.aig; trim ; write_aiger aigfuzz/simple_circuit_{i}.aig\"")
        # os.system(
        #     f"abc -c \"read_aiger aigfuzz/simple_circuit_{i}.aig; topo; trim ; write_aiger aigfuzz/simple_circuit_{i}.aig\"")


def load_circuits(file_count):
    for i in range(file_count):
        os.system(
            f"abc -c \"read_aiger aigfuzz/simple_circuit_{i}.aig; write_eqn aigfuzz/simple_circuit_{i}.eqn\"")
        os.system(
            f"aigtoaig aigfuzz/simple_circuit_{i}.aig aigfuzz/simple_circuit_{i}.aag")


def process_circuits(file_count):
    for i in range(file_count):
        print(f"processing No.{i} circuit")
        parser = CircuitParser(
            f"aigfuzz/simple_circuit_{i}.eqn", f"aigfuzz/simple_circuit_{i}.eqn")
        parser.process()
        with open(f"aigfuzz/simple_circuit_{i}.eqn", "r") as myfile:
            data = myfile.readlines()
        _ = run.conver_to_sexpr(
            data, multiple_output=True, output_file_path=f"aigfuzz/simple_circuit_{i}.sexpr")
        os.system(
            f"analyzer/target/debug/analyzer aigfuzz/simple_circuit_{i}.sexpr > aigfuzz/simple_circuit_{i}.data")


def run_abc(file_count):
    for i in range(file_count):
        os.system(
            f"abc -c \"read_eqn aigfuzz/simple_circuit_{i}.eqn; balance; refactor; print_stats -p; read_lib ../asap7_clean.lib ; map ; stime; \" > aigfuzz/simple_circuit_{i}.stats")


def parse_data(file_count):
    def parser(i):
        with open(f"aigfuzz/simple_circuit_{i}.data", "r") as f:
            data = f.read().split('\n')
            op_dict = {line.split(':')[0]: int(line.split(
                ':')[1].strip()) for line in data if line}
        with open(f"aigfuzz/simple_circuit_{i}.stats", "r") as f:
            stats = f.read()
            power = float(re.search(r"power =\s+(\d+\.\d+)", stats).group(1))
            lev = int(re.search(r"lev =\s+(\d+)", stats).group(1))
            area = float(re.search(r"Area =\s+(\d+\.\d+)", stats).group(1))
            delay = float(re.search(r"Delay =\s+(\d+\.\d+)", stats).group(1))
        return {"power": power, "lev": lev, "area": area, "delay": delay, **op_dict}
    df = pd.DataFrame([parser(i) for i in range(file_count)])
    df.to_csv("simple_circuit_analysis.csv", index=False)


if __name__ == "__main__":
    #print(sys.path)
    sys.path.append("..")
    #print(sys.path)
    import run 
    from CircuitParser import CircuitParser
    print(run.__file__)
    file_count = 5
    run_aigfuzz(file_count)
    load_circuits(file_count)
    process_circuits(file_count)
    run_abc(file_count)
    parse_data(file_count)
