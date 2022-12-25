from functools import lru_cache


@lru_cache(maxsize=None)
def fib(n):
    match n:
        case 0 | 1:
            return n
        case _:
            return fib(n - 1) + fib(n - 2)


print(fib(10))
