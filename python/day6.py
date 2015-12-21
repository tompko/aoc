import re

def light(inst):
    l = [[0 for i in range(1000)] for j in range(1000)]
    n = [[0 for i in range(1000)] for j in range(1000)]
    for i in inst:
        on = re.match("turn on (\d+),(\d+) through (\d+),(\d+)", i)
        off = re.match("turn off (\d+),(\d+) through (\d+),(\d+)", i)
        tog = re.match("toggle (\d+),(\d+) through (\d+),(\d+)", i)

        if on is not None:
            for x in range(int(on.group(1)), int(on.group(3)) + 1):
                for y in range(int(on.group(2)), int(on.group(4)) + 1):
                    l[x][y] = 1
                    n[x][y] += 1
        elif off is not None:
            for x in range(int(off.group(1)), int(off.group(3)) + 1):
                for y in range(int(off.group(2)), int(off.group(4)) + 1):
                    l[x][y] = 0
                    n[x][y] = max(0, n[x][y] - 1)
        elif tog is not None:
            for x in range(int(tog.group(1)), int(tog.group(3)) + 1):
                for y in range(int(tog.group(2)), int(tog.group(4)) + 1):
                    l[x][y] = 1 - l[x][y]
                    n[x][y] += 2

    return sum([sum(x) for x in l]), sum([sum(x) for x in n])

with open("day6.in") as fin:
    instructions = [l.strip() for l in fin]

print(light(instructions))
