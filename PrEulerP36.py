from time import perf_counter

t0 = perf_counter()


def binary(d):
    return int('{0:b}'.format(d))


def reverse(n):
    n = str(n)
    return int(n[::-1])


def isPalindrome(x):
    if x == reverse(x):
        return True

out = 0

for x in range(1, 1000000,2):
    if isPalindrome(x):
        y = binary(x)
        if isPalindrome(y):
            out += x

print(out)
t1 = perf_counter()
print(t1 - t0, "seconds")
