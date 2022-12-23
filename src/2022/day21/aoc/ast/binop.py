from aoc.ast.number import Number


class BinOp:
    reversed_ops = {'+': '-', '-': '+', '*': '/', '/': '*', }

    def __init__(self, left, op, right):
        self.left = left
        self.op = op
        self.right = right

    def reversed_op(self):
        return self.reversed_ops[self.op]

    def eval(self, module=None):
        if module:
            self.left = module.vars[self.left.name]
            self.right = module.vars[self.right.name]

        return eval(f'{self.left.eval(module)} {self.op} {self.right.eval(module)}')

    def __repr__(self):
        if type(self.left) == BinOp or type(self.left) == Number:
            left = self.left.eval()
        else:
            left = f'{self.left.name}'

        if type(self.right) == BinOp or type(self.right) == Number:
            right = self.right.eval()
        else:
            right = f'{self.right.name}'

        return f'{left} {self.op} {right}'
