from time import perf_counter
from math import log10

t0 = perf_counter()

fibb = [1,1]
index = 2
limit = 999
next = 2

while log10(next) < limit:
    fibb[0] = fibb[1]
    fibb[1] = next
    next = sum(fibb)
    index += 1

print(index,log10(fibb[1]))

t1 = perf_counter()
print(t1 - t0)
