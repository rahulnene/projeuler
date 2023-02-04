from time import perf_counter

t0 = perf_counter()


def isEven(n):
    if not (n % 2):
        return True
    else:
        return False

#
# def Collatz(n,count):
#     if n==1:
#         return count+1
#     if isEven(n):
#         n = n/2
#     else:
#         n = 3*n + 1
#
#     return Collatz(n,count+1)
#
# greatest = 0
# ans = 0
#
# for i in range(1000000,):
#     steps = Collatz(i,0)
#     if steps > greatest:
#         greatest = steps
#         ans = i
#
# print(ans,greatest)

def Collatz(n, count):
    if n == 1: return count
    if isEven(n):
        n = n / 2
        count += 1
    else:
        n = 1.5 * n + 0.5
        count += 2
    return Collatz(n, count)

max = 1000001
greatest = 1
ans = 0

for i in range(100000,max):
    steps = Collatz(i,1)
    if steps > greatest:
        greatest = steps
        ans = i

print(ans,greatest)

t1 = perf_counter()
print(t1 - t0)
