from time import perf_counter

# from primefactors import factorize
from math import sqrt, cos, log, sin, copysign, pi, floor, e
from sympy.ntheory import totient
import numba as nb
import numpy as np

t0 = perf_counter()
k = 100000000


@nb.njit("i8(i4[:])", locals=dict(
    n=nb.int32, s=nb.int64, i=nb.int32,
    j=nb.int32, q=nb.int32, f=nb.int32))
def summarum(phi):
    s = 0
    phi[1] = 1
    i = 2
    while i < k:
        if phi[i] == 0:
            phi[i] = i - 1
            j = 2
            while j * i < k:
                if phi[j] != 0:
                    q = j
                    f = i - 1
                    while q % i == 0:
                        f *= i
                        q //= i
                    phi[i * j] = f * phi[q]
                j += 1
        s += phi[i]
        i += 1
    # print(phi)
    return s


def run(k):
    total = 6
    temp = k * (k + 1) / 2
    temp -= summarum(phi=np.zeros(k, np.int32))
    return int(total * temp - 6 - 2.4*k)


print(run(k))

tf = perf_counter() - t0
print(tf, "seconds")
