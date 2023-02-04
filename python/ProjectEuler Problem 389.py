from random import *
import statistics

T = 0
C = 0
O = 0
D = 0

IList = []
ISum = 0

for i in range(0, 100):
    T = randint(1, 4)
    C = 0
    O = 0
    D = 0
    ISum = 0
    print("T is: ",T)
    for j in range(0, T):
        C += randint(1, 6)
    print("C is: ",C)
    for k in range(0, C):
        O += randint(1, 8)
    for l in range(0, O):
        D += randint(1, 12)
    for m in range(0, D):
        ISum += randint(1, 20)
    IList.append(ISum)
    # print(ISum)
    ISum = 0

print(len(IList))
# print(IList)
print("Variance is: ",statistics.variance(IList))

