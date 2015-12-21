import re
import itertools

def parse(lines):
    diffs = {}
    for l in lines:
        m = re.match("(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)", l)
        score = int(m.group(3))
        if m.group(2) == "lose":
            score = -score
        diffs[m.group(1)][m.group(4)] = diffs.setdefault(m.group(1), {}).get(m.group(4), 0) + score
        diffs[m.group(4)][m.group(1)] = diffs.setdefault(m.group(4), {}).get(m.group(1), 0) + score
    return diffs

def seat(diffs):
    names = list(diffs.keys())
    max_score = 0

    for p in itertools.permutations(names):
        s = 0
        for a, b in zip(p, p[1:] + (p[0],)):
            s += diffs[a][b]
        max_score = max(s, max_score)
    return max_score

with open("day13.in") as fin:
    contents = fin.read().strip().split("\n")

happiness = parse(contents)
change = seat(happiness)
print(change)

names = list(happiness.keys())
happiness["me"] = {}
for n in names:
    happiness["me"][n] = 0
    happiness[n]["me"] = 0
change = seat(happiness)
print(change)
