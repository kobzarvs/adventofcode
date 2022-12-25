from collections import namedtuple
from dataclasses import dataclass
from operator import add, sub, mul, floordiv

operators = {"+": add, "-": sub, "*": mul, "/": floordiv}
inv_ops = {'+': '-', '-': '+', '*': '/', '/': '*'}

Mask = namedtuple('Mask', 'NAME', defaults=[''])
Number = namedtuple('Number', 'name value')


@dataclass(match_args=True)
class Expr:
    name: str
    left: str
    op: str
    right: str
    ref: bool = False
    context = {}
    refs = {}

    def __new__(cls, *args, **kwargs):
        if 'ref' in kwargs:
            cls.refs[args[1]] = args[0]
            cls.refs[args[3]] = args[0]
        return super().__new__(cls)

    @property
    def value(self):
        return operators[self.op](Expr.context[self.left].value, Expr.context[self.right].value)

    @property
    def inv_op(self):
        return inv_ops[self.op]

    @staticmethod
    def get(name):
        return Expr.context[name]

    @staticmethod
    def get_all_refs(name):
        while name in Expr.refs:
            yield Expr.get(name := Expr.refs[name])

    def invert(self, target):
        target = Mask(target)
        new_expr = Expr(target.NAME, self.name, self.inv_op, self.right)
        match self:
            case Expr(_, _, '+' | '*', target.NAME):
                new_expr = Expr(target.NAME, self.name, self.inv_op, self.left)
            case Expr(_, _, '-' | '/', target.NAME):
                new_expr = Expr(target.NAME, self.left, self.op, self.name)
        Expr.context[target.NAME] = new_expr
        return self.name, new_expr


def load_data(filename):
    with open(filename, 'r') as f:
        for line in f:
            line = line.rstrip()
            name, expr = line.split(': ')
            if expr.isnumeric():
                yield name, Number(name, int(expr))
            else:
                left, op, right = expr.split()
                yield name, Expr(name, left, op, right, ref=True)


def part_2(target):
    search = target
    for monkey in Expr.get_all_refs(target):
        if monkey.name != 'root':
            search, expr = monkey.invert(search)
    else:
        root = Expr.get('root')
        branch = root.left if search == root.right else root.right
        Expr.context[search] = Number(search, Expr.get(branch).value)
    return Expr.get(target).value


if __name__ == '__main__':
    short = False

    if short:
        filename = 'day21.short.txt'
    else:
        filename = 'day21.long.txt'

    Expr.context = dict(load_data(filename))

    print('Part I:', Expr.get('root').value)
    print('Part II:', part_2('humn'))
