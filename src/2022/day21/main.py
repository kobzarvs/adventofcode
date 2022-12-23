from aoc_ast import *
from aoc_lexer import AocLexer
from aoc_parser import AocParser

if __name__ == '__main__':
    short = False
    ROOT_NODE = 'root'
    TARGET_NODE = 'humn'

    if short:
        filename = 'day21.short.txt'
    else:
        filename = 'day21.long.txt'

    with open(filename) as f:
        data = f.read()

    #
    # Part I
    #

    # Парсинг и вычисление основного потока данных
    lexer = AocLexer()
    parser = AocParser()
    result = parser.parse(lexer.tokenize(data))
    result.eval()
    root = parser.vars[ROOT_NODE]
    print(f'\nPart I: {root.eval()}')

    #
    # Part II
    #

    # Поиск пути от root до humn
    # генерация кода для обратных выражений
    visitor = Visitor(ROOT_NODE, TARGET_NODE)
    traverse(root, visitor)

    # Ветки от root
    left = root.expr.left
    right = root.expr.right

    # Вычисление ветки, в которой находится TARGET_NODE='humn'
    top_node = visitor.path[0]
    if top_node.name == left.name:
        initial = f'{top_node.name}: {right.eval()}'
    else:
        initial = f'{top_node.name}: {left.eval()}'

    # Инициализация переменной-ветки, в которой находится humn
    visitor.code.insert(0, initial)

    # Склейка кода
    code = '\n'.join(visitor.code)
    print(f'\n-----\n{code}\n-----')

    # Парсинг и вычисление сгенерированных обратных выражений
    lexer = AocLexer()
    pars = AocParser()
    result = pars.parse(lexer.tokenize(code))

    print('\nPart II:', pars.vars[TARGET_NODE].eval())
