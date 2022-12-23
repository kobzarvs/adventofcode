from sly import Parser

from aoc_lexer import AocLexer
from aoc_ast import *


class AocParser(Parser):
    debugfile = 'day21.out'
    tokens = AocLexer.tokens

    precedence = (
        ('left', PLUS, MINUS),
        ('left', MUL, DIVIDE),
    )

    def __init__(self):
        super().__init__()
        self.vars = {}

    def get_or_create_var(self, name, initial=None):
        if name in self.vars:
            if initial is not None:
                self.vars[name].expr = initial
            return self.vars[name]
        else:
            self.vars[name] = Identifier(name, initial)
            return self.vars[name]

    @_('{ assignment }')
    def program(self, p):
        return Program(p.assignment)

    @_('NAME ASSIGN expr')
    def assignment(self, p):
        return self.get_or_create_var(p.NAME, p.expr)

    @_('expr PLUS expr',
       'expr MINUS expr',
       'expr MUL expr',
       'expr DIVIDE expr')
    def expr(self, p):
        return BinOp(p.expr0, p[1], p.expr1)

    @_('NAME')
    def expr(self, p):
        return self.get_or_create_var(p.NAME)

    @_('NUMBER')
    def expr(self, p):
        return Number(p.NUMBER)

    def error(self, p):
        if p:
            print("Syntax error at token", p.type)
            self.errok()
        else:
            print("Syntax error at EOF")
