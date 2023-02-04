from time import clock as time
from math import ceil,sqrt

start = time()
max = [0,0]
count = 0
for perimeter in range(500,1000):
    if perimeter%2 == 0:
        count = 0
        for a in range(2,ceil(perimeter/2)):
           for b in range(a,ceil(perimeter/2)):
               if not sqrt(a*a + b*b)%1:
                    c = sqrt(a*a + b*b)
                    if a+b+c == perimeter:
                      count += 1
                    # print (a,"^2 + ", b , " ^2 = ",sqrt(a*a + b*b),"^2", "perimeter = ",perimeter,a+b+c)
    if count > max[0]:
        max[0] = count
        max[1] = perimeter
        print(count,perimeter)

print(time()-start)