import re

test = """
3   4
4   3
2   5
1   3
3   9
3   3
"""


def sum_sim(s):
    xl, yl = [], []
    for line in s.split("\n"):
        (x, y) = re.split(r"\s+", line)
        xl.append(int(x))
        yl.append(int(y))
    xl.sort()
    yl.sort()
    sum = 0
    for x in xl:
        sum += x * yl.count(x)
    return sum


print(sum_sim(test.strip()))

with open("inputs/01.txt") as f:
    chal = f.read()
    print(sum_sim(chal.strip()))
