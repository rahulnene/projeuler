from time import perf_counter

t0 = perf_counter()

pents = []
hex = []


def genHex():
    for n in range(0, 100000):
        temp = n
        temp *= 2 * n - 1
        hex.append(temp)


def checkPent(x):
    det = pow(24 * x + 1, 0.5)
    if (det + 1) % 6 == 0:
        return True
    return False


def find():
    genHex()
    for item in hex:
        if checkPent(item):
            print(item)
            if item > 40755: return 0


find()

t1 = perf_counter()
print(t1 - t0, "seconds")
