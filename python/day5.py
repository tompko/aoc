import re

def nice1(string):
    if sum([string.count(v) for v in 'aeiou']) < 3:
        return False
    if re.search("ab|cd|pq|xy", string) is not None:
        return False
    if re.search("(?P<r>.)(?P=r)", string):
        return True
    return False

def nice2(string):
    if re.search("(?P<r>..).*(?P=r)", string) is None:
        return False
    if re.search("(?P<r>.).(?P=r)", string) is None:
        return False
    return True

count1 = 0
count2 = 0
with open("day5.in") as fin:
    for line in fin:
        line = line.strip()

        if nice1(line):
            count1 += 1
        if nice2(line):
            count2 += 1
print(count1)
print(count2)
