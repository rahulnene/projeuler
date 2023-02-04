import time
import math
start = time.clock()
sum = 0
for i in range (1,10001):
    sum += i**i

print (sum)
print(math.log10(sum))
print (time.clock() - start)
