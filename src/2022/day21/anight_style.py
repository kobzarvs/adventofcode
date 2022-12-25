from __future__ import annotations

import re
from dataclasses import dataclass
from operator import add, sub, mul, floordiv
from typing import Dict, ClassVar, NamedTuple

operators = {"+": add, "-": sub, "*": mul, "/": floordiv}
inv_ops = {'+': '-', '-': '+', '*': '/', '/': '*'}


class Mask(NamedTuple):
    NAME: str


class Number(NamedTuple):
    name: str
    value: int


@dataclass
class Expr:
    name: str
    left: str
    op: str
    right: str
    ref: bool = False

    context: ClassVar[Dict[str, Expr | Number]] = {}
    refs: ClassVar[Dict[str, str]] = {}

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

    def invert(self, via):
        target = Mask(via)
        new_expr = Expr(target.NAME, self.name, self.inv_op, self.right)
        match self:
            case Expr(_, _, '+' | '*', target.NAME):
                new_expr = Expr(target.NAME, self.name, self.inv_op, self.left)
            case Expr(_, _, '-' | '/', target.NAME):
                new_expr = Expr(target.NAME, self.left, self.op, self.name)
        Expr.context[target.NAME] = new_expr
        return self.name, new_expr

    @staticmethod
    def get(name):
        return Expr.context[name]

    @staticmethod
    def get_all_refs(name):
        while (name := Expr.refs[name]) and name in Expr.refs:
            yield Expr.get(name)


@dataclass
class RegexEqual(str):
    string: str
    match: re.Match | None = None

    def __eq__(self, pattern):
        self.match = re.search(pattern, self.string)
        return self.match is not None

    def __getattr__(self, item):
        return self.match[item]


def load_data(filename):
    with open(filename, 'r') as f:
        for line in f:
            match RegexEqual(line.rstrip()):
                case r'(?P<name>\w+): (?P<value>\d+)' as e:
                    yield e.name, Number(e.name, int(e.value))
                case r'(?P<name>\w+): (?P<left>\w+) (?P<op>[+*-/]) (?P<right>\w+)' as e:
                    yield e.name, Expr(e.name, e.left, e.op, e.right, ref=True)


def part_2(target):
    search = target
    for monkey in Expr.get_all_refs(target):
        search, _ = monkey.invert(via=search)

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
