import time
ans = 0

start = time.clock()


for hunthousands in range(0,5):
    for tenthousands in range(0, 10):
        for thousands in range(0, 10):
            for hundreds in range(0, 10):
                for tens in range(0, 10):
                    for ones in range(0, 10):
                        powerVal = ones ** 4 + tens ** 4 + hundreds ** 4 + thousands ** 4 + tenthousands ** 4 + hunthousands**4
                        actVal = hunthousands*100000 + tenthousands * 10000 + thousands * 1000 + hundreds * 100 + tens * 10 + ones
                        if (powerVal == actVal):
                            ans += actVal
                            print(actVal)

ans -= 1
print (ans)
print(time.clock() - start)