from time import perf_counter
t0 = perf_counter()

from math import sqrt

def sum_div(number):
    divisors = [1]
    for i in range(2, int(sqrt(number))):
        if (number % i) == 0:
            divisors.append(i)
            divisors.append(number / i)
    return sum(divisors)

checked = []
ans = 0

for x in range(2, 10001):
    if x not in checked:
        y = sum_div(x)
        if y < 10000 and x != y:
            if x == sum_div(y):
                ans += (x + y)
                print(x, y)
        checked.append(x)
        checked.append(y)

print(ans)

t1 = perf_counter()
print(t1 - t0)
