from aoc.ast.BinOp import BinOp
from aoc.ast.Identifier import Identifier
from aoc.ast.Visitor import Visitor


class Program:
    def __init__(self, statements=None):
        if statements is None:
            statements = []
        self.statements = statements
        self.vars = {}

    def new_statement(self, name, expr):
        statement = self.get_or_create_var(name, expr)
        statement.module = self
        return statement

    def append(self, stmt):
        self.statements.append(self.new_statement(stmt.name, stmt.expr))

    def insert(self, stmt):
        self.statements.insert(0, self.new_statement(stmt.name, stmt.expr))

    def get_or_create_var(self, name, initial=None):
        if name in self.vars:
            if initial is not None:
                self.vars[name].expr = initial
        else:
            self.vars[name] = Identifier(name, initial)

        return self.vars[name]

    def eval(self):
        for statement in self.statements:
            s = self.get_or_create_var(statement.name, statement.expr)
            s.eval()
        return self.vars

    def traverse(self, node, visitor: Visitor):
        if isinstance(node, Identifier):
            if not visitor.enter_node(node):
                self.traverse(node.expr, visitor)
                if isinstance(node.expr, BinOp):
                    if not self.traverse(node.expr.left, visitor):
                        self.traverse(node.expr.right, visitor)
            return visitor.exit_node(node)
        return False

    def __repr__(self):
        return f'[{self.__class__.__name__}: {self.statements}]'
