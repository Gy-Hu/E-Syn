# E-Brush

## Prerequisite

1. Enter `abc` directory and run `make`
2. Install necessary dependencies in `package.txt`

## Usage

1. Convert the circuit to eqn format by using `write_eqn` in `abc`
2. Copy and paste the eqn to `test/raw_circuit.txt`
2. Run `python run.py`

## Benchamrk Description

1. ISCAS benchmark: https://github.com/santoshsmalagi/Benchmarks/tree/main
2. Ripple Carry Adder: Using ABC to generate the circuit ( `gen` command)
3. EPFL benchmark: https://github.com/lsils/benchmarks
4. Addtional benchmark1: https://pld.ttu.ee/~maksim/benchmarks/
5. Addtional benchmark2: https://github.com/jpsety/verilog_benchmark_circuits

## TODO

- [ ] Shell script to run all benchmarks
- [ ] Fine-tune the parameters of egg and symREG
- [ ] Add more rewrite rules in egg
- [ ] Add BDD to do equivalence checking
- [ ] Using monotonic cost function in symREG