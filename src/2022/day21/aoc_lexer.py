from sly import Lexer


class AocLexer(Lexer):
    # Set of token names. This is always required
    tokens = {NAME, NUMBER, PLUS, MINUS, MUL, DIVIDE, ASSIGN}

    # String containing ignored characters between tokens
    ignore = ' \t\n'
    ignore_comment_line = r'\#.*|\/\/.*'

    # Regular expression rules for tokens
    PLUS = r'\+'
    MINUS = r'-'
    MUL = r'\*'
    DIVIDE = r'/'
    ASSIGN = r':'

    literals = {':', '+', '-', '*', '/'}

    NAME = r'[a-zA-Z_][a-zA-Z0-9_]*'

    @_(r'\d+(\.\d+)?')
    def NUMBER(self, t):
        t.value = float(t.value)
        return t

    # Define a rule so we can track line numbers
    @_(r'\n+')
    def ignore_newline(self, t):
        self.lineno += len(t.value)

    # Error handling rule
    def error(self, t):
        print("Illegal character '%s'" % t.value[0])
        self.index += 1
