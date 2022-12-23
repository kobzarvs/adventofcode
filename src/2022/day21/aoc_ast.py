class Program:
    def __init__(self, statements):
        self.statements = statements

    def eval(self):
        for statement in self.statements:
            statement.eval()

    def __repr__(self):
        return f'[{self.__class__.__name__}: {self.statements}]'


class BinOp:
    reversed_ops = {'+': '-', '-': '+', '*': '/', '/': '*', }

    def __init__(self, left, op, right):
        self.left = left
        self.op = op
        self.right = right

    def reversed_op(self):
        return self.reversed_ops[self.op]

    def has(self, name):
        return self.left.has(name) or self.right.has(name)

    def eval(self):
        match self.op:
            case '+':
                return self.left.eval() + self.right.eval()
            case '-':
                return self.left.eval() - self.right.eval()
            case '*':
                return self.left.eval() * self.right.eval()
            case '/':
                return self.left.eval() / self.right.eval()

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


class Identifier:
    def __init__(self, name, expr):
        self.name = name
        self._expr = expr
        self.hook = None

    def __getattr__(self, item):
        if item == 'expr':
            return self.hook or self._expr

    def eval(self):
        if self.hook:
            return self.hook.eval()
        return self.expr.eval()

    def has(self, name):
        return self.name == name

    def what_is(self, name):
        if type(self.expr) != BinOp:
            return None
        names = [self.name, self.expr.left.name, self.expr.right.name]

        if name not in names:
            return None
        if name == self.name:
            return self

        if self.expr.op in ['+', '*']:
            if name == self.expr.left.name:
                return Identifier(name, BinOp(self, self.expr.reversed_op(), self.expr.right))
            elif name == self.expr.right.name:
                return Identifier(name, BinOp(self, self.expr.reversed_op(), self.expr.left))
        elif self.expr.op in ['-', '/']:
            if name == self.expr.left.name:
                return Identifier(name, BinOp(self, self.expr.reversed_op(), self.expr.right))
            elif name == self.expr.right.name:
                return Identifier(name, BinOp(self.expr.left, self.expr.op, self))

    def __repr__(self):
        return f'{self.name}: {self.expr}'


class Number:
    def __init__(self, value):
        self.value = value

    def eval(self):
        return self.value

    def has(self, name):
        return False

    def __repr__(self):
        return f'Number({self.value})'


class Visitor:
    def __init__(self, start, end):
        self.done = False
        self.start = start
        self.end = end
        self.path = []
        self.code = []

    def generate_reverse_expr(self, node):
        if type(node.expr) == BinOp:
            if node.expr.left in self.path:
                self.code.append(f'{node.expr.right.name}: {node.expr.right.eval()}')
                self.code.append(f'{node.what_is(node.expr.left.name)}')
            if node.expr.right in self.path:
                self.code.append(f'{node.expr.left.name}: {node.expr.left.eval()}')
                self.code.append(f'{node.what_is(node.expr.right.name)}')
        return self.code

    def enter_node(self, node, level):
        if node.name == self.end:
            self.path.append(node)
            self.generate_reverse_expr(node)
            self.done = True
        return self.done

    def exit_node(self, node, level):
        if self.done and node.name != self.start and type(node.expr) == BinOp:
            self.path.insert(0, node)
            self.generate_reverse_expr(node)
        return self.done


def traverse(node, visitor: Visitor, level=0):
    if isinstance(node, Identifier):
        if not visitor.enter_node(node, level + 1):
            traverse(node.expr, visitor, level + 1)
            if isinstance(node.expr, BinOp):
                if not traverse(node.expr.left, visitor, level):
                    traverse(node.expr.right, visitor, level)

        return visitor.exit_node(node, level + 1)
    return False


if __name__ == '__main__':
    x = Identifier('x', Number(4))
    y = Identifier('y', Number(2))

    e = Identifier('z', BinOp(x, '+', y))
    print('\n[+]')
    print(e.what_is('z'))
    print(e.what_is('x'))
    print(e.what_is('y'))

    e = Identifier('z', BinOp(x, '*', y))
    print('\n[*]')
    print(e.what_is('z'))
    print(e.what_is('x'))
    print(e.what_is('y'))

    e = Identifier('z', BinOp(x, '-', y))
    print('\n[-]')
    print(e.what_is('z'))
    print(e.what_is('x'))
    print(e.what_is('y'))

    e = Identifier('z', BinOp(x, '/', y))
    print('\n[*]')
    print(e.what_is('z'))
    print(e.what_is('x'))
    print(e.what_is('y'))
