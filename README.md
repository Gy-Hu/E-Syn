# E-Brush

## Prerequisite

1. Enter `abc` directory and run `make`
2. Install necessary dependencies in `package.txt`
3. Build all the rust project ( `s-converter` , `analyzer` , `circuitparser` , `e-rewriter` , `infix2lisp` , `lisp2infix` ) by enter dir and run `cargo build --release`

## Usage

### For alpha version

1. Convert the circuit to eqn format by using `write_eqn` in `abc`
2. Copy and paste the eqn to `test_data/raw_circuit.txt` 
3. Run `python run.py`

### For beta version (much faster!)

1. Convert the circuit to eqn format by using `write_eqn` in `abc`
2. Copy and paste the eqn to `test_data_beta_runner/raw_circuit.txt`
3. Run `python run_beta.py`

## Benchamrk Description

1. ISCAS benchmark: https://github.com/santoshsmalagi/Benchmarks/tree/main
2. Ripple Carry Adder: Using ABC to generate the circuit ( `gen` command)
3. EPFL benchmark: https://github.com/lsils/benchmarks
4. IWLS 2005: http://iwls.org/iwls2005/benchmarks.html
5. Addtional benchmark1: https://pld.ttu.ee/~maksim/benchmarks/
6. Addtional benchmark2: https://github.com/jpsety/verilog_benchmark_circuits

## TODO

- [ ] Support constant value (0, 1) for parsing and rewriting
- [ ] Do code review for e-rewriter (any bugs? verify rules? make more efficient? support constant rewrite?)
- [ ] Add bidirectional rewrite rules
- [ ] Review rewrite rules/cost function/egraph selection/egraph merging (analyzer)
- [ ] Add strong regressor for egraph selection
- [ ] Add multiple-cost function
- [x] Support parallel running for multiple egraph rewriting output
- [x] If eqn fully unfold, then do not concat tmp variable ( `new_nxx_` )
- [x] Add more rewrite rules in egg
- [x] Shell script to run all benchmarks
- [x] Fine-tune the parameters of egg and symREG
- [x] Using 10000/lines as the auto-termination unfolding condition
- [x] Add cec check before and after eqn unfolding 
- [x] Add BDD to do equivalence checking
- [x] Using monotonic cost function in symREG
- [x] Optimize parser 2, avoid duplicated parsing
- [x] BUG: EPFL max, 10 times of unfolding, not pass cec in eqn check -> Fixed: eqn unfold too many times
- [x] BUG: C6288 unfold 1 time, will meet the bug to parse -> Fixed: eqn unfold too many times