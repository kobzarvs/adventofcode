import pygame
import random
import math


WIDTH = 800
HEIGHT = 600
FPS = 15

BLACK = lambda a=255: (20, 20, 20, a)
WHITE = lambda a=255: (255, 255, 255, a)
RED = lambda a=255: (255, 0, 0, a)
GREEN = lambda a=255: (0, 255, 0, a)
BLUE = lambda a=255: (0, 0, 255, a)

# создаем игру и окно
pygame.init()
pygame.mixer.init()  # для звука
pygame.font.init()
my_font = pygame.font.SysFont('Comic Sans MS', 30)

screen = pygame.display.set_mode((WIDTH, HEIGHT))
pygame.display.set_caption("pygame playground")
clock = pygame.time.Clock()


# objects = pygame.sprite.Group()
# objects.add(sprite)


input = '''
R 10
D 20
L 20
U 20
R 20
D 20
L 20
U 20
R 20
'''


def get_move(move):
    dir, steps = move.split(' ')
    return [dir] * int(steps)

packed_moves = [get_move(i) for i in input.strip().split('\n')]
moves = []
for move in packed_moves:
    moves.extend(move)

print(moves)

def get_bounds(moves):
    left = -1
    right = 1
    top = 1
    bottom = -1
    x = 0
    y = 0

    for dir in moves:
        match dir:
            case 'U': y -= 1
            case 'D': y += 1
            case 'L': x -= 1
            case 'R': x += 1
        if x > right:
            right = x
        if x < left:
            left = x
        if y > top:
            top = y
        if y < bottom:
            bottom = y

    return left-1, right+1, top-1, bottom+1

left, right, top, bottom = get_bounds(moves)
print(left, right, top, bottom)


LENGTH = 10
hx = 0
hy = 0
tx = [0.0 for i in range(LENGTH*10)]
ty = [0.0 for i in range(LENGTH*10)]
lx = hx
ly = hy

tail_history = list()
head_history = list()

def move_head(dir, length=LENGTH):
    match dir:
        case 'U': ty[0] -= 1
        case 'D': ty[0] += 1
        case 'L': tx[0] -= 1
        case 'R': tx[0] += 1

    for i in range(1, length, 1):
        dist = math.dist((tx[i-1], ty[i-1]), (tx[i], ty[i]))
        dist_x = tx[i-1] - tx[i]
        dist_y = ty[i-1] - ty[i]
        if dist > math.sqrt(5):
            tx[i] += dist_x / 5
            ty[i] += dist_y / 5

    tail_history.append((tx[length - 1], ty[length - 1]))
    head_history.append((tx[0], ty[0]))

    if len(tail_history) > 20: tail_history.pop(0)
    if len(head_history) > 20: head_history.pop(0)

CENTER = (WIDTH / 2, HEIGHT / 2)
SCALE = 20
OFFSET_X = CENTER[0]
OFFSET_Y = CENTER[1] - 200

def get_xy(xy):
    return (xy[0]*SCALE + OFFSET_X, xy[1]*SCALE + OFFSET_Y)

def draw(screen: pygame.Surface, length=LENGTH):
    for i, xy in enumerate(head_history):
        if i == 0:
            pygame.draw.circle(screen, BLUE(50), get_xy(xy), 10)
        else:
            pygame.draw.circle(screen, BLUE(50), get_xy(xy), 5)
            pygame.draw.line(screen, BLUE(50), get_xy(last_xy), get_xy(xy))
        last_xy = xy

    for i, xy in enumerate(tail_history):
        if i > 0:
            pygame.draw.circle(screen, RED(80), get_xy(xy), 5)
            pygame.draw.line(screen, RED(80), get_xy(last_xy), get_xy(xy))
        last_xy = xy

    for i in range(0, length, 1):
        if i == 0:
            pygame.draw.circle(screen, GREEN(), get_xy((tx[i], ty[i])), 12)
        else:
            pygame.draw.circle(screen, GREEN(180), get_xy((tx[i], ty[i])), 10)
            pygame.draw.line(screen, GREEN(180), get_xy((tx[i-1], ty[i-1])), get_xy((tx[i], ty[i])))


pos = (0, 0)
srf = screen.convert_alpha()
# srf.set_colorkey((1, 0, 0, 255))
# srf.set_alpha(50)

i = 0
dir = 'R'
length = LENGTH

running = True
while running:
    clock.tick(FPS)

    for event in pygame.event.get():
        print(event)
        match event.type:
            case pygame.QUIT:
                running = False
            case pygame.MOUSEMOTION:
                pos = event.pos
            case pygame.KEYDOWN:
                match event.key:
                    case pygame.K_UP: dir = 'U'
                    case pygame.K_DOWN: dir = 'D'
                    case pygame.K_LEFT: dir = 'L'
                    case pygame.K_RIGHT: dir = 'R'
                    case 1073741911: length += 1
                    case 1073741910: length -= 1

    # objects.update()
    move_head(dir, length=length)
    i = (i + 1) % len(moves)

    # clear screen
    screen.fill(BLACK())

    # draw objects
    srf.fill((0, 0, 0, 50))
    # objects.draw(srf)
    draw(srf, length=length)

    text_surface = my_font.render(f'Length: {length}', False, (255, 255, 255))
    
    # draw screen
    screen.blit(srf, (0, 0))
    screen.blit(text_surface, (0, 0))
    pygame.display.flip()

pygame.quit()

# pygame.event.clear()
# while True:
#     event = pygame.event.wait()
#     if event.type == pygame.QUIT:
#         pygame.quit()
#         sys.exit()
#     elif event.type == pygame.KEYDOWN:
#         break
