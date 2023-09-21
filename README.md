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

- [ ] Using DC to read large eqn file at first
- [ ] Do code review for e-rewriter (any bugs? verify rules? make more efficient? support constant rewrite?)
- [ ] Review rewrite rules/cost function/egraph selection/egraph merging (analyzer)
- [ ] Concurrent parsing egg output and using metric (0.6 * delay + 0.4 * area) to select the best optimization result
- [ ] More features for regression
  - [ ] Leaf node and its responding boolean operation
- [ ] More customized for unfolding formula in parsing circuit (smart unfolding strategy, heuristic unfolding strategy, naive unfolding strategy, etc)
  - [ ] Unfold the longest critical path at first
  - [ ] Unfold according to bfs/dfs depth ordering
  - [ ] Unfold according to the number of lines
  - [ ] Unfold two times/one time (naive unfolding strategy)
- [ ] Modify Analyzer in egg
- [x] Enhancing ABC to support reading large eqn file -> in dev branch
- [X] Support constant value (0, 1) for parsing and rewriting
- [X] Add bidirectional rewrite rules + constant rewrite rules
- [X] Add strong regressor for egraph selection (using linfa?)
- [X] Richer feature for regression (propagation depth, number of nodes, number of edges, number of gates, number of inputs, number of outputs, number of operations, number of constants, etc)
  - [X] Sum of liberty * node number
  - [X] Average liberty * node number
  - [X] Total number of nodes
- [X] Add multiple-cost function (Astsize, Astdepth, cell library info, etc)
- [X] Add more variants for the e-graph generation -> enlarge the fluctuation range (e.g. using different cost function, different e-graph merging strategy, etc) -> check how the extractor extract same least cost result?
- [X] Add experiment to check result without abc optimization
- [X] Regressor in large circuit (mixed large and small circuit) -> modify data collection script
- [X] Add node cost to cost function
- [X] Fix bugs in SymREG - some circuit missed up
- [X] Support parallel running for multiple egraph rewriting output
- [X] If eqn fully unfold, then do not concat tmp variable ( `new_nxx_` )
- [X] Add more rewrite rules in egg
- [X] Shell script to run all benchmarks
- [X] Fine-tune the parameters of egg and symREG
- [X] Using 10000/lines as the auto-termination unfolding condition
- [X] Add cec check before and after eqn unfolding
- [X] Add BDD to do equivalence checking
- [X] Using monotonic cost function in symREG
- [X] Optimize parser 2, avoid duplicated parsing
- [X] BUG: EPFL max, 10 times of unfolding, not pass cec in eqn check -> Fixed: eqn unfold too many times
- [X] BUG: C6288 unfold 1 time, will meet the bug to parse -> Fixed: eqn unfold too many times
