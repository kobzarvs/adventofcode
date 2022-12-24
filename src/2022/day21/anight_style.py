from functools import partial
from operator import add, sub, mul, floordiv
from typing import Tuple

Expr = Tuple[str, str, str]

operators = {"+": add, "-": sub, "*": mul, "/": floordiv}
reversed_ops = {'+': '-', '-': '+', '*': '/', '/': '*'}
deps = {}


def handle_expr(left: str, op: str, right: str) -> int:
    return operators[op](monkeys[left](), monkeys[right]())


def load_data(filename):
    with open(filename, 'r') as f:
        for line in f:
            line = line.rstrip()
            name, expr = line.split(': ')
            if expr.isnumeric():
                yield name, partial(lambda value: int(value), expr)
            else:
                left, op, right = expr.split()
                deps[right] = deps[left] = name
                yield name, partial(handle_expr, left, op, right)


def swap_expr(name, value, expr):
    class Tmp:
        NAME = name

    match expr:
        case (Tmp.NAME, op, x):
            return value, reversed_ops[op], x
        case (x, op, Tmp.NAME) if op in ('+', '*'):
            return value, reversed_ops[op], x
        case (x, op, Tmp.NAME) if op in ('-', '/'):
            return x, op, value


def part_2(target: str) -> int:
    search = target
    while True:
        name = deps[search]
        if name not in monkeys:
            raise ValueError(f'No monkey for {name}')
        left, op, right = monkeys[name].args
        if name == 'root':
            branch = left if search == right else right
            monkeys[search] = partial(lambda: monkeys[branch]())
            break
        left, op, right = swap_expr(search, name, (left, op, right))
        monkeys[search] = partial(handle_expr, left, op, right)
        search = name
    return monkeys[target]()


if __name__ == '__main__':
    short = False

    if short:
        filename = 'day21.short.txt'
    else:
        filename = 'day21.long.txt'

    monkeys = dict(load_data(filename))

    print('Part I:', monkeys['root']())
    print('Part II:', part_2('humn'))
