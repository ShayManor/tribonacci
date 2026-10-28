# import functools

map = {}
# @functools.cache
def fib(n):
    if n < 3:
        if n == 0:
            return 0
        if n == 1:
            return 0
        if n == 2:
            return 1
    if n in map:
        return map[n]
    return fib(n - 1) + fib(n - 2) + fib(n - 3)


i = 1
while True:
    res = fib(i)
    print(res, flush=True)
    map[i] = res
    i += 1