import re

test = """
3   4
4   3
2   5
1   3
3   9
3   3
"""


def sum_dist(s):
    xl, yl = [], []
    for line in s.split("\n"):
        (x, y) = re.split(r"\s+", line)
        xl.append(int(x))
        yl.append(int(y))
    xl.sort()
    yl.sort()
    sum = 0
    for x, y in zip(xl, yl):
        sum += abs(x - y)
    return sum


print(sum_dist(test.strip()))

with open("inputs/01.txt") as f:
    chal = f.read()
    print(sum_dist(chal.strip()))
