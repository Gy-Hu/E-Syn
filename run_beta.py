
from sympy import symbols, sympify, simplify, Symbol, Eq, simplify_logic
from sympy.logic.boolalg import Equivalent
from sympy.logic.inference import satisfiable
from sympy.logic import simplify_logic
from sympy.logic.boolalg import And, Not, Or, Xor
from sympy import sqrt, simplify, count_ops, oo, S
import os
import to_sympy_parser, to_sympy_parser_sexpr, lisp2infix
from collections import OrderedDict
from sympy.parsing.sympy_parser import parse_expr
from tqdm import tqdm
import copy
import CircuitParser
import sys   
sys.setrecursionlimit(100000)

def check_equal(FORMULA_LIST, components):
    result = []
    pbar = tqdm(total=len(FORMULA_LIST)*len(components), desc='Processing Formulas')
    for i in range(len(FORMULA_LIST)):
        for j in range(len(components)):
            if not satisfiable(Not(Equivalent(FORMULA_LIST[i], components[j]))):
                result.append((i,j))
            pbar.update(1)
    pbar.close()
    return result

def my_measure(expr):
    OR = Symbol('OR') # 9 - !, 22 - AND, 26 - OR
    AND = Symbol('AND')
    # Discourage powers by giving POW a weight of 10
    count = count_ops(expr, visual=True).subs(OR, 26)
    count = count_ops(expr, visual=True).subs(AND, 22)
    # Every other operation gets a weight of 1 (the default)
    count = count.replace(Symbol, type(S.One))
    return count

def sympy_to_rust_sexpr(expr_str): # sympy to rust s-expression
    def recurse(expr):
        if isinstance(expr, And):
            if len(expr.args) > 2:
                return f'(* {recurse(And(*expr.args[:-1]))} {recurse(expr.args[-1])})'
            else:
                return '(* ' + ' '.join(map(recurse, expr.args)) + ')'
        elif isinstance(expr, Or):
            if len(expr.args) > 2:
                return f'(+ {recurse(Or(*expr.args[:-1]))} {recurse(expr.args[-1])})'
            else:
                return '(+ ' + ' '.join(map(recurse, expr.args)) + ')'
        elif isinstance(expr, Xor):
            if len(expr.args) > 2:
                return f'(& {recurse(Xor(*expr.args[:-1]))} {recurse(expr.args[-1])})'
            else:
                return '(& ' + ' '.join(map(recurse, expr.args)) + ')'
        elif isinstance(expr, Not):
            return f'(! {recurse(expr.args[0])})'
        else:
            return str(expr)

    expr_str = sympify(expr_str)
    #expr_str = simplify(expr_str, measure=my_measure)
    #expr_str = simplify_logic(expr_str, force=True)
    return recurse(expr_str)

def sympy_to_abc_eqn_normal_bool(expr): # sympy to abc eqn s-expression
    # Enter : 
    # ((pi0 & pi1) | ~(pi0 & pi1)) ^ ((pi0 & pi1 & pi2 & pi3) | (~(pi0 & pi1) & ~(pi2 & pi3))) ^ (((pi0 & pi1) | (pi2 & pi3)) & (~(pi0 & pi1) | ~(pi2 & pi3)))
    if isinstance(expr, And):
        return "(" + " * ".join(map(sympy_to_abc_eqn_normal_bool, expr.args)) + ")"
    elif isinstance(expr, Or):
        return "(" + " + ".join(map(sympy_to_abc_eqn_normal_bool, expr.args)) + ")"
    elif isinstance(expr, Xor):
        return "(" + " & ".join(map(sympy_to_abc_eqn_normal_bool, expr.args)) + ")"
    elif isinstance(expr, Not):
        return f"(!{sympy_to_abc_eqn_normal_bool(expr.args[0])})"
    else:  # Base case, assuming it's a symbol
        return str(expr)
    # Return :
    # (((pi0 * pi1) + (!(pi0 * pi1))) & ((pi0 * pi1 * pi2 * pi3) + ((!(pi0 * pi1)) * (!(pi2 * pi3)))) & (((pi0 * pi1) + (pi2 * pi3)) * ((!(pi0 * pi1)) + (!(pi2 * pi3)))))

def conver_to_sexpr(data, multiple_output = False, output_file_path = "test_data_beta_runner/sexpr_for_egg.txt"):
   # global order
    if not multiple_output:
        eqn = data.split(" = ")[1].rstrip().strip(";") #strip the `;` ?
    else:
        eqn, FORMULA_LIST = concatenate_equations(data) # concatenate the equations, strip the `;` ?
    print("success load file")
    

    # use `sympy_to_rust_sexpr()` to convert to s-expression
    # parse the string to sympy
    
    # parser = to_sympy_parser.PropParser()
    # parser.build()
    # result = str(sympy_to_rust_sexpr(parser.parse(eqn)))
    
   
    
    # print("success convert to s-expression")
    # with open (output_file_path, "w") as myfile: 
    #     myfile.write(result)
        
    # if multiple_output: 
    #     FORMULA_LIST = [parser.parse(eqn) for eqn in FORMULA_LIST]
    #     return FORMULA_LIST
    
    # use s-converter to convert to s-expression
    # dump eqn to test_data_beta_runner/input_for_s-converter.txt
    with open ("test_data_beta_runner/input_for_s-converter.txt", "w") as myfile:
        myfile.write(eqn)
    
    os.system("s-converter/target/release/s-converter test_data_beta_runner/input_for_s-converter.txt test_data_beta_runner/sexpr_for_egg.txt lisp")
    if multiple_output: 
        return None

        
def convert_to_abc_eqn(data, FORMULA_LIST=None, multiple_output = False):
    # using the s-converter to convert to abc eqn
    #os.system("s-converter/target/release/s-converter test_data_beta_runner/output_from_egg.txt test_data_beta_runner/output_from_s-converter.txt test_data_beta_runner/split_concat.txt")
    
    if not multiple_output:
        parser = to_sympy_parser_sexpr.PropParser(); parser.build()
        # read the s-expression file and convert to aag
        with open ("test_data_beta_runner/output_from_egg.txt", "r") as myfile:
            # read line by line
            sexpr=myfile.readlines()
        
        parse_res, _ = parser.parse(sexpr[0])
        result = str( sympy_to_abc_eqn_normal_bool(parse_res) )
        # write a new eqn file
        with open ("test_data_beta_runner/optimized_circuit.eqn", "w") as myfile: 
            # write the first 3 lines of the original file - from data[0] to data[2]
            for i in range(3):
                myfile.write(data[i])
            # write the new eqn
            myfile.write(data[3].split(" = ")[0] + " = " + result + "\n")
    else:
        # parser = to_sympy_parser.PropParser(); parser.build()
        # with open ("test_data_beta_runner/output_from_s-converter.txt", "r") as myfile:
        #     sexpr=myfile.readlines()
        
        parser = to_sympy_parser_sexpr.PropParser(); parser.build()
        #parser = lisp2infix.PropParser(); parser.build()
        with open ("test_data_beta_runner/output_from_egg.txt", "r") as myfile:
            sexpr=myfile.readlines()
        
        # read s-converter/split_concat.txt
        # with open ("test_data_beta_runner/output_from_s-converter.txt", "r") as myfile:
        #     # read line by line
        #     lines=myfile.readlines()
        
        # with open ("test_data_beta_runner/split_concat.txt", "r") as myfile:
        #     lines=myfile.readlines()
        
        # Use the function
        #equ_check_result = check_equal(FORMULA_LIST, components); print(len(equ_check_result)); print(equ_check_result)
        
        parser_res, _ = parser.parse(sexpr[0])
        
        with open("test_data_beta_runner/tmp.txt", "w") as myfile:
            for id in _:
                myfile.write(str(_[id])+'\n------------------------\n')
                
        #components =  list(parser_res.args)
        
        # convert dict _ to list
        components = list(_.values())
        # for every component, simplify it
        components = [simplify(component, measure=my_measure) for component in components]
        # for every result , replace the symbol `|`  to `+` , `~` to `!` , `&` to `*`
        result = [(str(component)).replace("|", "+").replace("~", "!").replace("&", "*") for component in components]
        
        #result = components
        
        print("multiple output circuit parse success")
        # write a new eqn file
        with open("test_data_beta_runner/optimized_circuit.eqn", "w") as myfile:
            # write the first 3 lines of the original file - from data[0] to data[2]
            for i in range(3):
                myfile.write(data[i])
            # write the new eqn
            for i in range(len(result)):
                myfile.write(data[3+i].split(" = ")[0] + " = " + result[i] + ";" + "\n")
                
                #myfile.write(data[3+i].split(" = ")[0] + " = " + result[int(symbol_order[i])] + ";" + "\n")
                #myfile.write(data[3+i].split(" = ")[0] + " = " + result[i] + ";" + "\n")
                
                # split the lines in first space, for example, 1 xxx abc -> 1, xxx abc
                #myfile.write(data[3+i].split(" = ")[0] + " = " + (lines[i].split(' ',1)[1]).rstrip().strip(";") + ";" + "\n")

        
        
def concatenate_equations(lines):
    #equations = [f"({line.split('= ')[0]}) & ({line.split('= ')[1].rstrip().strip(';')})" for line in lines if line.startswith('po')]  # extract the equations
    
    #order = [line.split('= ')[0] for line in lines if line.startswith('po')]
    
    FORMULA_LIST = [line.split('= ')[1].rstrip().strip(';') for line in lines[3:]]
    # copy the FORMULA_LIST to equations
    equations = copy.deepcopy(FORMULA_LIST)
    
    num_concat = 0
    
    while len(equations) > 1:  # while there are more than one equation left
        equations[0] = f'({equations[0]} & {equations[1]})'  # concatenate the first two equations
        num_concat += 1
        del equations[1]  # remove the second equation
    return equations[0], FORMULA_LIST  # return the single remaining equation

# python main function
if __name__ == "__main__":
    global FORMULA_LIST
    # -------------------------------------------------------------------------------------------------
    multiple_output_flag = False
    
    # process the raw circuit file
    input_file_path = "test_data_beta_runner/raw_circuit.eqn"
    output_file_path = "test_data_beta_runner/original_circuit.eqn"
    
    os.system("alpha_utils/circuitparser/target/release/circuitparser test_data_beta_runner/raw_circuit.eqn test_data_beta_runner/original_circuit.eqn test_data_beta_runner/input_for_s-converter.txt 2")

    #os.system("./circuitparser.out test_data_beta_runner/raw_circuit.eqn test_data_beta_runner/original_circuit.eqn")

    # parser =  CircuitParser.CircuitParser(input_file_path, output_file_path)
    # parser.process()
    
    # load file to convert to s-expression (test)
    with open ("test_data_beta_runner/original_circuit.eqn", "r") as myfile:
        # read line by line
        data=myfile.readlines()
        
    '''
    #############################################################################
    #
    #                    Pre-processing the circuit for egg ....
    #
    #############################################################################
    '''
        
    # if data[2] is 'OUTORDER = po0;\n':
    if len(data[2].split(" = ")[1].rstrip().strip(";").split()) == 1:
        # one output circuit
        
        conver_to_sexpr(data[3]) # put the only one equation to the function
        FORMULA_LIST = None
   
    else:
        # multiple output circuit
        print("multiple output circuit")
        multiple_output_flag = True
        
        # load all the content to `convert_to_sexpr()`
        # file to input string
        #FORMULA_LIST = conver_to_sexpr(data, multiple_output = multiple_output_flag)  
        os.system("alpha_utils/infix2lisp/target/release/s-converter test_data_beta_runner/input_for_s-converter.txt test_data_beta_runner/sexpr_for_egg.txt")
          
        

    '''
    #############################################################################
    #
    #                 Using egg to optimize the circuit ....
    #
    #############################################################################
    '''
    # run egg 
    os.system("e-rewriter/target/release/e-rewriter test_data_beta_runner/sexpr_for_egg.txt test_data_beta_runner/output_from_egg.txt test_data_beta_runner")
    
    '''
    #############################################################################
    #
    #                  Post-processing the circuit for abc .... 
    #
    #############################################################################
    '''
    convert_to_abc_eqn(data, None, multiple_output= multiple_output_flag)
    
    '''
    #############################################################################
    #
    #                  Using abc to optimize/test the circuit ....
    #   
    #############################################################################   
    '''
    
    # for original circuit
    print("\n\n------------------------------------Original circuit------------------------------------")
    #command = "./abc/abc -c \"read_eqn test_data_beta_runner/original_circuit.eqn; balance; refactor; print_stats -p; read_lib asap7_clean.lib ; map ; stime; strash ; andpos; write_aiger test_data_beta_runner/original_circuit.aig\""
    #command = "./abc/abc -c \"read_eqn test_data_beta_runner/original_circuit.eqn; balance; refactor; print_stats; read_lib asap7_clean.lib ; map ; stime; strash ; write_aiger test_data_beta_runner/original_circuit.aig\""
    #command = "./abc/abc -c \"read_eqn test_data_beta_runner/original_circuit.eqn;balance; refactor; balance; rewrite; rewrite -z; balance; rewrite -z; balance; print_stats -p; read_lib asap7_clean.lib ; map ; stime; collapse; write_blif test_data_beta_runner/original_circuit.blif\""
    
    ################## gate sizing ###################
    #command = "./abc/abc -c \"read_eqn test_data_beta_runner/raw_circuit.eqn; st; print_stats -p; read_lib asap7_clean.lib ; map ; topo; upsize; dnsize; stime\""
    
    ################## st + dch ######################
    command = "./abc/abc -c \"read_eqn test_data_beta_runner/raw_circuit.eqn; st; dch -f; print_stats -p; read_lib asap7_clean.lib ; map ; topo; upsize; dnsize; stime\""
    os.system(command)
    print("----------------------------------------------------------------------------------------")
    
    # for optized circuit
    print("\n\n------------------------------------Optimized circuit------------------------------------")
    #command = "./abc/abc -c \"read_eqn test_data_beta_runner/optimized_circuit.eqn; balance; refactor; print_stats -p; read_lib asap7_clean.lib ; map ; stime;  strash ; andpos; write_aiger test_data_beta_runner/optimized_circuit.aig\""
    #command = "./abc/abc -c \"read_eqn test_data_beta_runner/optimized_circuit.eqn; balance; refactor; print_stats; read_lib asap7_clean.lib ; map ; stime; strash ; write_aiger test_data_beta_runner/optimized_circuit.aig\""
    #command = "./abc/abc -c \"read_eqn test_data_beta_runner/optimized_circuit.eqn; balance; refactor; print_stats -p; read_lib asap7_clean.lib ; map ; stime; collapse; write_blif test_data_beta_runner/optimized_circuit.blif\""
    
   
    ################## gate sizing ###################
    #command = "./abc/abc -c \"read_eqn test_data_beta_runner/optimized_circuit.eqn; st; print_stats -p; read_lib asap7_clean.lib ; map ; topo; upsize; dnsize; stime\""
    
    ################## st + dch ######################
    command = "./abc/abc -c \"read_eqn test_data_beta_runner/optimized_circuit.eqn; st; dch -f; print_stats -p; read_lib asap7_clean.lib ; map ; topo; upsize; dnsize; stime\""
    os.system(command)
    print("----------------------------------------------------------------------------------------")
    
    '''
    #############################################################################
    #
    #               Equivalence checking between original and optimized circuit
    #
    #############################################################################
    '''
    # for original circuit
    # print("\n\n------------------------------------Equivalence checking------------------------------------")
    # os.system("./abc/abc -c \"cec test_data_beta_runner/original_circuit_and_all.aig test_data_beta_runner/optimized_circuit_and_all.aig\"")
    # print("-----------------------------------------Finish Equivalence checking-----------------------------------------")
    

    '''
    #############################################################################
    #
    #               Additional quivalence checking between original and optimized circuit
    #
    #############################################################################
    '''
    # additional test
    # os.system("./abc/abc -c \"read_eqn test_data_beta_runner/original_circuit.eqn; balance; refactor;  read_lib asap7_clean.lib ; map ; strash ; orpos; write_aiger test_data_beta_runner/original_circuit_or_all.aig\"")
    
    # os.system("./abc/abc -c \"read_eqn test_data_beta_runner/optimized_circuit.eqn; balance; refactor; read_lib asap7_clean.lib ; map ;  strash ; orpos; write_aiger test_data_beta_runner/optimized_circuit_or_all.aig\"")
    
    # print("\n\n------------------------------------Additional Equivalence checking------------------------------------")
    # os.system("./abc/abc -c \"cec test_data_beta_runner/original_circuit_or_all.aig test_data_beta_runner/optimized_circuit_or_all.aig\"")
    # print("-----------------------------------------Finish Equivalence checking-----------------------------------------")
    
    '''
    #############################################################################
    #
    #               Using cec to check the equivalence between original and optimized circuit
    #
    #############################################################################
    '''
    os.system("./abc/abc -c \"cec test_data_beta_runner/raw_circuit.eqn test_data_beta_runner/optimized_circuit.eqn\"")
    # os.system("./abc/abc -c \"read_eqn test_data_beta_runner/raw_circuit.eqn; strash; write_aiger test_data_beta_runner/raw_circuit.aig\"")
    # os.system("./abc/abc -c \"read_eqn test_data_beta_runner/optimized_circuit.eqn; strash; write_aiger test_data_beta_runner/optimized_circuit.aig\"")
    # os.system("./abc/abc -c \"read_aiger test_data_beta_runner/raw_circuit.aig; collapse; write_blif test_data_beta_runner/raw_circuit.blif\"")
    # os.system("./abc/abc -c \"read_aiger test_data_beta_runner/optimized_circuit.aig; collapse; write_blif test_data_beta_runner/optimized_circuit.blif\"")
    # os.system("./abc/abc -c \"cec test_data_beta_runner/raw_circuit.blif test_data_beta_runner/optimized_circuit.blif\"")
    # os.system("./aigbdd/aiglec test_data_beta_runner/raw_circuit.aig test_data_beta_runner/optimized_circuit.aig")