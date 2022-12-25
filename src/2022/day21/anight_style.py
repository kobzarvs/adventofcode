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
    def get_by_ref(name):
        return Expr.get(Expr.refs[name])

    @staticmethod
    def put(other):
        Expr.context[other.name] = other
        return other

    def invert(self, target):
        target = Mask(target)
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
                yield name, Expr(name, left, op, right, ref=True)


def part_2(target: str) -> int:
    search = target
    while True:
        monkey = Expr.get_by_ref(search)
        if monkey.name == 'root':
            branch = monkey.left if search == monkey.right else monkey.right
            Expr.put(Number(search, Expr.get(branch).value))
            break
        Expr.put(monkey.invert(search))
        search = monkey.name
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
