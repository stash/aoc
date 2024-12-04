import re
import io
import os


def sum_expr(s):
    sum = 0
    for m in re.findall(r"mul\((\d{1,3}),(\d{1,3})\)", s):
        sum += int(m[0]) * int(m[1])
    return sum

test = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
print(sum_expr(test))

with open("inputs/03p1.txt") as f:
    chal = f.read().replace("\n"," ")
print(sum_expr(chal))