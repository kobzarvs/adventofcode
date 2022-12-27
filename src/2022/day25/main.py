from functools import lru_cache


def load_data(filename):
    with open(filename, 'r') as f:
        for line in f:
            yield line.strip()


SNAFU = {'0': 0, '1': 1, '2': 2, '=': -2, '-': -1}
SNAFU_KEYS = ['0', '1', '2', '=', '-']


@lru_cache
def radix_5_to_10(snafu):
    result = 0
    for c in snafu:
        result = result * 5 + SNAFU[c]
    return int(result)


@lru_cache
def radix_10_to_5(n):
    if n == 0:
        return '0'
    result = ''
    while n > 0:
        n, r = divmod(n, 5)
        result = SNAFU_KEYS[r] + result
        if r > 2:
            # if snafu == '-' or '=' then overflow
            n = n + 1
    return result


if __name__ == '__main__':
    short = False

    if short:
        filename = 'input.short.txt'
    else:
        filename = 'input.txt'

    part_1_sum = 0
    for line in load_data(filename):
        part_1_sum += radix_5_to_10(line)

    # part 1
    print('radix 10 sum:', part_1_sum)
    part_1 = radix_10_to_5(part_1_sum)
    print('SNAFU:', part_1)  # 2=020-===0-1===2=020
