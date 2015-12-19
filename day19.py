def parse(lines):
    rs = []
    for l in lines:
        rs.append(l.split(" => "))
    return rs

def replace(molecule, rep):
    a, b = rep
    parts = molecule.split(a)
    for i in range(len(parts) - 1):
        newm = parts[0]
        for j, p in enumerate(parts[1:]):
            if j == i:
                newm += b
            else:
                newm += a
            newm += p
        yield newm

def search(replacements, start):
    queue = [(start, 0)]
    seen = set()

    while queue:
        mol, steps = queue.pop()

        if mol == "e":
            return steps

        for r in replacements:
            for s in replace(mol, r):
                if s in seen:
                    continue
                seen.add(s)
                queue.append((s, steps + 1))
        queue.sort(key=lambda x: (len(x[0]), x[1], x[0]), reverse=True)

with open("day19.in") as fin:
    contents = fin.read().strip().split("\n")

medecine = contents.pop()
# pop blank line
contents.pop()
replacements = parse(contents)

molecules = set()
for r in replacements:
    for s in replace(medecine, r):
        molecules.add(s)

print(len(molecules))

# Reverse the replacements and work backwords from the large molecule to e
replacements = [(b, a) for (a, b) in replacements]
print(search(replacements, medecine))
