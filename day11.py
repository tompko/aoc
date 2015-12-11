def valid_pass(password):
    alphabet = "abcdefghijklmnopqrstuvwxyz"
    triples = ["".join(aas) for aas in zip(alphabet, alphabet[1:], alphabet[2:])]
    pairs = ["".join(aas) for aas in zip(alphabet, alphabet)]

    forbidden = ["i", "o", "l"]

    for f in forbidden:
        if f in password:
            return False

    if not any([t in password for t in triples]):
        return False

    count = 0
    for p in pairs:
        if p in password:
            count += 1

    if count < 2:
        return False

    return True


def increment_pass(password):
    ps = [ord(p) - ord('a') for p in password]

    for i in range(len(ps) - 1, -1, -1):
        ps[i] += 1
        if ps[i] <= 25:
            break
        ps[i] -= 26

    # Skip past any forbidden letters
    forbidden = [ord(c) - ord('a') for c in "iol"]

    for i, p in enumerate(ps):
        if p in forbidden:
            ps[i] += 1
            for j in range(i+1, len(ps)):
                ps[j] = 0

    return "".join([chr(p + ord('a')) for p in ps])

def next_pass(password):
    while not valid_pass(password):
        password = increment_pass(password)
    return password

n = next_pass("cqjxjnds")
print(n)
print(next_pass(increment_pass(n)))
