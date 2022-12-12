import sys
import tokenize as tk
from tokenize import tokenize, untokenize, NUMBER, STRING, NAME, OP
from io import BytesIO
from dataclasses import dataclass
import pygame


WIDTH = 1800
HEIGHT = 900
FPS = 30

BLACK = lambda a=255: (20, 20, 20, a)
WHITE = lambda a=255: (255, 255, 255, a)
RED = lambda a=255: (255, 0, 0, a)
GREEN = lambda a=255: (0, 255, 0, a)
BLUE = lambda a=255: (0, 0, 255, a)


def init_pygame():
    pygame.init()
    pygame.mixer.init()
    pygame.font.init()
    font = pygame.font.SysFont('Comic Sans MS', 30)
    screen = pygame.display.set_mode((WIDTH, HEIGHT))
    pygame.display.set_caption("pygame playground")
    clock = pygame.time.Clock()
    
    return screen, clock, font


lines = []
with open('src/2022/day11/input.txt', 'r') as f:
    for line in f:
        line = line.rstrip().encode('utf-8')
        if line:
            lines.append(line)

lines.append(b'')

input = """
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"""

# lines = input.split('\n')
# lines = [line.encode('utf-8') for line in lines]
# lines.append(b'')


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

screen, clock, my_font = init_pygame()

def event_handler(key=None):
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            sys.exit()
        if event.type == pygame.KEYDOWN:
            if key is None or event.key == key:
                return True
            else:
                return False

pygame.event.clear()


def perform_round():
    for i, monkey in enumerate(monkeys.values()):
        while monkey.items:
            item = monkey.get_item()
            item2 = item if monkey.vars[2] == 'old' else int(monkey.vars[2])

            if monkey.op == '*':
                item *= item2
            elif monkey.op == '+':
                item += item2

            item //= 3

            if item % monkey.div_by == 0:
                monkeys[monkey.throw[0]].add_item(item)
            else:
                monkeys[monkey.throw[1]].add_item(item)


def interactive():
    # render
    screen.fill(BLACK())

    text = my_font.render(f'Round: {round + 1}', False, (255, 255, 255))
    screen.blit(text, (10, 5))

    for i, monkey in enumerate(monkeys.values()):
        text = my_font.render(f'{monkey}', False, (255, 255, 255))
        screen.blit(text, (10, i * 32 + 50))

    pygame.display.flip()


    while not event_handler(pygame.K_SPACE):
        clock.tick(FPS)
        pass


MAX_ROUND = 20

for round in range(MAX_ROUND):
    # update data
    perform_round()



print('---')
for monkey in monkeys.values():
    print(monkey)

inspections = sorted([m.inspections for m in monkeys.values()], reverse=True)
print(inspections[0] * inspections[1])
