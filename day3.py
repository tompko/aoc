def houses(string):
    x, y = 0, 0
    houses = {(x, y)}
    for s in string:
        x += {'>': 1, '<': -1}.get(s, 0)
        y += {'^': 1, 'v': -1}.get(s, 0)
        houses.add((x, y))

    return houses

with open("day3.in") as fin:
    path = fin.read()

print(len(houses(path)))
print(len(houses(path[::2]).union(houses(path[1::2]))))
