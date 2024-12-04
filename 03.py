import re


def sum_expr(s):
    sum = 0
    while len(s) > 0:
        try:
            (valid, s) = s.split("don't()", 1)
            try:
                (_,s) = s.split("do()", 1)
            except ValueError:
                s = "" # no trailing do block
        except ValueError:
            (valid, s) = (s,"")
        for m in re.findall(r"mul\((\d{1,3}),(\d{1,3})\)", valid):
            sum += int(m[0]) * int(m[1])
    return sum

test = "xmul(2,4)&mul[3,7]!^don't()\n_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
print(sum_expr(f"{test}".replace("\n"," ")))

with open("inputs/03p2.txt") as f:
    chal = f.read().replace("\n"," ")
print(sum_expr(f"{chal}"))