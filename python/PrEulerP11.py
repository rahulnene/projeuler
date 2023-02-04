from time import perf_counter
t0 = perf_counter()

import pandas as pd
import numpy as np


ary = np.genfromtxt("fileio.txt", dtype=None)
greatestOut = 0

def prodHor(row):
    greatest = 1
    for j in range(0, 4):
        greatest *= ary[row][j]
    for i in range(0,16):
        if ary[row][i] < ary[row][i+4]:
            greatest = 1
            for j in range(i+1, i+5):
                greatest *= ary[row][j]
    return greatest

def prodVert(column):
    greatest = 1
    for j in range(0, 4):
        greatest *= ary[j][column]
    for i in range(0,15):
        if ary[column][i] < ary[column][i+4]:
            greatest = 1
            for j in range(i+1, i+5):
                greatest *= ary[column][j]
    return greatest



print(prodHor(1))

t1 = perf_counter()
print(t1-t0)
