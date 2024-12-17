desired = [2, 4, 1, 3, 7, 5, 0, 3, 4, 1, 1, 5, 5, 5, 3, 0]
desired.reverse()


def check(a, d):
    b = a % 8
    b = b ^ 3
    c = a >> b
    # a = a >> 3
    b = b ^ c
    out = (b ^ 5) % 8
    return out == d


candidates = [0]
for d in desired:
    print(f"{d} with {candidates}:")
    found = []
    for candidate in candidates:
        for bits in range(0, 8):
            a = (candidate << 3) + bits
            if check(a, d):
                print(f"{bits} ~ {bin(a)}")
                found.append(a)
    candidates = found
    print("---")

print(candidates)
