def process(line):
    sides = [int(x) for x in line.split("x")]
    sides.sort()

    paper = 2*sides[0]*sides[1] + 2*sides[1]*sides[2] + 2*sides[2]*sides[0]
    paper += sides[0]*sides[1]
    perim = sides[0]*2 + sides[1] * 2
    vol = sides[0] * sides[1] * sides[2]
    return paper, perim + vol

wrapping = 0
ribbon = 0
with open("day2.in") as fin:
    for line in fin:
        p, r = process(line)
        wrapping += p
        ribbon += r
print(wrapping)
print(ribbon)
