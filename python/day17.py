def count(target, capacities):
    capacities.sort()
    c = {}
    queue = [(target, [], capacities)]

    while queue:
        t, cs, ls = queue.pop()
        if t == 0:
            c[len(cs)] = c.get(len(cs), 0) + 1
            continue
        if t < 0 or sum(ls) < t:
            continue

        l = ls.pop(0)
        queue.append((t- l, cs + [l], ls[:]))
        queue.append((t, cs[:], ls[:]))

    return c

with open("day17.in") as fin:
    containers = []
    for l in fin:
        containers.append(int(l.strip()))

packings = count(150, containers)
min_num = 0
min_p = 150
total = 0
for k, v in packings.items():
    if k < min_p:
        min_p = k
        min_num = v
    total += v
print(total)
print(min_num)
