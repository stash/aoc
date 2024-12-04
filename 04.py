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


# This isn't needed for part 2 since the search is always in bounds, but left here for reference:
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


def search_one(m, at: Vec2D) -> int:
    try:
        if m[at.y][at.x] == "A":  # fast: center first
            corners = "".join(
                [  # clockwise:
                    m[at.y - 1][at.x - 1],  # up left
                    m[at.y - 1][at.x + 1],  # up right
                    m[at.y + 1][at.x + 1],  # down right
                    m[at.y + 1][at.x - 1],  # down left
                ]
            )
            return (
                1  # cycle pattern clockwise:
                if corners == "MMSS"
                or corners == "SMMS"
                or corners == "SSMM"
                or corners == "MSSM"
                else 0
            )
        else:
            return 0
    except IndexError:
        return 0


def search(m):
    sum = 0
    for x in range(1, len(m[0]) - 1):
        for y in range(1, len(m) - 1):
            center = Vec2D(x, y)
            sum += search_one(m, center)
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
