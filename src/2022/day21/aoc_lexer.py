from sly import Lexer


class AocLexer(Lexer):
    # Set of token names. This is always required
    tokens = {NAME, NUMBER, PLUS, MINUS, MUL, DIVIDE, ASSIGN}
    literals = {':', '+', '-', '*', '/'}

    # String containing ignored characters between tokens
    ignore = ' \t\n'
    ignore_comment_line = r'\#.*|\/\/.*'
    ignore_newline = r'\n+'

    # Regular expression rules for tokens
    PLUS = r'\+'
    MINUS = r'-'
    MUL = r'\*'
    DIVIDE = r'/'
    ASSIGN = r':'

    NAME = r'[a-zA-Z_][a-zA-Z0-9_]*'

    @_(r'\d+(\.\d+)?')
    def NUMBER(self, t):
        t.value = float(t.value)
        return t

    # Error handling rule
    def error(self, t):
        print("Illegal character '%s'" % t.value[0])
        self.index += 1
