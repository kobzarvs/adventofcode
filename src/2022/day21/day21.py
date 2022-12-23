from aoc_lexer import SlyLexer
from aoc_parser import SlyParser
from aoc_ast import *


def generate_code(vars, path, debug=False):
    code = initial + '\n'
    for key, value in vars.items():
        if key in path:
            if type(value.expr) == BinOp:
                if value.expr.left.name in path:
                    code += f'{value.expr.right.name}: {value.expr.right.eval()}\n'
                    code += f'{value.what_is(value.expr.left.name)}\n'
                if value.expr.right.name in path:
                    code += f'{value.expr.left.name}: {value.expr.left.eval()}\n'
                    code += f'{value.what_is(value.expr.right.name)}\n'
    if debug:
        print('\n- generated reverse code -')
        print(code)
        print('--------------------------\n')
    return code


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
    lexer = SlyLexer()
    parser = SlyParser()
    result = parser.parse(lexer.tokenize(data))
    result.eval()
    root = parser.vars[ROOT_NODE]
    print(f'\nPart I: {root.eval()}')

    #
    # Part II
    #

    # Прямые зависимости root
    left = root.expr.left
    right = root.expr.right

    # Поиск и вычисление пути от root до нужной переменной
    visitor = Visitor(ROOT_NODE, TARGET_NODE)
    traverse(root, visitor)

    # Вычисление ветки, в которой находится TARGET_NODE='humn'
    top_node = visitor.path[0]
    if top_node.name == left.name:
        initial = f'{top_node.name}: {right.eval()}'
    else:
        initial = f'{top_node.name}: {left.eval()}'

    print('\n', root)
    print(f' {"*" if top_node.name == left.name else " "}', left.name, left.eval())
    print(f' {"*" if top_node.name == right.name else " "}', right.name, right.eval())

    # Генерация кода для обратных выяислений humn
    path = [i.name for i in visitor.path]
    code = generate_code(parser.vars, path, debug=False)

    # Парсинг и вычисление сгенерированных обратных выражений
    lexer = SlyLexer()
    pars = SlyParser()
    result = pars.parse(lexer.tokenize(code))

    print('\nPart II:', pars.vars[TARGET_NODE].eval())
