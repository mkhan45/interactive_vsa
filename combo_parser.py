import ast

class NodeCounter(ast.NodeVisitor):
    def __init__(self):
        self.count = 0

    def visit(self, node):
        self.count += 1
        self.generic_visit(node)
    
    def count_nodes(node):
        counter = NodeCounter()
        counter.visit(node)
        return counter.count

class ComboParser(ast.NodeVisitor):
    def __init__(self):
        self.exprs = []

    def visit(self, node):
        size = NodeCounter.count_nodes(node)
        # self.exprs.append((node, size))
        self.exprs.append(node)
        self.generic_visit(node)

    def get_combos(node):
        parser = ComboParser()
        parser.visit(node)
        return parser.exprs

def is_expr(node):
    allowed_types = [
        ast.Expr, ast.BinOp, ast.Constant
    ]

    return type(node) in allowed_types

code = "1 * 2 + 3"
tree = ast.parse(code)
exprs = ComboParser.get_combos(tree)

# print([(ast.unparse(expr), size) for (expr, size) in exprs if is_expr(expr)])
# print([(ast.unparse(expr), size) for (expr, size) in exprs if is_expr(expr)])
