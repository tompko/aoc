import itertools

def look_and_say(seq):
    return "".join([str(len(list(g))) + k for k, g in itertools.groupby(seq)])

input = "1113222113"

for i in range(50):
    input = look_and_say(input)
    if i == 39:
        print(len(input))
print(len(input))

