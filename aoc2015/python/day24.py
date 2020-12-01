from functools import reduce
from itertools import combinations
from operator import mul

weights = []
with open("day24.in") as fin:
    for line in fin:
        weights.append(int(line.strip()))

def day24(num_groups):
    group_sum = sum(weights) // num_groups
    for i in range(len(weights)):
        qes = [reduce(mul, c) for c in combinations(weights, i)
                        if sum(c) == group_sum]
        if qes:
            return min(qes)

print(day24(3))
print(day24(4))
