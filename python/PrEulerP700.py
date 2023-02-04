from time import perf_counter
t0 = perf_counter()

a = 1504170715041707
mod = 4503599627370517
total = a
min = a
max = a

while True:
    if min == 1: break
    temp = min + max
    temp %= mod
    if temp > max: max = temp
    if temp < min:
        min = temp
        total += min

print(total)


t1 = perf_counter()
print(t1 - t0, "seconds")
