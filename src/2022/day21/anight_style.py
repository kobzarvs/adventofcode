from collections import namedtuple
from dataclasses import dataclass
from operator import add, sub, mul, floordiv

operators = {"+": add, "-": sub, "*": mul, "/": floordiv}
inv_ops = {'+': '-', '-': '+', '*': '/', '/': '*'}
deps = {}

Mask = namedtuple('Mask', 'NAME', defaults=[''])
Number = namedtuple('Number', 'name value')


@dataclass(match_args=True)
class Expr:
    name: str
    left: str | int | float
    op: str = '+'
    right: str = None
    context = {}

    @staticmethod
    def get(name):
        return Expr.context[name]

    @property
    def value(self):
        return operators[self.op](Expr.context[self.left].value, Expr.context[self.right].value)

    @property
    def inv_op(self):
        return inv_ops[self.op]

    def swap(self, target: Mask):
        match self:
            case Expr(_, target.NAME, _, _):
                return Expr(target.NAME, self.name, self.inv_op, self.right)
            case Expr(_, _, '+' | '*', target.NAME):
                return Expr(target.NAME, self.name, self.inv_op, self.left)
            case Expr(_, _, '-' | '/', target.NAME):
                return Expr(target.NAME, self.left, self.op, self.name)


def load_data(filename):
    with open(filename, 'r') as f:
        for line in f:
            line = line.rstrip()
            name, expr = line.split(': ')
            if expr.isnumeric():
                yield name, Number(name, int(expr))
            else:
                left, op, right = expr.split()
                deps[right] = deps[left] = name
                yield name, Expr(name, left, op, right)


def part_2(target: str) -> int:
    search = target
    while True:
        name = deps[search]
        monkey = Expr.context[name]
        if name == 'root':
            branch = monkey.left if search == monkey.right else monkey.right
            Expr.context[search] = Number(search, Expr.context[branch].value)
            break
        Expr.context[search] = monkey.swap(Mask(NAME=search))
        search = name
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
