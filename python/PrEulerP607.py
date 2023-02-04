from time import perf_counter
from math import sqrt
from sympy import *
from sympy.abc import theta

t0 = perf_counter()

n0 = 1
n1 = 10 / 9
n2 = 10 / 8
n3 = 10 / 7
n4 = 10 / 6
n5 = 10 / 5
n6 = 1

x1 = -25 + 50 / sqrt(2)
x2 = x1 + 10
x3 = x1 + 20
x4 = x1 + 30
x5 = x1 + 40
x6 = x1 + 50


def distance(p1, p2):
    deltax = p1[0] - p2[0]
    deltay = p1[1] - p2[1]
    dist = sqrt(deltax ** 2 + deltay ** 2)
    return dist


xArr = [0, x1, x2, x3, x4, x5, x6, 100 / sqrt(2)]
nArr = [n0, n1, n2, n3, n4, n5, n6]

sinArr = [sin(theta), 0, 0, 0, 0, 0, 0]
for i in range(0, 6):
    sinArr[i + 1] = sinArr[i] * (nArr[i] / nArr[i + 1])

slopes = [0, 0, 0, 0, 0, 0, 0]
for i in range(0, 7):
    value = sinArr[i]
    slopes[i] = (value / sqrt(1 - value ** 2))

yArr = [0, x1, x2, x3, x4, x5, x6, 100 / sqrt(2)]
for i in range(1, 7):
    yArr[i] = yArr[i - 1] + slopes[i - 1] * (xArr[i] - xArr[i - 1])


def properDistance():
    pDist = 0
    for i in range(0, 7):
        A = (xArr[i], yArr[i])
        B = (xArr[i + 1], yArr[i + 1])
        pDist += nArr[i] * distance(A, B)
    return pDist


distanceFunction = properDistance()
f = N(diff(distanceFunction, theta))
fprime = diff(f, theta)
g = theta - (f / fprime)

thetaN = 1

for i in range(1, 4):
    theta_next = g.subs(theta, thetaN)
    thetaN = N(theta_next)
    print(i, thetaN)


print(N(distanceFunction.subs(theta, thetaN)))

tf = perf_counter() - t0
print(tf, "seconds")
