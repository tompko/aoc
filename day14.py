import re

def parse(rs):
    m = re.match("(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.", rs)
    name = m.group(1)
    speed = int(m.group(2))
    time = int(m.group(3))
    rest = int(m.group(4))
    return (name, speed, time, rest)

def run(p, limit):
    dist = 0
    name, speed, time, rest = p
    while limit > 0:
        t = min(time, limit)
        dist += speed * t
        limit -= t
        limit -= rest
    return dist

def simulate1(reindeers, limit):
    max_dist = 0
    winner = None
    for r in reindeers:
        d = run(r, limit)
        if d > max_dist:
            max_dist = d
            winner = r[0]
    return winner, max_dist

def simulate2(reindeers, limit):
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

    states.sort(key=lambda x: x[4], reverse=True)
    return (states[0][0], states[0][4])

with open("day14.in") as fin:
    contents = fin.read().strip().split("\n")

reindeers = [parse(c) for c in contents]
winner = simulate1(reindeers, 2503)
print(winner)
winner = simulate2(reindeers, 2503)
print(winner)
