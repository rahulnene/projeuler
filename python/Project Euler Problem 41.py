from math import sqrt,ceil,factorial
import itertools
from time import clock as time

start = time()
def checkPrime(n):
    for check in range(2,ceil(sqrt(n))):
        if n%check:
            return 1
    return 0
permutatons = []
def permute():
    hmm = list(itertools.permutations([1,2,3,4,5,6,7]))
    temp = 0
    for i in range(0,5040):
        for j in range(0,7):
            temp += hmm[i][j] *(10**j)
        permutatons.append(temp)
        temp = 0
permute()

max = 0
for item in permutatons:
    if checkPrime(item):
        if item > max: max = item

print(max,"in",(time()-start)*1000,"ms")