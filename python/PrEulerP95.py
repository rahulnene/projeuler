from time import perf_counter
# from primefactors import factorize
from math import sqrt, factorial, log10, floor

t0 = perf_counter()


def factorSum(n):
    factors = {1}
    for i in range(2, int(sqrt(n)) + 1):
        if n % i == 0:
            factors.add(i)
            factors.add(n // i)
    return sum(factors)


def generateChain(n):
    chain = []
    while n not in chain:
        if n < 1000000:
            chain.append(n)
            n = factorSum(n)
        else:
            return 0
    # return chain
    if chain[0] == n:
        return len(chain)
    else:
        return 1


max = 0
for i in range(2**9, 1000000, 2**9):
    temp = generateChain(i)
    if temp > max:
        max = temp
        print(i, max)

print(max)

tf = perf_counter() - t0
print(tf, "seconds")
