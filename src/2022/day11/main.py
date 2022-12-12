import tokenize as tk
from tokenize import tokenize, untokenize, NUMBER, STRING, NAME, OP
from io import BytesIO
from dataclasses import dataclass


lines = []
with open('src/2022/day11/input.txt', 'r') as f:
    for line in f:
        line = line.rstrip().encode('utf-8')
        if line:
            lines.append(line)

lines.append(b'')


class Monkey:
    id: int
    items: list
    vars: list
    op: str
    div_by: int
    throw: list

    def __init__(self, id):
        self.id = id
        self.items = list()
        self.vars = list()
        self.op = None
        self.throw = list()
        self.div_by = None
    
    def __repr__(self):
        return f'Monkey {self.id} has {len(self.items)} items and {len(self.vars)} vars and operation "{self.op}" and div by {self.div_by} throws {self.throw}'

    def add_item(self, item):
        self.items.append(item)
    

i = 0
def readline():
    global i
    line = lines[i]
    i += 1
    return line


def enum(**enums):
    return type('Enum', (), enums)


States = enum(MONKEY=1, ITEMS=2, OPERATION=3, TEST=4, THROW=5)

state = None
monkeys = dict({})
monkey = None

tokens = tokenize(readline)
for token in tokens:
    match state:
        case None:
            if token.type == tk.NAME and token.string == 'Monkey':
                state = States.MONKEY

        case States.MONKEY:
            if token.type == tk.NUMBER:
                monkey = Monkey(int(token.string))
                monkeys[int(token.string)] = monkey
                state = States.ITEMS

        case States.ITEMS:
            match token.type:
                case tk.NUMBER:
                    monkey.add_item(int(token.string))
                case tk.NAME:
                    if token.string == 'Operation':
                        state = States.OPERATION

        case States.OPERATION:
            match token.type:
                case tk.NAME | tk.NUMBER:
                    monkey.vars.append(token.string)
                case tk.OP:
                    if token.string == '*' or token.string == '+':
                        monkey.op = token.string
            if len(monkey.vars) == 3:
                state = States.TEST

        case States.TEST:
            if token.type == tk.NUMBER:
                monkey.div_by = int(token.string)
                state = States.THROW

        case States.THROW:
            if token.type == tk.NUMBER:
                monkey.throw.append(int(token.string))
                if len(monkey.throw) == 2:
                    state = None

for monkey in monkeys.values():
    print(monkey)
