from functools import partial
from operator import add, sub, mul, floordiv

operators = {"+": add, "-": sub, "*": mul, "/": floordiv}
reversed_ops = {'+': '-', '-': '+', '*': '/', '/': '*'}


def handle_expr(name, left, op, right, updated):
    return operators[op](monkeys[left](), monkeys[right]())


def load_data(filename):
    with open(filename, 'r') as f:
        for line in f:
            line = line.rstrip()
            name, expr = line.split(': ')
            if expr.isnumeric():
                yield name, partial(lambda name, value: int(value), name, expr)
            else:
                left, op, right = expr.split()
                yield name, partial(handle_expr, name, left, op, right, False)


def find_monkey_expr(search, arr):
    for value in arr.values():
        if len(value.args) == 2:
            continue
        name, left, op, right, updated = value.args
        if not updated and search in [left, right]:
            return name, left, op, right
    return None


def reverse_expr(name, expr):
    id, left, op, right = expr
    new_expr = None
    if left == name:
        new_expr = (id, reversed_ops[op], right)
    elif right == name:
        if op in ['+', '*']:
            new_expr = (id, reversed_ops[op], left)
        else:
            new_expr = (left, op, id)
    return new_expr


def part_2(search):
    while True:
        monkey = find_monkey_expr(search, monkeys)
        if monkey is None:
            break
        if monkey[0] == 'root':
            branches = [monkey[1], monkey[3]]
            branches.remove(search)
            monkeys[search] = partial(lambda key: monkeys[key](), branches[0])
            break
        left, op, right = reverse_expr(search, monkey)
        monkeys[search] = partial(handle_expr, search, left, op, right, True)
        search = monkey[0]
    return monkeys['humn']


if __name__ == '__main__':
    short = False

    if short:
        filename = 'day21.short.txt'
    else:
        filename = 'day21.long.txt'

    monkeys = dict(load_data(filename))

    print('Part I:', monkeys['root']())
    print('Part II:', part_2('humn')())
