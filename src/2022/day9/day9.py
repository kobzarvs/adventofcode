import os
import sys
import pygame
import random
import math


WIDTH = 1800
HEIGHT = 900
FPS = 30

BLACK = lambda a=255: (20, 20, 20, a)
WHITE = lambda a=255: (255, 255, 255, a)
RED = lambda a=255: (255, 0, 0, a)
GREEN = lambda a=255: (0, 255, 0, a)
BLUE = lambda a=255: (0, 0, 255, a)

pygame.init()
pygame.mixer.init()
pygame.font.init()
my_font = pygame.font.SysFont('Comic Sans MS', 30)

screen = pygame.display.set_mode((WIDTH, HEIGHT))
pygame.display.set_caption("pygame playground")
clock = pygame.time.Clock()


def load_data():
	with open('src/2022/day9/input.txt', 'r') as f:
		for line in f:
			line = line.rstrip()
			dir, steps = line.split(' ')
			yield [dir] * int(steps)

def get_move(move):
    dir, steps = move.split(' ')
    return [dir] * int(steps)

moves = []
for move in load_data():
    moves.extend(move)


LENGTH = 10
hx = 0
hy = 0
tx = [0.0 for i in range(LENGTH*10)]
ty = [0.0 for i in range(LENGTH*10)]
lx = hx
ly = hy

tail_history = list()
head_history = list()


def get_bounds(moves):
    """
    It takes a list of moves and returns the bounds of the grid that would be needed to draw the path
    
    :param moves: a list of moves, e.g. ['U', 'D', 'L', 'R']
    :return: The bounds of the grid.
    """
    left = -1
    right = 1
    top = 1
    bottom = -1
    x = y = 0

    for dir in moves:
        match dir:
            case 'U': y -= 1
            case 'D': y += 1
            case 'L': x -= 1
            case 'R': x += 1

        right = max(right, x)
        left = min(left, x)
        top = max(top, y)
        bottom = min(bottom, y)

    return left-1, right+1, top-1, bottom+1


left, right, top, bottom = get_bounds(moves)
print(left, right, top, bottom)


SCALE = 4 # min(WIDTH / (right - left), HEIGHT / (top - bottom))
CENTER = (WIDTH / 2, HEIGHT / 2)
OFFSET_X = CENTER[0] - 200 #  + SCALE * (right - left) / 2
OFFSET_Y = CENTER[1] - 170 # + SCALE * (top - bottom) / 2
SIZE = .25 * SCALE


def move_head(dir, length=LENGTH):
    """
    It moves the head of the snake in the direction specified by the argument, and then it moves the
    tail of the snake to the position of the head of the snake before the head was moved
    
    :param dir: The direction the head is moving in
    :param length: The length of the snake
    """
    match dir:
        case 'U': ty[0] -= 1
        case 'D': ty[0] += 1
        case 'L': tx[0] -= 1
        case 'R': tx[0] += 1

    for i in range(1, length, 1):
        dist = math.dist((tx[i-1], ty[i-1]), (tx[i], ty[i]))
        dx = tx[i-1] - tx[i]
        dy = ty[i-1] - ty[i]
        if dist > math.sqrt(2):
            tx[i] += 0 if dx == 0 else abs(dx)/dx
            ty[i] += 0 if dy == 0 else abs(dy)/dy 

    tail_history.append((tx[length - 1], ty[length - 1]))
    head_history.append((tx[0], ty[0]))


def get_xy(xy):
    """
    It takes a tuple of two numbers, multiplies the first by the scale factor, adds the x offset,
    multiplies the second by the scale factor, and adds the y offset
    
    :param xy: The xy coordinates of the point to be drawn
    :return: The x and y coordinates of the point.
    """
    return (xy[0]*SCALE + OFFSET_X, xy[1]*SCALE + OFFSET_Y)


def draw(screen: pygame.Surface, length=LENGTH):
    """
    It draws the head and tail of the snake, and the body of the snake.
    
    :param screen: the surface to draw on
    :type screen: pygame.Surface
    :param length: The length of the tail
    """
    for i, xy in enumerate(head_history):
        if i == 0:
            pygame.draw.circle(screen, BLUE(50), get_xy(xy), SIZE * 2)
        else:
            pygame.draw.circle(screen, BLUE(50), get_xy(xy), SIZE)
            pygame.draw.line(screen, BLUE(50), get_xy(last_xy), get_xy(xy))
        last_xy = xy

    for i, xy in enumerate(tail_history):
        if i > 0:
            pygame.draw.circle(screen, RED(80), get_xy(xy), SIZE)
            pygame.draw.line(screen, RED(80), get_xy(last_xy), get_xy(xy))
        last_xy = xy

    for i in range(0, length, 1):
        if i == 0:
            pygame.draw.circle(screen, GREEN(), get_xy((tx[i], ty[i])), SIZE * 2)
        else:
            pygame.draw.circle(screen, GREEN(180), get_xy((tx[i], ty[i])), SIZE * 2)
            pygame.draw.line(screen, GREEN(180), get_xy((tx[i-1], ty[i-1])), get_xy((tx[i], ty[i])))

def run():
    length = LENGTH

    running = True
    for i, dir in enumerate(moves):
        for event in pygame.event.get():
            match event.type:
                case pygame.QUIT:
                    running = False
        
        if not running:
            break

        # update moves
        move_head(dir, length=length)
        
        if i != 11395 and i % 50 != 0:
            continue

        # clear screen
        screen.fill(BLACK())

        srf = screen.copy()

        # draw objects
        srf.fill((0, 0, 0, 50))
        draw(srf, length=length)
        screen.blit(srf, (0, 0))

        text_surface = my_font.render(f'Tail visits: {len(set(tail_history))}', False, (255, 255, 255))
        screen.blit(text_surface, (12, 8))

        pygame.display.flip()

    while running:
        for event in pygame.event.get():
            match event.type:
                case pygame.QUIT:
                    running = False

    print(i, len(set(tail_history)))
    pygame.quit()


if __name__ == '__main__':
    run()
