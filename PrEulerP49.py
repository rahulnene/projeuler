from time import perf_counter

t0 = perf_counter()

limit = 10000


def genPrimes(n):
    sieve = [True] * (n // 2)
    for i in range(3, int(n ** 0.5) + 1, 2):
        if sieve[i // 2]:
            sieve[i * i // 2::i] = [False] * ((n - i * i - 1) // (2 * i) + 1)
    return [2] + [2 * i + 1 for i in range(1, n // 2) if sieve[i]]


primes = genPrimes(limit)
primes1 = primes[250:650]
primes2 = primes[500:900]
primes3 = primes[800:1229]

def checkPerm(a,b,c):
    if a == b or b == c or a == c:
        return False
    a = sorted(str(a))
    b = sorted(str(b))
    c = sorted(str(c))
    if a == b and b == c:
        return True

    return False

def run():
    for prime1 in primes1:
        for prime2 in primes2:
            for prime3 in primes3:
                if prime1 > prime2:
                    break
                if 2 * prime2 == prime1 + prime3:
                    if checkPerm(prime1, prime2, prime3):
                        print(prime1, prime2, prime3)
                        if prime1 > 2000: return 0

run()

t1 = perf_counter()
print(t1 - t0, "seconds")
