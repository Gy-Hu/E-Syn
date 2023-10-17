import ply.yacc as yacc
from prop_lexer import PropLexer

class PropParser(object):
    tokens = PropLexer.tokens
    """
        id: symbol | ( prop )
        term : id
        prop: term
            | ( * prop prop )
            | ( + prop prop )
            | ( & prop prop )
            | ( ! prop )
    """

    # Parsing rules
    precedence = (
        ("left", "OR"),
        ("left", "AND"),
        ("left", "CONCAT"),
        ("right", "NOT"),
    )

    def __init__(self):
        self.lexer = PropLexer()
        self.lexer.build()
        self.atoms = {}
        self.concat_spliter = {}
        self.concat_spliter_id = 0

    # for parsing the proposition
    def p_prop_term(self, p):
        "prop : term"
        p[0] = p[1]

    def p_prop_and(self, p):
        "prop : LPAREN AND prop prop RPAREN"
        p[0] = f"({p[3]} * {p[4]})"

    def p_prop_or(self, p):
        "prop : LPAREN OR prop prop RPAREN"
        p[0] = f"({p[3]} + {p[4]})"

    def p_prop_concat(self, p):
        "prop : LPAREN CONCAT prop prop RPAREN"
        p[0] = f"({p[3]} & {p[4]})"
        # if self.concat_spliter_id is 0, add p[1] and p[3] to concat_spliter
        if self.concat_spliter_id == 0:
            self.concat_spliter[self.concat_spliter_id] = p[3]
            self.concat_spliter[self.concat_spliter_id + 1] = p[4]
            self.concat_spliter_id += 2
        else:
            self.concat_spliter[self.concat_spliter_id] = p[4]
            self.concat_spliter_id += 1


    def p_prop_not(self, p):
        "prop : LPAREN NOT prop RPAREN"
        p[0] = f"(!{p[3]})"

    def p_term_id(self, p):
        "term : id"
        p[0] = p[1]

    def p_id_symbol(self, p):
        "id : SYMBOL"
        p[0] = p[1]

    def p_error(self, p):
        print("Syntax error at '%s'" % p)

    def build(self, **kwargs):
        self.parser = yacc.yacc(module=self, **kwargs)

    def parse(self, data):
        self.lexer.input(data)
        return self.parser.parse(data, lexer=self.lexer.lexer), self.concat_spliter

# test string
# parser = PropParser()
# parser.build()
# result, _ = parser.parse("(& (* pi2 pi3) pi4)")
# print(result)
# print(type(list(_.values())[0]))