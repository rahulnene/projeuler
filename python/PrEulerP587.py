from time import perf_counter
from math import sqrt, asin, pi

t0 = perf_counter()

L = 1 - (pi / 4)
end = 0.0001


def areaPercent(n):
    x0 = n * (n + 1 - sqrt(2 * n)) / (n ** 2 + 1)
    area = (x0) / (2 * n)
    area += (1 - x0 - asin(1 - x0)) / 2
    return 100 * (area / L)


found = False
n = 1

while True:
    if areaPercent(n) < end:
        break
    n += 1

print(n)

tf = perf_counter() - t0
print(tf, "seconds")
