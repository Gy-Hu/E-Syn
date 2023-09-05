# E-Brush

## Prerequisite

1. Enter `abc` directory and run `make`
2. Install necessary dependencies in `package.txt`

## Usage

1. Convert the circuit to eqn format by using `write_eqn` in `abc`
2. Copy and paste the eqn to `test_data/raw_circuit.txt`
2. Run `python run.py`

## Benchamrk Description

1. ISCAS benchmark: https://github.com/santoshsmalagi/Benchmarks/tree/main
2. Ripple Carry Adder: Using ABC to generate the circuit ( `gen` command)
3. EPFL benchmark: https://github.com/lsils/benchmarks
4. IWLS 2005: http://iwls.org/iwls2005/benchmarks.html
5. Addtional benchmark1: https://pld.ttu.ee/~maksim/benchmarks/
6. Addtional benchmark2: https://github.com/jpsety/verilog_benchmark_circuits

## TODO

- [x] Shell script to run all benchmarks
- [x] Fine-tune the parameters of egg and symREG
- [ ] Add more rewrite rules in egg
- [x] Add BDD to do equivalence checking
- [x] Using monotonic cost function in symREG
- [ ] Optimize parser 2, avoid duplicated parsing