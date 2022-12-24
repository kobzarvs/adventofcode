from functools import partial
from operator import add, sub, mul, floordiv

operators = {"+": add, "-": sub, "*": mul, "/": floordiv}
reversed_ops = {'+': '-', '-': '+', '*': '/', '/': '*'}
deps = {}


def handle_expr(left, op, right):
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


def swap_expr(name, via_value, expr):
    left, op, right = expr
    new_expr = (via_value, reversed_ops[op], right)
    if right == name:
        if op in ['+', '*']:
            new_expr = (via_value, reversed_ops[op], left)
        else:
            new_expr = (left, op, via_value)
    return new_expr


def part_2(target):
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
