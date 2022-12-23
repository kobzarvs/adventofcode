import numpy as np
import pygame
from colorir import Grad
from matplotlib.colors import to_rgb

from interactive import Renderer, BLACK, WHITE

brown, white = to_rgb('brown'), to_rgb('white')
print(brown, white)
palette = Grad(['#ffffff', '#30B500', '#1F7500', '#C48000', '#754C00']).n_colors(26)


# point no.2
def mix(col0, col1, ratio):
    return tuple(cmp0 * ratio + cmp1 * (1 - ratio)
                 for cmp0, cmp1 in zip(col0, col1))


CELL_SIZE = 20


class Cartographer(Renderer):
    def __init__(self, short=True):
        super().__init__(fps=120)
        self.size = None
        self.start = None
        self.height_map = None
        self.end = None
        self.filename = None
        self.load_data(short=short)
        self.run(start_key=pygame.K_ESCAPE, slide_show_key=pygame.K_SPACE)

    def load_data(self, short=False):
        if short:
            self.filename = 'short.txt'
        else:
            self.filename = 'input.txt'

        start = (-1, 0)
        end = (-1, 0)
        hm = []

        with open(self.filename, 'r') as f:
            for row, line in enumerate(f):
                line = line.rstrip()
                if 'S' in line:
                    start = (line.find('S'), row)
                    line = line.replace('S', 'a')
                if 'E' in line:
                    end = (line.find('E'), row)
                    line = line.replace('E', 'z')
                hm.append([ord(h) - ord('a') for h in line])

        self.height_map = np.array(hm, dtype=int)
        self.size = self.height_map.shape
        self.start = start
        self.end = end

    path = []
    visited = set()
    best_path = None

    def find_path(self, start):
        self.path.append(start)
        self.visited.add(start)
        yield 'enter', start, self.end, self.path, self.height_map

        if start == self.end:
            if not self.best_path or len(self.path) < len(self.best_path):
                self.best_path = self.path.copy()
            print('end', len(self.path))
            yield 'found path', start, self.end, self.path, self.height_map
            self.path.pop()
            # return True
            return

        cx, cy = start
        current_height = self.height_map[cy, cx]
        for x, y in self.get_neighbours_xy(start):
            delta = self.height_map[y, x] - current_height
            if (x, y) not in self.visited and delta in [0, 1]:
                # print('**', delta, x, y)
                for result in self.find_path((x, y)):
                    yield result
                    # print('***', result)
                    if result[0] == 'found path':
                        break
        self.path.pop()
        # return False
        # yield 'exit', start, self.end, self.path, self.height_map
        return

    def update(self):
        for ctx in self.find_path(self.start):
            yield ctx

    def get_neighbours_xy(self, point):
        x, y = point
        neighbours = []
        if x > 0:
            neighbours.append((x - 1, y))
        if x < self.size[1] - 1:
            neighbours.append((x + 1, y))
        if y > 0:
            neighbours.append((x, y - 1))
        if y < self.size[0] - 1:
            neighbours.append((x, y + 1))
        return neighbours

    def render(self, scr: pygame.Surface, font: pygame.font.Font, is_pass: bool, event: pygame.event.Event, pg: pygame,
               ctx: tuple):
        # s = pygame.Surface((350, 350), pygame.SRCALPHA)
        # s.set_alpha(255)
        # self.clear(BLACK(50))

        # pg.draw.rect(scr, RED(255), (100, y, 100, 100))
        # pg.draw.rect(s, BLUE(50), (200, 200, 100, 100))
        # pg.draw.rect(s, GREEN(150), (150, 150, 100, 100))
        # y += 5
        # y %= 300

        # pg.draw.circle(scr, GREEN(255), (100, 100), 5)
        # pg.draw.circle(scr, RED(255), (300, 170), 5)
        # mxy = pg.mouse.get_pos()
        # appart_pt = move_pt(pt1=(100, 100), pt2=mxy, dist=400)
        # draw_arrow(scr, pg, (100, 100), appart_pt, GREEN(255), 2)
        for ((row, col), val) in np.ndenumerate(self.height_map):
            # print(row, col)
            x = col * CELL_SIZE + 20
            y = row * CELL_SIZE + 60
            # pg.draw.rect(scr, BLACK(255), (x, y, CELL_SIZE, CELL_SIZE))
            # print(palette[val])
            pg.draw.rect(scr, palette[val], (x, y, CELL_SIZE, CELL_SIZE))

            # text = font.render(f'{val:>2}', True, WHITE())
            # scr.blit(text, [x + 5, y + 5])

        for i in self.path:
            x = i[0] * CELL_SIZE + 20
            y = i[1] * CELL_SIZE + 60
            pg.draw.rect(scr, BLACK(), (x, y, CELL_SIZE, CELL_SIZE), 2)
            pg.draw.rect(scr, (255, 255, 255), (x + 2, y + 2, CELL_SIZE - 4, CELL_SIZE - 4))

        x = self.end[0] * CELL_SIZE + 20
        y = self.end[1] * CELL_SIZE + 60
        pg.draw.rect(scr, BLACK(), (x, y, CELL_SIZE, CELL_SIZE), 2)
        pg.draw.rect(scr, (255, 50, 50), (x + 2, y + 2, CELL_SIZE - 4, CELL_SIZE - 4))

        status, start, self.end, self.path, self.height_map = ctx

        x = start[0] * CELL_SIZE + 20
        y = start[1] * CELL_SIZE + 60
        pg.draw.rect(scr, BLACK(), (x, y, CELL_SIZE, CELL_SIZE), 2)
        pg.draw.rect(scr, (50, 50, 255), (x + 2, y + 2, CELL_SIZE - 4, CELL_SIZE - 4))

        text = font.render(f'ctx: {ctx[0]}', True, WHITE())
        Renderer.frame += 1
        scr.blit(text, [5, 32])

        # print(ctx)


cart = Cartographer(short=True)
print(cart.start, cart.end)

# cart.run(start_key=pygame.K_ESCAPE, slide_show_key=pygame.K_SPACE)


# print(c.start, c.end)
# print('path', c.best_path)
# print(len(c.best_path) - 1)
# print(c.path)
# print(c.height_map)
