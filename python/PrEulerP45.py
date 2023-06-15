import math
import time

def main():
    start = time.time()
    n = 1
    count = 0

    while count < 4:
        hex_num = n * (2 * n - 1)
        if is_pentagonal(hex_num):
            print(hex_num)
            count += 1
        n += 1

    print("Time elapsed:", time.time() - start)

def is_pentagonal(x):
    sqrt = math.sqrt(1 + 24 * x)
    pent = (sqrt + 1) / 6
    return pent.is_integer()

if __name__ == "__main__":
    main()
