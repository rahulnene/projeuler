import random
iterate = 0
summ = 0
totalIter = 1000


while iterate < totalIter:
    colors = [1,2,3,4,5,6,7]
    urn = []
    for i in range (1,8):
        for j in range (0,10):
            urn.append(i)

    for draw in range (1,21):
        num_to_select = 20  # set the number to select here.
        drawnBalls = random.sample(urn, num_to_select)

    #print(drawnBalls)
    drawnQ = [False, False, False, False, False, False, False]
    for item in drawnBalls:
        if item in colors:
            indexBall = colors.index(item)
            drawnQ[indexBall] = True
    totalColors = 0
    for entry in drawnQ:
        if entry:
            totalColors += 1

    #print(totalColors)
    iterate += 1
    summ += totalColors

print (summ/totalIter)