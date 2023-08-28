
from sympy import symbols, sympify, simplify, Symbol, Eq, simplify_logic
from sympy.logic.boolalg import Equivalent
from sympy.logic.inference import satisfiable
from sympy.logic import simplify_logic
from sympy.logic.boolalg import And, Not, Or, Xor
from sympy import sqrt, simplify, count_ops, oo, S
import os
import to_sympy_parser, to_sympy_parser_sexpr
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

def conver_to_sexpr(data, multiple_output = False, output_file_path = "test_data/sexpr_for_egg.txt"):
   # global order
    if not multiple_output:
        eqn = data.split(" = ")[1].rstrip().strip(";") #strip the `;` ?
    else:
        eqn, FORMULA_LIST = concatenate_equations(data) # concatenate the equations, strip the `;` ?
    print("success load file")
    

    # use `sympy_to_rust_sexpr()` to convert to s-expression
    # parse the string to sympy
    
    parser = to_sympy_parser.PropParser()
    parser.build()
    result = str(sympy_to_rust_sexpr(parser.parse(eqn)))
    
    print("success convert to s-expression")
    with open (output_file_path, "w") as myfile: 
        myfile.write(result)
        
    if multiple_output: 
        FORMULA_LIST = [parser.parse(eqn) for eqn in FORMULA_LIST]
        return FORMULA_LIST
    
        

        
def convert_to_abc_eqn(data, FORMULA_LIST=None, multiple_output = False):
    # read the s-expression file and convert to aag
    with open ("test_data/output_from_egg.txt", "r") as myfile:
        # read line by line
        sexpr=myfile.readlines()

    parser = to_sympy_parser_sexpr.PropParser()
    parser.build()
    
    if not multiple_output:
        result = str( sympy_to_abc_eqn_normal_bool(parser.parse(sexpr[0])) )
        # write a new eqn file
        with open ("test_data/optimized_circuit.txt", "w") as myfile: 
            # write the first 3 lines of the original file - from data[0] to data[2]
            for i in range(3):
                myfile.write(data[i])
            # write the new eqn
            myfile.write(data[3].split(" = ")[0] + " = " + result + "\n")
    else:
        components =  list(parser.parse(sexpr[0]).args)
        
        '''
        global order
        # get the key component, if str(component) start with `~(po`, then it is the key component
        # find the key component
        for component in components:
            if str(component).startswith("~(po"):
                order = component
                break
        components.remove(order)
        
        
        def pre(expr,lst):
            if isinstance(expr, Symbol):
                if str(expr).startswith("po"): lst.append(str(expr).replace("po",""))
            for arg in expr.args:
                pre(arg,lst)
        
        symbol_order = []; pre(order, symbol_order)
        '''
        
        # Use OrderedDict to keep the order of components
        # components = OrderedDict((str(component), component) for component in components)
        result = [str(sympy_to_abc_eqn_normal_bool(component)) for component in components]
        
        # Use the function
        #equ_check_result = check_equal(FORMULA_LIST, components); print(len(equ_check_result)); print(equ_check_result)
        
        print("multiple output circuit parse success")
        # write a new eqn file
        with open("test_data/optimized_circuit.txt", "w") as myfile:
            # write the first 3 lines of the original file - from data[0] to data[2]
            for i in range(3):
                myfile.write(data[i])
            # write the new eqn
            for i in range(len(result)):
                #myfile.write(data[3+i].split(" = ")[0] + " = " + result[int(symbol_order[i])] + ";" + "\n")
                myfile.write(data[3+i].split(" = ")[0] + " = " + result[i] + ";" + "\n")
        
        
        
def concatenate_equations(lines):
    #equations = [f"({line.split('= ')[0]}) & ({line.split('= ')[1].rstrip().strip(';')})" for line in lines if line.startswith('po')]  # extract the equations
    
    #order = [line.split('= ')[0] for line in lines if line.startswith('po')]
    
    FORMULA_LIST = [line.split('= ')[1].rstrip().strip(';') for line in lines[3:]]
    # copy the FORMULA_LIST to equations
    equations = copy.deepcopy(FORMULA_LIST)
    
    while len(equations) > 1:  # while there are more than one equation left
        equations[0] = f'({equations[0]}) & ({equations[1]})'  # concatenate the first two equations
        del equations[1]  # remove the second equation
    return equations[0], FORMULA_LIST  # return the single remaining equation

# python main function
if __name__ == "__main__":
    global FORMULA_LIST
    # -------------------------------------------------------------------------------------------------
    multiple_output_flag = False
    
    # process the raw circuit file
    input_file_path = "test_data/raw_circuit.txt"
    output_file_path = "test_data/original_circuit.txt"

    parser =  CircuitParser.CircuitParser(input_file_path, output_file_path)
    parser.process()
    
    # load file to convert to s-expression (test)
    with open ("test_data/original_circuit.txt", "r") as myfile:
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
        FORMULA_LIST = conver_to_sexpr(data, multiple_output = multiple_output_flag)    
        

    '''
    #############################################################################
    #
    #                 Using egg to optimize the circuit ....
    #
    #############################################################################
    '''
    # run egg
    command = "e-rewriter/target/debug/e-rewriter test_data/sexpr_for_egg.txt test_data/output_from_egg.txt"
    os.system(command)
    
    '''
    #############################################################################
    #
    #                  Post-processing the circuit for abc .... 
    #
    #############################################################################
    '''
    convert_to_abc_eqn(data, FORMULA_LIST, multiple_output= multiple_output_flag)
    
    '''
    #############################################################################
    #
    #                  Using abc to optimize/test the circuit ....
    #   
    #############################################################################   
    '''
    
    # for original circuit
    print("\n\n------------------------------------Original circuit------------------------------------")
    command = "./abc/abc -c \"read_eqn test_data/original_circuit.txt; balance; refactor; print_stats -p; read_lib asap7_clean.lib ; map ; stime; strash ; andpos; write_aiger test_data/original_circuit.aig\""
    #command = "./abc/abc -c \"read_eqn test_data/original_circuit.txt; balance; refactor; print_stats; read_lib asap7_clean.lib ; map ; stime; strash ; write_aiger test_data/original_circuit.aig\""
    os.system(command)
    print("----------------------------------------------------------------------------------------")
    
    # for optized circuit
    print("\n\n------------------------------------Optimized circuit------------------------------------")
    command = "./abc/abc -c \"read_eqn test_data/optimized_circuit.txt; balance; refactor; print_stats -p; read_lib asap7_clean.lib ; map ; stime;  strash ; andpos; write_aiger test_data/optimized_circuit.aig\""
    #command = "./abc/abc -c \"read_eqn test_data/optimized_circuit.txt; balance; refactor; print_stats; read_lib asap7_clean.lib ; map ; stime; strash ; write_aiger test_data/optimized_circuit.aig\""
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
    print("\n\n------------------------------------Equivalence checking------------------------------------")
    verify_command = "./abc/abc -c \"cec test_data/original_circuit.aig test_data/optimized_circuit.aig\""
    os.system(verify_command)
    print("-----------------------------------------Finish Equivalence checking-----------------------------------------")
    
    '''
    #############################################################################
    #
    #               Additional quivalence checking between original and optimized circuit
    #
    #############################################################################
    '''
    # additional test
    command = "./abc/abc -c \"read_eqn test_data/original_circuit.txt; balance; refactor;  read_lib asap7_clean.lib ; map ; strash ; orpos; write_aiger test_data/original_circuit.aig\""
    os.system(command)
    
    command = "./abc/abc -c \"read_eqn test_data/optimized_circuit.txt; balance; refactor; read_lib asap7_clean.lib ; map ;  strash ; orpos; write_aiger test_data/optimized_circuit.aig\""
    os.system(command)
    
    print("\n\n------------------------------------Additional Equivalence checking------------------------------------")
    os.system(verify_command)
    print("-----------------------------------------Finish Equivalence checking-----------------------------------------")