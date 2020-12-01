presents = [0 for _ in range(1000000)]

for i in range(1, 1000000):
    for j in range(i, 1000000, i):
        presents[j] += i*10
    if presents[i] > 36000000:
        print(i)
        break

presents = [0 for _ in range(1000000)]

for i in range(1, 1000000):
    for j in range(i, min(1000000, 51*i), i):
        presents[j] += i*11
    if presents[i] > 36000000:
        print(i)
        break
