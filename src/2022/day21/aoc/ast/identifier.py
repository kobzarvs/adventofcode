from aoc.ast.binop import BinOp
from aoc.ast.number import Number


class Identifier:
    def __init__(self, name, expr):
        self.name = name
        self.expr = Number(expr) if type(expr) in [int, float] else expr
        self.module = None

    def eval(self, module=None):
        return self.expr.eval(module or self.module)

    def what_is(self, name):
        names = [self.name, self.expr.left.name, self.expr.right.name]

        if name not in names:
            return None

        if name == self.name:
            return self

        expr = self.expr
        left = expr.left
        right = expr.right
        op = expr.op
        reversed_op = expr.reversed_op()

        # if name == left.name
        new_expr = BinOp(self, reversed_op, right)

        if name == right.name:
            if op in ['+', '*']:
                new_expr = BinOp(self, reversed_op, left)
            else:  # if op in ['-', '/']:
                new_expr = BinOp(left, op, self)

        return Identifier(name, new_expr)

    def __repr__(self):
        return f'{self.name}: {self.expr}'


if __name__ == '__main__':
    x = Identifier('x', 4)
    y = Identifier('y', 2)
    z1 = Identifier('z1', BinOp(x, '+', y))
    z2 = Identifier('z2', BinOp(x, '*', y))
    z3 = Identifier('z3', BinOp(x, '-', y))
    z4 = Identifier('z4', BinOp(x, '/', y))

    print('\n[+]')
    print(z1.what_is('z1'))
    print(z1.what_is('x'))
    print(z1.what_is('y'))

    print('\n[*]')
    print(z2.what_is('z2'))
    print(z2.what_is('x'))
    print(z2.what_is('y'))

    print('\n[-]')
    print(z3.what_is('z3'))
    print(z3.what_is('x'))
    print(z3.what_is('y'))

    print('\n[/]')
    print(z4.what_is('z4'))
    print(z4.what_is('x'))
    print(z4.what_is('y'))
