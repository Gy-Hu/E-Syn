
from sympy import symbols, sympify, simplify, Symbol
from sympy.logic import simplify_logic
from sympy.logic.boolalg import And, Not, Or, Xor
from sympy import sqrt, simplify, count_ops, oo, S
import os
import to_sympy_parser, to_sympy_parser_sexpr
from collections import OrderedDict
    
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

def conver_to_sexpr(data, multiple_output = False):
   # global order
    if not multiple_output:
        eqn = data.split(" = ")[1].rstrip() #strip the `;` ?
    else:
        eqn = concatenate_equations(data) # concatenate the equations, strip the `;` ?
    print("success load file")

    # use `sympy_to_rust_sexpr()` to convert to s-expression
    # parse the string to sympy
    parser = to_sympy_parser.PropParser()
    parser.build()
    result = str(sympy_to_rust_sexpr(parser.parse(eqn)))
    print("success convert to s-expression")
    with open ("test_data/sexpr_for_egg.txt", "w") as myfile: 
        myfile.write(result)
        

        
def convert_to_abc_eqn(data, multiple_output = False):
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
        global order; order = components[0]
        components = components[1:]
        
        def pre(expr,lst):
            if isinstance(expr, Symbol):
                lst.append(str(expr).replace("po",""))
            for arg in expr.args:
                pre(arg,lst)
        
        symbol_order = []; pre(order, symbol_order)
        
        
        # Use OrderedDict to keep the order of components
        # components = OrderedDict((str(component), component) for component in components)
        result = [str(sympy_to_abc_eqn_normal_bool(component)) for component in components]
        
        print("multiple output circuit parse success")
        # write a new eqn file
        with open("test_data/optimized_circuit.txt", "w") as myfile:
            # write the first 3 lines of the original file - from data[0] to data[2]
            for i in range(3):
                myfile.write(data[i])
            # write the new eqn
            for i in range(len(result)):
                myfile.write(data[3+i].split(" = ")[0] + " = " + result[int(symbol_order[i])] + ";" + "\n")
        
        
        
def concatenate_equations(lines):
    equations = [f"({line.split('= ')[0]}) & ({line.split('= ')[1].rstrip().strip(';')})" for line in lines if line.startswith('po')]  # extract the equations
    #order = [line.split('= ')[0] for line in lines if line.startswith('po')]
    while len(equations) > 1:  # while there are more than one equation left
        equations[0] = f'({equations[0]}) & ({equations[1]})'  # concatenate the first two equations
        del equations[1]  # remove the second equation
    return equations[0]  # return the single remaining equation

# python main function
if __name__ == "__main__":
    # -------------------------------------------------------------------------------------------------
    multiple_output_flag = False
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
    if data[2].split(" = ")[1].rstrip() == "po0;":
        # one output circuit
        
        conver_to_sexpr(data[3]) # put the only one equation to the function
   
    else:
        # multiple output circuit
        print("multiple output circuit")
        multiple_output_flag = True
        
        # load all the content to `convert_to_sexpr()`
        # file to input string
        conver_to_sexpr(data, multiple_output = multiple_output_flag)    
        

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
    convert_to_abc_eqn(data, multiple_output= multiple_output_flag)
    
    '''
    #############################################################################
    #
    #                  Using abc to optimize/test the circuit ....
    #   
    #############################################################################   
    '''
    
    # for original circuit
    print("\n\n------------------------------------Original circuit------------------------------------")
    #command = "abc -c \"read_eqn test_data/original_circuit.txt; balance; refactor; print_stats; read_lib asap7_clean.lib ; map ; stime; strash ; andpos; write_aiger test_data/original_circuit.aig\""
    command = "abc -c \"read_eqn test_data/original_circuit.txt; balance; refactor; print_stats; read_lib asap7_clean.lib ; map ; stime; strash ; write_aiger test_data/original_circuit.aig\""
    os.system(command)
    print("----------------------------------------------------------------------------------------")
    
    # for optized circuit
    print("\n\n------------------------------------Optimized circuit------------------------------------")
    #command = "abc -c \"read_eqn test_data/optimized_circuit.txt; balance; refactor; print_stats; read_lib asap7_clean.lib ; map ; stime;  strash ; andpos; write_aiger test_data/optimized_circuit.aig\""
    command = "abc -c \"read_eqn test_data/optimized_circuit.txt; balance; refactor; print_stats; read_lib asap7_clean.lib ; map ; stime; strash ; write_aiger test_data/optimized_circuit.aig\""
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
    command = "abc -c \"cec test_data/original_circuit.aig test_data/optimized_circuit.aig\""
    os.system(command)