import re
import json

def add(obj):
    if isinstance(obj, int):
        return obj, obj
    if isinstance(obj, str):
        return 0, 0
    if isinstance(obj, list):
        n, r = 0, 0
        for x in obj:
            a = add(x)
            n += a[0]
            r += a[1]
        return n, r
    if isinstance(obj, dict):
        n, r = 0, 0
        for x in obj.values():
            a = add(x)
            n += a[0]
            r += a[1]
        if any([v == "red" for v in obj.values()]):
            return n, 0
        return n, r

with open("day12.in") as fin:
    contents = fin.read()

j = json.loads(contents)
one, two = add(j)
print(one)
print(two)
