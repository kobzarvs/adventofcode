from aoc.ast import Visitor, BinOp, Identifier


class CodeGenerator(Visitor):
    def __init__(self, start: str, end: str):
        super().__init__(start, end)
        self.code = []

    def generate_reverse_expr(self, node: Identifier):
        if type(node.expr) == BinOp:
            if node.expr.left in self.path:
                expr = node.what_is(node.expr.left.name)
                self.code.insert(0, str(expr))
                self.code.insert(0, f'{node.expr.right.name}: {node.expr.right.eval()}')
            if node.expr.right in self.path:
                expr = node.what_is(node.expr.right.name)
                self.code.insert(0, str(expr))
                self.code.insert(0, f'{node.expr.left.name}: {node.expr.left.eval()}')
