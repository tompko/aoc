import re

def parse(lines):
    aunts = []
    for l in lines:
        m = re.match("Sue (\d+): (.*)", l)
        id = int(m.group(1))
        props = m.group(2).split(",")
        props = [p.strip().split(": ") for p in props]
        props = {p[0]:int(p[1]) for p in props}
        aunts.append((id, props))
    return aunts

def match(t, ss):
    for id, ps in ss:
        bad = False
        for k, v in t.items():
            if ps.get(k, v) != v:
                bad = True
                break
        if not bad:
            return id
    return -1

def match2(t, ss):
    for id, ps in ss:
        bad = False
        for k, v in t.items():
            if k in ("cats", "trees"):
                if ps.get(k, v+1) <= v:
                    bad = True
                    break
            elif k in ("pomeranians", "goldfish"):
                if ps.get(k, v-1) >= v:
                    bad = True
                    break
            else:
                if ps.get(k, v) != v:
                    bad = True
                    break
        if not bad:
            return id
    return -1

with open("day16.in") as fin:
    contents = fin.read().strip().split("\n")

sues = parse(contents)
target = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1,
}

winner = match(target, sues)
print(winner)
winner = match2(target, sues)
print(winner)
