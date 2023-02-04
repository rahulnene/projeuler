from time import perf_counter

t0 = perf_counter()

with open('p022_names.txt') as f:
    names = f.read()

names = names.strip().split(',')

names = [x[1:-1] for x in names]

names.sort()

ans = 0

def nameScore(name):
    letters = list(name)
    letters = [ord(x) - 64 for x in letters]
    return sum(letters)


for name in names:
    ans += (nameScore(name) * (names.index(name) + 1))

print(ans)


t1 = perf_counter()
print(t1 - t0)
