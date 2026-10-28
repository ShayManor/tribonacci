def fib(n):
    if n < 3:
        if n == 0:
            return 0
        if n == 1:
            return 0
        if n == 2:
            return 1
    return fib(n - 1) + fib(n - 2) + fib(n - 3)

i = 1
while True:
    print(fib(i), flush=True)
    i += 1