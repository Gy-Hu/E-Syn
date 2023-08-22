
from sympy import symbols, sympify, simplify, Symbol
from sympy.logic import simplify_logic
from sympy.logic.boolalg import And, Not, Or
from sympy import sqrt, simplify, count_ops, oo, S
import os
import to_sympy_parser, to_sympy_parser_sexpr
    
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
        elif isinstance(expr, Not):
            return f'(! {recurse(expr.args[0])})'
        else:
            return str(expr)

    expr_str = sympify(expr_str)
    #expr_str = simplify(expr_str, measure=my_measure)
    #expr_str = simplify_logic(expr_str, force=True)
    return recurse(expr_str)

def sympy_to_abc_eqn_normal_bool(expr): # sympy to abc eqn s-expression
    if isinstance(expr, And):
        return "(" + " * ".join(map(sympy_to_abc_eqn_normal_bool, expr.args)) + ")"
    elif isinstance(expr, Or):
        return "(" + " + ".join(map(sympy_to_abc_eqn_normal_bool, expr.args)) + ")"
    elif isinstance(expr, Not):
        return f"(!{sympy_to_abc_eqn_normal_bool(expr.args[0])})"
    else:  # Base case, assuming it's a symbol
        return str(expr)

# -------------------------------------------------------------------------------------------------

# load file to convert to s-expression (test)
with open ("test_data/out_test_abc_eqn.txt", "r") as myfile:
    # read line by line
    data=myfile.readlines()

eqn = data[3].split(" = ")[1].rstrip()
print("success load file")

# use `sympy_to_rust_sexpr()` to convert to s-expression
# parse the string to sympy
parser = to_sympy_parser.PropParser()
parser.build()
result = str(sympy_to_rust_sexpr(parser.parse(eqn)))
print("success convert to s-expression")
with open ("test_data/out_test_abc_eqn_sexpr.txt", "w") as myfile: 
    myfile.write(result)
    
'''
#############################################################################
#
#                     Using egg to optimize the circuit .... 
#
#############################################################################
'''    


# read the s-expression file and convert to aag
with open ("test_data/out_test_abc_eqn_sexpr.txt", "r") as myfile:
    # read line by line
    sexpr=myfile.readlines()

parser = to_sympy_parser_sexpr.PropParser()
parser.build()
result = str( sympy_to_abc_eqn_normal_bool(parser.parse(sexpr[0])) )

# write a new eqn file
with open ("test_data/out_test_abc_eqn_new.txt", "w") as myfile: 
    # write the first 3 lines of the original file - from data[0] to data[2]
    for i in range(3):
        myfile.write(data[i])
    # write the new eqn
    myfile.write(data[3].split(" = ")[0] + " = " + result + "\n")