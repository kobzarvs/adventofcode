from typing import List

from aoc.ast.binop import BinOp
from aoc.ast.identifier import Identifier


class Visitor:
    def __init__(self, start: str, end: str):
        self.done = False
        self.start = start
        self.end = end
        self.path: List[Identifier] = []

    def generate_reverse_expr(self, node: Identifier):
        pass

    def enter_node(self, node: Identifier):
        if node.name == self.end:
            self.path.append(node)
            self.generate_reverse_expr(node)
            self.done = True
        return self.done

    def exit_node(self, node: Identifier):
        if self.done and node.name != self.start and type(node.expr) == BinOp:
            self.path.insert(0, node)
            self.generate_reverse_expr(node)
        return self.done
