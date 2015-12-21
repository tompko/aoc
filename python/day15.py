import itertools
import re

def prod(xs):
    p = 1
    for x in xs:
        p *= x
    return p

def parse(lines):
    ing = []
    for l in lines:
        print(l)
        m = re.match("\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)", l)
        ing.append(tuple(int(x) for x in m.groups()))
    return ing

def mix(ing, calories=None):
    max_score = 0
    for mix in itertools.product(range(100), repeat=len(ing)-1):
        if sum(mix) > 100:
            continue
        mix = list(mix)
        mix.append(100 - sum(mix))
        score = [0 for _ in range(5)]
        for x, q in zip(ing, mix):
            for i, j in enumerate(x):
                score[i] += j*q
        if calories is not None and score[4] != calories:
            continue
        max_score = max(max_score, prod([max(0, s) for s in score[:-1]]))
    return max_score

with open("day15.in") as fin:
    contents = fin.read().strip().split("\n")

ingredients = parse(contents)

score = mix(ingredients)
print(score)
score = mix(ingredients, 500)
print(score)
