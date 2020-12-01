import re

def parse(rs):
    m = re.match("(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.", rs)
    name = m.group(1)
    speed = int(m.group(2))
    time = int(m.group(3))
    rest = int(m.group(4))
    return (name, speed, time, rest)

def simulate(reindeers, limit):
    states = []
    for r in reindeers:
        # name, state, time left in state, distance, points
        states.append([r[0], "RUNNING", r[2], 0, 0])
    reindeers = {r[0]: r[1:] for r in reindeers}

    for _ in range(limit):
        for s in states:
            if s[1] == "RUNNING":
                s[3] += reindeers[s[0]][0]
                s[2] -= 1
                if s[2] == 0:
                    s[2] = reindeers[s[0]][2]
                    s[1] = "RESTING"
            else:
                s[2] -= 1
                if s[2] == 0:
                    s[2] = reindeers[s[0]][1]
                    s[1] = "RUNNING"
        m = max(s[3] for s in states)
        for s in states:
            if s[3] == m:
                s[4] += 1

    states.sort(key=lambda x: x[3], reverse=True)
    distance = (states[0][0], states[0][3])
    states.sort(key=lambda x: x[4], reverse=True)
    points = (states[0][0], states[0][4])
    return distance, points

with open("day14.in") as fin:
    contents = fin.read().strip().split("\n")

reindeers = [parse(c) for c in contents]
distance, points= simulate(reindeers, 2503)
print(distance)
print(points)
