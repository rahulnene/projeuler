from time import perf_counter


t0 = perf_counter()
ans = 1
size = 1001
rang = (size+1)//2

for x in range (1,rang):
    temp = 0
    temp += x*x
    temp *= 4
    temp += x
    temp += 1
    temp *= 4
    ans += temp

print(ans)

t1 = perf_counter()
print(t1 - t0)
