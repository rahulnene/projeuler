from time import perf_counter
t0 = perf_counter()


def count(arr, size, n):
    if n==0: return 1
    if n<0 or ((size <= 0) and n >= 1): return 0
    return count(arr,size-1,n) + count(arr,size,n-arr[size-1])

coins = [1,2,5,10,20,50,100,200]
n = 200
size = 8
ans = count(coins,size,n)

print(ans)


t1 = perf_counter()
print(t1 - t0)
