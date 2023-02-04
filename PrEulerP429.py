from time import perf_counter
from math import floor

t0 = perf_counter()


def genPrimes(n):
    sieve = [True] * (n // 2)
    for i in range(3, int(n ** 0.5) + 1, 2):
        if sieve[i // 2]:
            sieve[i * i // 2::i] = [False] * ((n - i * i - 1) // (2 * i) + 1)
    return [2] + [2 * i + 1 for i in range(1, n // 2) if sieve[i]]


mod = 1000000009


def S(n):
    factors = factorizeFactorial(n)
    out = 1
    for prime in factors:
        count = factors[prime]
        out *= 1 + pow(prime, 2 * count, mod)
        out %= mod
    return out % mod


def factorizeFactorial(n):
    factors = {}
    primes = set(genPrimes(n))
    for p in primes:
        denom = p
        if 2 * p > n:
            factors[p] = 1
        else:
            temp = 0
            while denom <= n:
                temp += floor(n / denom)
                denom *= p
            factors[p] = temp
    return factors


print(S(10 ** 8))

tf = perf_counter()
print(tf - t0, "seconds")
