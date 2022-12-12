import pygame


WIDTH = 1800
HEIGHT = 900
FPS = 30

BLACK = lambda a=255: (20, 20, 20, a)
WHITE = lambda a=255: (255, 255, 255, a)
RED = lambda a=255: (255, 0, 0, a)
GREEN = lambda a=255: (0, 255, 0, a)
BLUE = lambda a=255: (0, 0, 255, a)


initialized = False
screen = clock = font = None

def init_pygame():
    global initialized, screen, clock, font

    if initialized:
        return screen, clock, font

    pygame.init()
    pygame.mixer.init()
    pygame.font.init()
    font = pygame.font.SysFont('Comic Sans MS', 30)
    screen = pygame.display.set_mode((WIDTH, HEIGHT))
    pygame.display.set_caption("ESC to continue...")
    clock = pygame.time.Clock()
    initialized = True
    
    return screen, clock, font


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


def interactive(update=None, render=None, ctx=None):
    screen, clock, my_font = init_pygame()

    # update models
    if update:
        update(ctx)

    # render
    screen.fill(BLACK())

    if render:
        render(screen, font=my_font, clock=clock, event=pygame.event, ctx=ctx)

    pygame.display.flip()

    while not event_handler(pygame.K_ESCAPE):
        clock.tick(FPS)
        pass
