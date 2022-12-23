import sys
from time import sleep
import pygame
import math
from typing import List, Callable


WIDTH = 800
HEIGHT = 600
FPS = 120


def BLACK(a=255): return (20, 20, 20, a)
def WHITE(a=255): return (255, 255, 255, a)
def RED(a=255): return (255, 0, 0, a)
def GREEN(a=255): return (0, 255, 0, a)
def BLUE(a=255): return (0, 0, 255, a)


initialized = False
screen = clock = font = None


def event_handler(key=None):
    """
    "If the user has pressed the key that was passed to the function, return True. Otherwise, return
    False."

    The first thing we do is get a list of all the events that have happened since the last time we
    called this function. Then we loop through each event in the list

    :param key: The key to check for. If None, any key will do
    :return: A boolean value.
    """
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            sys.exit()
        if event.type == pygame.KEYDOWN:
            if key is None or event.key == key:
                yield True
            else:
                yield False


def move_pt(pt1, pt2=None, angle=0, dist=1):
    """
    Move a point in a given direction by a given distance.

    :param pt1: The point to move
    :param pt2: The point to move away from. If this is None, then the angle parameter is used
    :param angle: The angle of the line, defaults to 0 (optional)
    :param dist: The distance to move the point, defaults to 1 (optional)
    :return: A tuple of the x and y coordinates of the new point.
    """
    x, y = pt1

    if pt2:
        angle = math.atan2(pt2[1] - pt1[1], pt2[0] - pt1[0])

    x += math.cos(angle) * dist
    y += math.sin(angle) * dist
    return x, y


def draw_arrow(scr: pygame.Surface, pg: pygame, pt1, pt2, color, width=1):
    """
    Draw a line from pt1 to pt2, then draw a triangle at pt2 with the base of the triangle pointing in
    the direction of the line

    :param scr: the screen to draw on
    :param pg: the pygame surface to draw on
    :param pt1: The starting point of the arrow
    :param pt2: The point where the arrow will be pointing to
    :param color: The color of the arrow
    :param width: The width of the line, defaults to 1 (optional)
    """
    pg.draw.line(scr, color, pt1, pt2, width)
    angle = math.atan2(pt1[1] - pt2[1], pt1[0] - pt2[0])
    pt3 = move_pt(pt2, angle=angle - math.pi / 12, dist=25)
    pt4 = move_pt(pt2, angle=angle + math.pi / 12, dist=25)
    pg.draw.polygon(scr, color, [pt2, pt3, pt4])


class Renderer:
    initialized = False

    def __init__(self, width=WIDTH, height=HEIGHT, fps=60, title="ESC to continue...", show_fps=True):
        Renderer.width = width
        Renderer.height = height
        Renderer.title = title
        Renderer.show_fps = show_fps

        Renderer.init_pygame()

        Renderer.pygame = pygame
        Renderer.event = pygame.event
        Renderer.fps = fps

        self.manual_commit = False
        self.manual_clear = False

    @staticmethod
    def init_pygame():
        """
        It initializes pygame, and returns the screen, clock, and font objects
        :return: screen, clock, font
        """
        if Renderer.initialized:
            return Renderer.screen, Renderer.clock, Renderer.font

        pygame.init()
        pygame.mixer.init()
        pygame.font.init()
        font = pygame.font.SysFont('Dejavu Sans Mono', size=16, bold=False)
        screen = pygame.display.set_mode((Renderer.width, Renderer.height))
        pygame.display.set_caption(Renderer.title)
        clock = pygame.time.Clock()
        Renderer.initialized = True
        Renderer.screen = screen
        Renderer.font = font
        Renderer.clock = clock

        return screen, clock, font

    started = False
    start_key=None
    slide_show_key=None
    state = 'render'

    def event_loop(self):
        # self.state = 'render'
        # while True:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                sys.exit()
            if not self.started and self.start_key:
                self.state = 'render'
                yield event
                self.state = 'pause'
                self.started = True
                continue
            else:
                if event.type == pygame.KEYDOWN:
                    match event.key:
                        case self.start_key:
                            self.state = 'render' if self.state == 'pause' else 'pause'
                            yield event
                        case self.slide_show_key:
                            self.state = 'slide-render' if self.state == 'pause' else 'pause'
                            yield event
                else:
                    yield event
        yield None



    def update(self):
        while True:
            yield

    def run(self, start_key=None, slide_show_key=None):
        self.start_key = start_key
        self.slide_show_key = slide_show_key

        for _ in self.update():
            while True:
                for event in self.event_loop():
                    Renderer.clock.tick(Renderer.fps)
                    self.renderer(Renderer.screen, Renderer.font, self.state, event)
                    if self.state == 'pause':
                        continue
                if self.state != 'pause':
                    break
            if self.state == 'slide-render':
                self.state = 'pause'

    def clear(self, color=BLACK()):
        Renderer.screen.fill(color)

    frame = 0
    def renderer(self, scr: pygame.Surface, font: pygame.font.Font, is_pass: bool, event: pygame.event.Event):
        if not self.manual_clear:
            self.screen.fill(BLACK())

        self.render(scr, font, is_pass, event, pygame)

        if Renderer.show_fps:
            text = font.render(f'frame: {Renderer.frame} state: {self.state}', True, WHITE())
            Renderer.frame += 1
            scr.blit(text, [5, 5])

        if not self.manual_commit:
            pygame.display.flip()

    def render(self, scr: pygame.Surface, font: pygame.font.Font, is_pass: bool, event: pygame.event.Event, pg: pygame):
        pass
