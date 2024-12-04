from dataclasses import dataclass

test = """
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"""


@dataclass
class Vec2D:
    x: int
    y: int


class Nowrap:
    def __init__(self, inner):
        self.inner = inner

    def __getitem__(self, key):
        if key < 0 or key >= len(self.inner):
            raise IndexError
        return self.inner[key]

    def __len__(self):
        return len(self.inner)

    def __repr__(self):
        return repr(self.inner)


def parse(input):
    rows = [Nowrap(row) for row in input.strip().split("\n")]
    return Nowrap(rows)


def search_one(m, at: Vec2D, d: Vec2D) -> int:
    try:
        if (
            m[at.y][at.x] == "X"
            and m[at.y + d.y][at.x + d.x] == "M"
            and m[at.y + 2 * d.y][at.x + 2 * d.x] == "A"
            and m[at.y + 3 * d.y][at.x + 3 * d.x] == "S"
        ):
            # print(f"{at!r} {step!r}")
            return 1
        else:
            return 0
    except IndexError:
        return 0


directions = [
    Vec2D(1, 0),
    Vec2D(1, 1),
    Vec2D(0, 1),
    Vec2D(-1, 1),
    Vec2D(-1, 0),
    Vec2D(-1, -1),
    Vec2D(0, -1),
    Vec2D(1, -1),
]


def search(m):
    sum = 0
    for x in range(len(m[0])):
        for y in range(len(m)):
            at = Vec2D(x, y)
            # print(f"{at!r}:")
            for d in directions:
                sum += search_one(m, at, d)
    return sum


# print(repr(parse(test)))
try:
    parse(test)[-1]
    print("not ok")
except IndexError:
    print("ok")

try:
    parse(test)[0][-1]
    print("not ok")
except IndexError:
    print("ok")

print(search(parse(test)))

with open("inputs/04.txt") as f:
    print(search(parse(f.read())))
