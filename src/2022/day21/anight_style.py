from functools import partial
from operator import add, sub, mul, floordiv

operators = {"+": add, "-": sub, "*": mul, "/": floordiv}
reversed_ops = {'+': '-', '-': '+', '*': '/', '/': '*'}


def handle_expr(name, expr):
    left, op, right = expr
    return {
        'expr': (name, left, op, right),
        'eval': lambda: operators[op](monkeys[left]['eval'](), monkeys[right]['eval']()),
    }


def handle_number(name, value):
    return {
        'expr': (name, int(value)),
        'eval': lambda: int(value),
    }


def load_data(filename):
    with open(filename, 'r') as f:
        for line in f:
            line = line.rstrip()
            name, expr = line.split(': ')
            if expr.isnumeric():
                yield name, handle_number(name, expr)
            else:
                yield name, handle_expr(name, expr.split())


def find_monkey(search, arr):
    for value in arr.values():
        if len(value['expr']) != 2 and (value['expr'][1] == search or value['expr'][3] == search):
            return value
    return None


def reverse_expr(name, expr):
    result, left, op, right = expr
    new_expr = None
    if left == name:
        new_expr = (name, result, reversed_ops[op], right)
    elif right == name:
        if op in ['+', '*']:
            new_expr = (name, result, reversed_ops[op], left)
        else:
            new_expr = (name, left, op, result)
    return new_expr


def part_2(search):
    while True:
        monkey = find_monkey(search, monkeys)
        if monkey is None:
            break
        if monkey['expr'][0] == 'root':
            branches = [monkey['expr'][1], monkey['expr'][3]]
            branches.remove(search)
            monkeys[search]['eval'] = partial(lambda key: monkeys[key]['eval'](), branches[0])
            break
        result, left, op, right = reverse_expr(search, monkey['expr'])
        monkeys[search]['eval'] = partial(
            lambda left, op, right: op(monkeys[left]['eval'](), monkeys[right]['eval']()), left, operators[op],
            right)
        search = monkey['expr'][0]
    return monkeys['humn']['eval']()


if __name__ == '__main__':
    short = False

    if short:
        filename = 'day21.short.txt'
    else:
        filename = 'day21.long.txt'

    monkeys = dict(load_data(filename))

    print('Part I:', monkeys['root']['eval']())
    print('Part II:', part_2('humn'))
