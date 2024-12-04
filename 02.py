def is_incr(x,y):
    return y>x and y-x<=3
def is_decr(x,y):
    return is_incr(y,x)

tests = """
    7 6 4 2 1    
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
"""

chal = """
"""

def comp_list(a):
    a = a.strip()
    if a == "":
        return False
    a = [int(x) for x in a.strip().split(" ")]
    if a[0] == a[1]:
        return False
    elif a[0] > a[1]:
        return all([is_decr(a[x],a[x+1]) for x in range(len(a)-1)])
    else:
        return all([is_incr(a[x],a[x+1]) for x in range(len(a)-1)])

n_safe = 0
for i in chal.split("\n"):
    if comp_list(i):
        n_safe += 1

print(n_safe)
