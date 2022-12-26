from __future__ import annotations

import re
from dataclasses import dataclass
from operator import add, sub, mul, floordiv
from types import SimpleNamespace
from typing import Dict, ClassVar, Generator, Tuple, Optional

operators = {"+": add, "-": sub, "*": mul, "/": floordiv}
inv_ops = {'+': '-', '-': '+', '*': '/', '/': '*'}


@dataclass
class Mask:
    NAME: str


@dataclass
class Number:
    name: str
    value: int
    pattern: ClassVar[str] = r'(?P<name>\w+): (?P<value>\d+)'

    def __post_init__(self) -> None:
        self.value = int(self.value)


@dataclass
class Expr:
    name: str
    left: str
    op: str
    right: str
    ref: bool = False

    context: ClassVar[Dict[str, Expr | Number]] = {}
    refs: ClassVar[Dict[str, str]] = {}
    pattern: ClassVar[str] = r'(?P<name>\w+): (?P<left>\w+) (?P<op>[-+*/]) (?P<right>\w+)'

    def __post_init__(self) -> None:
        if self.ref:
            Expr.refs[self.left] = self.name
            Expr.refs[self.right] = self.name

    @property
    def value(self) -> int:
        return operators[self.op](Expr.get(self.left).value, Expr.get(self.right).value)

    @property
    def inv_op(self) -> str:
        return inv_ops[self.op]

    def invert(self, via: str) -> Tuple[str, Expr]:
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
    def get(name: str) -> Expr:
        return Expr.context[name]

    @staticmethod
    def get_all_refs(name) -> Generator[Expr, None, None]:
        while (name := Expr.refs[name]) and name in Expr.refs:
            yield Expr.get(name)


@dataclass
class RegexParser:
    string: str
    match: Optional[re.Match] = None

    def __eq__(self, other: Expr | Number) -> bool:
        self.match = re.search(other.pattern, self.string)
        return self.match is not None

    def __getattr__(self, item: str) -> Optional[str]:
        if self.match is not None:
            return self.match[item]

    @property
    def values(self) -> Optional[Tuple[str, ...]]:
        if self.match is not None:
            return self.match.groups()


Ast = SimpleNamespace(Number=Number, Expr=Expr)


def load_data(filename: str) -> Generator[Tuple[str, Number | Expr], None, None]:
    with open(filename, 'r') as f:
        for line in f:
            match data := RegexParser(line.rstrip()):
                case Ast.Number:
                    yield data.name, Number(*data.values)
                case Ast.Expr:
                    yield data.name, Expr(*data.values, ref=True)


def part_2(target: str) -> int:
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
