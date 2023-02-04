from timeit import default_timer
start = default_timer()
long = 0
dList = []
for j in range(1,999):
    if (j%2)&(j%5):dList.append(j)
for d in dList:
    for i in range(1,d):
        if not(10**i - 1)%d:
            if (i > long):long = i
            break
print(long,"at 1/",d,"in",(default_timer()-start)*1000,"ms")