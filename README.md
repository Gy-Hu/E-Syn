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
5. Comperhensive digital benchmark: https://ddd.fit.cvut.cz/www/prj/Benchmarks/index.php?page=download
6. Addtional benchmark1: https://pld.ttu.ee/~maksim/benchmarks/
7. Addtional benchmark2: https://github.com/jpsety/verilog_benchmark_circuits

## TODO

- [x] Support constant value (0, 1) for parsing and rewriting
- [ ] Do code review for e-rewriter (any bugs? verify rules? make more efficient? support constant rewrite?)
- [x] Add bidirectional rewrite rules + constant rewrite rules
- [ ] Review rewrite rules/cost function/egraph selection/egraph merging (analyzer)
- [x] Add strong regressor for egraph selection (using linfa?)
- [x] Richer feature for regression (propagation depth, number of nodes, number of edges, number of gates, number of inputs, number of outputs, number of operations, number of constants, etc)
    - [x] Sum of liberty * node number
    - [x] Average liberty * node number
    - [x] Total number of nodes
- [ ] Add multiple-cost function (Astsize, Astdepth, cell library info, etc)
- [ ] Concurrent parsing egg output and using metric (0.6 * delay + 0.4 * area) to select the best optimization result
- [x] Add more variants for the e-graph generation -> enlarge the fluctuation range (e.g. using different cost function, different e-graph merging strategy, etc) -> check how the extractor extract same least cost result?
- [x] Add experiment to check result without abc optimization
- [ ] Regressor in large circuit (mixed large and small circuit) -> modify data collection script
- [x] Add node cost to cost function
- [ ] More customized for unfolding formula in parsing circuit
- [ ] Modify Analyzer in egg
- [x] Fix bugs in SymREG - some circuit missed up
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