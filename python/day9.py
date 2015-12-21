import itertools

def paths(distances):
    cities = list(distances.keys())
    min_dist = None
    max_dist = None

    for perm in itertools.permutations(cities):
        dist = 0
        for a, b in zip(perm, perm[1:]):
            dist += distances[a][b]

        if min_dist is None:
            min_dist = dist
            max_dist = dist
        min_dist = min(dist, min_dist)
        max_dist = max(dist, max_dist)

    return min_dist, max_dist

dists = {}
with open("day9.in") as fin:
    for line in fin:
        line = line.strip().split()
        dists.setdefault(line[0], {})[line[2]] = int(line[4])
        dists.setdefault(line[2], {})[line[0]] = int(line[4])

short, long = paths(dists)
print(short)
print(long)
