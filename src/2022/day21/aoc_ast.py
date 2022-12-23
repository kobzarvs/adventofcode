class Program:
    def __init__(self, statements=[]):
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

        if name == left.name:
            new_expr = BinOp(self, reversed_op, right)
        elif name == right.name:
            if op in ['+', '*']:
                new_expr = BinOp(self, reversed_op, left)
            else:  # op in ['-', '/']:
                new_expr = BinOp(left, op, self)

        return Identifier(name, new_expr)

    def __repr__(self):
        return f'{self.name}: {self.expr}'


class Number:
    def __init__(self, value):
        self.value = value

    def eval(self, module=None):
        return self.value

    def __repr__(self):
        return f'Number({self.value})'


class Visitor:
    def __init__(self, start, end):
        self.done = False
        self.start = start
        self.end = end
        self.path = []

    def generate_reverse_expr(self, node):
        pass

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


class AstGenerator(Visitor):
    def __init__(self, start, end):
        super().__init__(start, end)
        self.prog = Program()

    def generate_reverse_expr(self, node):
        if type(node.expr) == BinOp:
            if node.expr.left in self.path:
                expr = node.what_is(node.expr.left.name)
                self.prog.insert(expr)
                self.prog.insert(Identifier(node.expr.right.name, node.expr.right.eval()))
            if node.expr.right in self.path:
                expr = node.what_is(node.expr.right.name)
                self.prog.insert(expr)
                self.prog.insert(Identifier(node.expr.left.name, node.expr.left.eval()))


class CodeGenerator(Visitor):
    def __init__(self, start, end):
        super().__init__(start, end)
        self.code = []

    def generate_reverse_expr(self, node):
        if type(node.expr) == BinOp:
            if node.expr.left in self.path:
                expr = node.what_is(node.expr.left.name)
                self.code.insert(0, str(expr))
                self.code.insert(0, f'{node.expr.right.name}: {node.expr.right.eval()}')
            if node.expr.right in self.path:
                expr = node.what_is(node.expr.right.name)
                self.code.insert(0, str(expr))
                self.code.insert(0, f'{node.expr.left.name}: {node.expr.left.eval()}')


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

    print('\n[*]')
    print(z4.what_is('z4'))
    print(z4.what_is('x'))
    print(z4.what_is('y'))
