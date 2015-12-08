with open("day8.in") as fin:
    contents = [l.strip() for l in fin]

count = 0
for c in contents:
    count += len(c)
    count -= len(c.decode("string-escape")) - 2
print count
