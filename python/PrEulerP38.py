from time import perf_counter
from math import log10
from math import ceil

t0 = perf_counter()


def dig(x):
    log = log10(x)
    if log % 1 == 0:
        return log
    else:
        return ceil(log)


def check(k):
    out = 0
    digLeft = 9
    mult = 1
    while digLeft > 0:
        kmult = k * mult
        next = digLeft - dig(kmult)
        out += kmult * pow(10, next)
        digLeft = next
        mult += 1
    if out % 1 == 0:
        if sorted(str(out)) == ['1', '2', '3', '4', '5', '6', '7', '8', '9']:
            return out

    return 0


greatest = 0

for i in range(1, 100000):
    if log10(i) % 1 != 0:
        temp = check(i)
        if temp > greatest:
            greatest = temp
            greI = i

print(greatest)

t1 = perf_counter()
print(t1 - t0, "seconds")
