def escape(string):
    escaped = ''
    for s in string:
        if s == "\\":
            escaped += "\\\\"
        elif s == '"':
            escaped += "\\\""
        else:
            escaped += s
    return escaped

with open("day8.in") as fin:
    contents = [l.strip() for l in fin]

count1 = 0
count2 = 0
for c in contents:
    count1 += len(c)
    count1 -= len(c.encode().decode("unicode-escape")) - 2
    count2 += len(escape(c)) + 2
    count2 -= len(c)
print(count1)
print(count2)
