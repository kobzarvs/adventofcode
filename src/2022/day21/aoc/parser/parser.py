from sly import Parser

from aoc.ast import Program, BinOp, Number
from aoc.parser.lexer import AocLexer


class AocParser(Parser):
    debugfile = 'day21.out'
    tokens = AocLexer.tokens

    precedence = (
        ('left', PLUS, MINUS),
        ('left', MUL, DIVIDE),
    )

    def __init__(self):
        super().__init__()
        self.program = Program()

    @_('{ assignment }')
    def program(self, p):
        self.program.statements = p.assignment
        return self.program

    @_('NAME ASSIGN expr')
    def assignment(self, p):
        return self.program.get_or_create_var(p.NAME, p.expr)

    @_('expr PLUS expr',
       'expr MINUS expr',
       'expr MUL expr',
       'expr DIVIDE expr')
    def expr(self, p):
        return BinOp(p.expr0, p[1], p.expr1)

    @_('NAME')
    def expr(self, p):
        return self.program.get_or_create_var(p.NAME)

    @_('NUMBER')
    def expr(self, p):
        return Number(p.NUMBER)

    def error(self, p):
        if p:
            print("Syntax error at token", p.type)
            self.errok()
        else:
            print("Syntax error at EOF")
