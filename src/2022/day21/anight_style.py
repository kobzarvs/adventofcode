from collections import namedtuple
from dataclasses import dataclass
from operator import add, sub, mul, floordiv

operators = {"+": add, "-": sub, "*": mul, "/": floordiv}
reversed_ops = {'+': '-', '-': '+', '*': '/', '/': '*'}
deps = {}

Mask = namedtuple('Mask', 'search replace', defaults=['', ''])


@dataclass
class Number:
    name: str
    value: int

    def eval(self):
        return self.value


@dataclass
class Expr:
    name: str
    left: str
    op: str
    right: str
    context = {}

    @staticmethod
    def get(name):
        return Expr.context[name]

    def eval(self):
        return operators[self.op](Expr.context[self.left].eval(), Expr.context[self.right].eval())

    def rop(self):
        return reversed_ops[self.op]

    def swap(self, mask: Mask):
        match self:
            case Expr(name, mask.search, _, right):
                return Expr(mask.search, name, self.rop(), right)
            case Expr(name, left, '+' | '*', mask.search):
                return Expr(mask.search, name, self.rop(), left)
            case Expr(name, left, '-' | '/', mask.search):
                return Expr(mask.search, left, self.op, name)


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
            Expr.context[search] = Number(search, Expr.context[branch].eval())
            break
        Expr.context[search] = monkey.swap(Mask(search))
        search = name
    return Expr.get(target).eval()


if __name__ == '__main__':
    short = False

    if short:
        filename = 'day21.short.txt'
    else:
        filename = 'day21.long.txt'

    Expr.context = dict(load_data(filename))

    print('Part I:', Expr.get('root').eval())
    print('Part II:', part_2('humn'))
