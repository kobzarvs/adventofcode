from aoc.ast import Visitor, Program, Identifier, BinOp


class AstGenerator(Visitor):
    def __init__(self, start: str, end: str):
        super().__init__(start, end)
        self.prog = Program()

    def generate_reverse_expr(self, node: Identifier):
        if type(node.expr) == BinOp:
            if node.expr.left in self.path:
                expr = node.what_is(node.expr.left.name)
                self.prog.insert(expr)
                self.prog.insert(Identifier(node.expr.right.name, node.expr.right.eval()))
            if node.expr.right in self.path:
                expr = node.what_is(node.expr.right.name)
                self.prog.insert(expr)
                self.prog.insert(Identifier(node.expr.left.name, node.expr.left.eval()))
