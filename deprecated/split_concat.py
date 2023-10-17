import ply.yacc as yacc
from prop_lexer import PropLexer

class PropParser(object):
    tokens = PropLexer.tokens

    def __init__(self):
        self.lexer = PropLexer()
        self.lexer.build()
        self.atoms = {}

    def p_prop_term(self, p):
        "prop : term"
        p[0] = p[1]

    def p_prop_and(self, p):
        "prop : prop AND prop"
        left = p[1] if p[1].startswith('(') else f'({p[1]})'
        right = p[3] if p[3].startswith('(') else f'({p[3]})'
        p[0] = f'{left} * {right}'

    def p_prop_or(self, p):
        "prop : prop OR prop"
        left = p[1] if p[1].startswith('(') else f'({p[1]})'
        right = p[3] if p[3].startswith('(') else f'({p[3]})'
        p[0] = f'{left} + {right}'

    def p_prop_concat(self, p):
        "prop : prop CONCAT prop"
        left = p[1] if p[1].startswith('(') else f'({p[1]})'
        right = p[3] if p[3].startswith('(') else f'({p[3]})'
        p[0] = f'{left} & {right}'

    def p_prop_not(self, p):
        "prop : NOT prop"
        operand = p[2] if p[2].startswith('(') else f'({p[2]})'
        p[0] = f'!{operand}'

    def p_term_id(self, p):
        "term : id"
        p[0] = p[1]

    def p_id_symbol(self, p):
        "id : SYMBOL"
        if p[1] not in self.atoms:
            self.atoms[p[1]] = f'{p[1]}'
        p[0] = self.atoms[p[1]]

    def p_id_paren(self, p):
        "prop : LPAREN prop RPAREN"
        p[0] = f'({p[2]})'

    def p_error(self, p):
        print("Syntax error at '%s'" % p)

    def build(self, **kwargs):
        self.parser = yacc.yacc(module=self, **kwargs)

    def parse(self, data):
        self.lexer.input(data)
        parsed_data = self.parser.parse(data, lexer=self.lexer.lexer)
        tokens = parsed_data.split(' & ')
        result = '\n'.join([f'po{i} = {tokens[i]};' for i in range(len(tokens))])
        return result

# test string
parser = PropParser()
parser.build()
result = parser.parse("!(pi2 * pi3) + (pi1 * pi2) & !(pi0 * pi1) + (pi2 * pi3) & !(pi0 * pi1) * !(pi2 * pi3) + (pi0 * pi1) * (pi2 * pi3)")
print(result)