<h2 align="center">AIGBDD</br>Logical Equivalence Checker</h2>



<!-- TABLE OF CONTENTS -->
### Table of Contents

* [About the Project](#about-the-project)
* [Dependencies](#dependencies)
* [Installation](#installation)
* [Usage](#usage)
* [License](#license)
* [Contact](#contact)
* [Acknowledgments](#acknowledgments)



<!-- ABOUT THE PROJECT -->
### About The Project

This project consists of two programs, `aiglec` and `aigviz`. The first can fast check the logical equivalence between 2 small combinational AIGs using Reduced Ordered Binary Decision Diagrams (BDDs). The second generates graphical visualization for an AIG and its associated BDD. AIGs with latches (sequential circuits) are not supported.

The AIG files must be in the [AIGER](http://fmv.jku.at/aiger/) format (binary or ASCII).

<!-- GETTING STARTED -->
### Dependencies

[GraphViz](https://graphviz.org/about/) is needed to generate graphical visualization. You will also need `g++` and `make` to compile the source code. To install them in a computer with Ubuntu, open a terminal and type:
```
sudo apt-get install graphviz g++ make
```

### Installation

1. Clone the project in your computer:
```
git clone https://github.com/rafaelcalcada/aigbdd.git
```
2. Open the project directory and build `aiglec` and `aigviz` running `make`:
```
cd ./aigbdd
make
```
3. The programs are now ready to use.

<!-- USAGE EXAMPLES -->
### Usage

To check the logical equivalence between 2 distinct AIGs, type:
```
./aiglec [aig-file-1] [aig-file-2]
```
You can generate graphical visualization for both AIGs (and their associated BDDs) appending `-v` in the command line. For example:
```
./aiglec graph/test1.aag graph/test2.aag -v
```
If you're only interested in generating graphical visualization for an AIG, use `aigviz`:
```
./aigviz [aig-file]
```

<!-- LICENSE -->
### License

Distributed under the MIT License. See `LICENSE.md` for more information.

<!-- CONTACT -->
### Contact

Rafael Calçada - rafaelcalcada@gmail.com

Project Link: [https://github.com/rafaelcalcada/aigbdd](https://github.com/rafaelcalcada/aigbdd)

<!-- ACKNOWLEDGMENTS -->
### Acknowledgments

This work was done under the orientation of Professor [André Reis](http://www.inf.ufrgs.br/~andreis/) as the final work of his Logic Synthesis course. He gave valuable lessons about logic synthesis algorithms.

I also thank my colleague [Francisco Knebel](https://github.com/FranciscoKnebel) for the tips on how to make a decent README page.
