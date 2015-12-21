import itertools

def life_step(cells, size=100):
    new_state = set()
    diffs = [x for x in itertools.product((-1,0,1), repeat=2) if x != (0, 0)]

    for (x,y) in itertools.product(range(size), repeat=2):
        neighbours  = len({(i, j) for (i, j) in diffs if (x+i, y+j) in cells})

        if (x,y) in cells and neighbours in (2, 3):
            new_state.add((x,y))
        elif (x,y) not in cells and neighbours == 3:
            new_state.add((x,y))
    return new_state

with open("day18.in") as fin:
    layout = set()
    for y, line in enumerate(fin):
        for x, cell in enumerate(line.strip()):
            if cell == '#':
                layout.add((x,y))

lights = layout
for _ in range(100):
    lights = life_step(lights)
print(len(lights))

corners = {(0, 0), (99, 0), (0, 99), (99, 99)}
lights = layout.union(corners)
for _ in range(100):
    lights = life_step(lights).union(corners)
print(len(lights))

