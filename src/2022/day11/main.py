import sys
import tokenize as tk
import math
import itertools as it

from interactive import interactive


class Monkey:
    id: int
    items: list
    vars: list
    op: str
    div_by: int
    throw: list
    inspections: int

    def __init__(self, id):
        self.id = id
        self.items = list()
        self.vars = list()
        self.op = None
        self.throw = list()
        self.div_by = None
        self.inspections = 0
    
    def __repr__(self):
        return f'Monkey #{self.id} inspections: {self.inspections} {self.items}'

    def get_item(self):
        self.inspections += 1
        return self.items.pop(0)

    def add_item(self, item):
        self.items.append(item)
    

def enum(**enums):
    return type('Enum', (), enums)


def render(screen, font, clock, event, ctx={ 'monkeys': {}, 'round': 0 }):
    round = ctx['round'] + 1
    text = font.render(f'Round: {round}', False, (255, 255, 255))
    screen.blit(text, (10, 5))

    for i, monkey in enumerate(ctx['monkeys'].values()):
        text = font.render(f'{monkey}', False, (255, 255, 255))
        screen.blit(text, (10, i * 32 + 50))


class Game:
    def __init__(self, max_round, with_lcm=False, debug=True, short=True):
        self.monkeys = {}
        self.max_round = max_round
        self.round = 0
        self.with_lcm = with_lcm
        self.lcm = None
        self.lines = []
        self.debug = debug

        self.load_data(short=short)
        self.parse()

    def load_data(self, short=False):
        if short:
            self.filename = 'src/2022/day11/short.txt'
        else:
            self.filename = 'src/2022/day11/input.txt'
        
        self.lines = []
        with open(self.filename, 'r') as f:
            for line in f:
                line = line.rstrip().encode('utf-8')
                if line:
                    self.lines.append(line)
        self.lines.append(b'')
        return self.lines


    def parse(self, lines=None):
        i = 0
        lines = lines or self.lines
        def readline():
            nonlocal i
            line = lines[i]
            i += 1
            return line

        States = enum(MONKEY=1, ITEMS=2, OPERATION=3, TEST=4, THROW=5)

        state = None
        monkeys = self.monkeys
        monkey = None

        tokens = tk.tokenize(readline)
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
        if self.with_lcm:
            self.lcm = math.lcm(*[monkey.div_by for monkey in monkeys.values()])

        return monkeys

    def perform(self, monkeys=None, render=None):
        monkeys = self.monkeys

        for round in range(self.max_round):
            self.round = round
            self.perform_round()
            if render:
                interactive(render=render, ctx={ 'monkeys': monkeys, 'round': round })
        
        inspections = sorted([m.inspections for m in monkeys.values()], reverse=True)
        result = inspections[0] * inspections[1]

        if self.debug:
            print('---')
            for monkey in monkeys.values():
                print(monkey)
            print(f'inspections: {result}')

        return result

    def perform_round(self):
        monkeys = self.monkeys
        lcm = self.lcm

        for monkey in monkeys.values():
            while monkey.items:
                item = monkey.get_item()
                item2 = item if monkey.vars[2] == 'old' else int(monkey.vars[2])

                if monkey.op == '*':
                    item *= item2
                elif monkey.op == '+':
                    item += item2

                if lcm == None:
                    item //= 3
                else:
                    item %= lcm

                if item % monkey.div_by == 0:
                    monkeys[monkey.throw[0]].add_item(item)
                else:
                    monkeys[monkey.throw[1]].add_item(item)


# Part I
game_1 = Game(max_round=20, with_lcm=False, debug=True, short=False)
game_1.perform()

# Part II
game_2 = Game(max_round=10000, with_lcm=True, debug=True, short=False)
game_2.perform()
