def is_incr(x,y):
    return y>x and y-x<=3
def is_decr(x,y):
    return is_incr(y,x)

def classify(x,y):
    if y>x and y-x<=3:
        return "inc"
    elif x>y and x-y<=3:
        return "dec"
    else:
        return "crap"

def count_eq(b, cls):
    count = 0
    for x in b:
        count += 1 if x == cls else 0
    return count

tests = """
    7 6 4 2 1: Safe without removing any level.
    1 2 7 8 9: Unsafe regardless of which level is removed.
    9 7 6 2 1: Unsafe regardless of which level is removed.
    1 3 2 4 5: Safe by removing the second level, 3.
    8 6 4 4 1: Safe by removing the third level, 4.
    1 3 6 7 9: Safe without removing any level.
"""

chal = """
"""

def combos(a):
    return [
        a[:i] + a[i+1:] for i in range(len(a)+1)
    ]

def comp_one_list(a):
    print(repr(a))
    if a[0] == a[1]:
        return False
    elif a[0] > a[1]:
        return all([is_decr(a[x],a[x+1]) for x in range(len(a)-1)])
    else:
        return all([is_incr(a[x],a[x+1]) for x in range(len(a)-1)])

def comp_list(b):
    b = b.strip()
    if b == "":
        return False
    b = b.split(":")[0]
    print(f"try {b!r}")
    b = [int(x) for x in b.strip().split(" ")]
    n = len(b)-1
    return any(comp_one_list(a) for a in combos(b))

n_safe = 0
for i in chal.split("\n"):
    if comp_list(i):
        n_safe += 1

print(n_safe)
