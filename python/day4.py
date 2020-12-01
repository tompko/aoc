import hashlib

def brute_search(prefix, target):
    i = 0
    prefix = prefix.encode()
    while True:
        m = hashlib.md5()
        m.update(prefix)
        m.update(str(i).encode())
        if m.hexdigest().startswith(target):
            return i
        i += 1

print(brute_search("bgvyzdsv", "00000"))
print(brute_search("bgvyzdsv", "000000"))
