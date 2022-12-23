class Number:
    def __init__(self, value):
        self.value = value

    def eval(self, module=None):
        return self.value

    def __repr__(self):
        return f'Number({self.value})'
