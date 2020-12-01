import itertools

weapons = [
    ("Dagger", 8, 4, 0),
    ("Shortsword", 10, 5, 0),
    ("Warhammer", 25, 6, 0),
    ("Longsword", 40, 7, 0),
    ("Greataxe", 74, 8, 0),
]

armor = [
    ("None", 0, 0, 0),
    ("Leather", 13, 0, 1),
    ("Chainmail", 31, 0, 2),
    ("Splintmail", 53, 0, 3),
    ("Bandedmail", 75, 0, 4),
    ("Platemail", 102, 0, 5),
]

rings = [
    ("None", 0, 0, 0),
    ("None", 0, 0, 0),
    ("Damage +1", 25, 1, 0),
    ("Damage +2", 50, 2, 0),
    ("Damage +3", 100, 3, 0),
    ("Defense +1", 20, 0, 1),
    ("Defense +2", 40, 0, 2),
    ("Defense +3", 80, 0, 3),
]

bhp = 100
barmor = 2
bdamage = 8

php = 100

def simulate(pdamage, parmor):
    h = php
    i = bhp

    while True:
        i -= max(1, pdamage - barmor)
        if i <= 0:
            return True
        h -= max(1, bdamage - parmor)
        if h <= 0:
            return False


best_win = 10**100
worst_lose = 0

for w in weapons:
    for a in armor:
        for r, s in itertools.combinations(rings, 2):
            cost = w[1] + a[1] + r[1] + s[1]
            pdam = w[2] + a[2] + r[2] + s[2]
            parm = w[3] + a[3] + r[3] + s[3]
            if simulate(pdam, parm):
                best_win = min(best_win, cost)
            else:
                worst_lose = max(worst_lose, cost)

print(best_win)
print(worst_lose)
