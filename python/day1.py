import itertools

with open("day1.in") as fin:
    bs = fin.readline()
    print(sum([1 if b == '(' else -1 for b in bs]))
    print(list(itertools.accumulate([1 if b == '(' else -1 for b in bs])).index(-1) + 1)

