import ply.lex as lex

class PropLexer(object):
    tokens = ("AND", "OR", "NOT", "CONCAT", "SYMBOL", "LPAREN", "RPAREN")

    # Regular expression rules for simple tokens
    t_AND = r"\*"
    t_OR = r"\+"
    t_CONCAT = r"\&"
    t_NOT = r"!"
    t_LPAREN = r"\("
    t_RPAREN = r"\)"

    t_ignore = " \t"

    def t_SYMBOL(self, t):
        r"[a-zA-Z_?][a-zA-Z_0-9]*"
        t.value = str(t.value)  # Converting symbol to string
        return t

    def t_error(self, t):
        print("Illegal character '%s'" % t.value[0])
        t.lexer.skip(1)

    def build(self, **kwargs):
        self.lexer = lex.lex(module=self, **kwargs)

    def input(self, input):
        self.lexer.input(input)

    def lexer_test(self, data):
        self.lexer.input(data)
        while True:
            tok = self.lexer.token()
            if not tok:
                break
            print(tok)
