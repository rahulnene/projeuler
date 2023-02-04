lines = [line.rstrip('\n') for line in open('C:/Users/rahul/Desktop/p102_triangles.txt','r')]
# print(lines)
answer = 0
pointX = 0
pointY =0


def checkInside(inString):
    inList = inString.split(',')
    x1 = float(inList[0])
    x2 = float(inList[2])
    x3 = float(inList[4])
    y1 = float(inList[1])
    y2 = float(inList[3])
    y3 = float(inList[5])

    alpha = ((y2 - y3) * (pointX - x3) + (x3 - x2) * (pointY - y3)) / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3))
    beta = ((y3 - y1) * (pointX - x3) + (x1 - x3) * (pointY - y3)) / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3))
    gamma = 1 - alpha - beta

    isAlpha = alpha > 0
    isBeta = beta > 0
    isGamma = gamma > 0

    if isAlpha & isBeta & isGamma:
        return True
    else:
        return False


for triangle in lines:
    if checkInside(triangle):
        answer += 1
    # print(triangle)

print(answer)
