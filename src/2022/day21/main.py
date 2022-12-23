from aoc.ast import Identifier
from aoc.gen import AstGenerator, CodeGenerator
from aoc.parser import AocLexer, AocParser

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
    program = parser.parse(lexer.tokenize(data))

    root = program.vars[ROOT_NODE]
    print(f'\nPart I: {root.eval()}')

    # Ветки от root
    left = root.expr.left
    right = root.expr.right

    #
    # Part II via AST
    #
    print('\n--- Part II via AST ---')

    # Поиск пути от root до humn
    # генерация AST для обратных выражений
    ast_visitor = AstGenerator(ROOT_NODE, TARGET_NODE)
    ast = ast_visitor.prog
    ast.traverse(root, ast_visitor)

    # Вычисление ветки, в которой находится TARGET_NODE='humn'
    top_node = ast_visitor.path[0]
    if top_node.name == left.name:
        initial = right
    else:
        initial = left

    # Инициализация переменной-ветки, в которой находится humn
    ast.insert(Identifier(top_node.name, initial.eval()))
    for s in ast.statements:
        print(f'{s} => {s.eval()}')

    print('\nPart II (via AST builder):', ast.vars[TARGET_NODE], ast.vars[TARGET_NODE].eval())

    #
    # Part II via text code generation
    #
    print('\n--- Part II via text code generation ---')

    # Поиск пути от root до humn
    # генерация кода для обратных выражений
    code_visitor = CodeGenerator(ROOT_NODE, TARGET_NODE)
    ast.traverse(root, code_visitor)

    # Вычисление ветки, в которой находится TARGET_NODE='humn'
    top_node = code_visitor.path[0]
    if top_node.name == left.name:
        initial = right
    else:
        initial = left

    # Инициализация переменной-ветки, в которой находится humn
    code_visitor.code.insert(0, f'{top_node.name}: {initial.eval()}')
    code = '\n'.join(code_visitor.code)
    print(code)

    # Парсинг и вычисление сгенерированных обратных выражений
    parser = AocParser()
    result = parser.parse(lexer.tokenize(code))

    print('\nPart II (via parse code):', result.vars[TARGET_NODE].eval())
