from time import clock as time
start = time()
triangleNums = []
alphabet = ['0','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z']
for i in range(1,27):
    triangleNums.append(0.5 * i * (i+1))

dataSum = []

f_open = open('p42.txt','r')
data = f_open.read()
# data = data.upper()
data = data.split(',')

for item in data:
    # print(item)
    dataSum.append(0)
    for letter in item:
        if letter in alphabet:
            dataSum[data.index(item)] += alphabet.index(letter)

ans = 0
for item in dataSum:
    if item in triangleNums:
        ans += 1

print(ans,"in",(time()-start)*1000,"ms")