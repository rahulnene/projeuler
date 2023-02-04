from time import perf_counter

t0 = perf_counter()

iter_limit = 50


def iterate(n):
    rev = str(n)
    rev = int(rev[::-1])
    return rev + n


def checkPalin(n):
    rev = str(n)
    rev = int(rev[::-1])
    if n == rev: return True
    return False


def lychrel(prev, iterations):
    i = iterations
    if i > iter_limit: return -1
    n = iterate(prev)
    i += 1
    if checkPalin(n):
        return i
    else:
        return lychrel(n, i)


ans = 0
for i in range(1, 10000):
    if lychrel(i, 0) == -1:
        ans += 1

print(ans)

t1 = perf_counter()
print(t1 - t0, "seconds")
