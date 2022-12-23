import numpy as np

lines = open("./src/input/day22.txt", "r").read().split("\n")

n_cols = max([len(x) for x in lines[:-2]])
n_rows = len(lines) - 2

jungle = []

for line in lines[:-2]:
    row = ["E" if c == " " else c for c in line]
    row += ["E"] * (n_cols - len(row))
    jungle.append(row)

jungle = np.array(jungle).T


def parse_instructions(instr):
    l_acc = []
    str_acc = ""
    for c in instr:
        if c.isdigit():
            str_acc += c
        else:
            l_acc += [int(str_acc), c]
            str_acc = ""
    l_acc += [int(str_acc)]
    return l_acc


instructions = parse_instructions(lines[-1])

dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]]
dir_dict = {"R": 1, "L": -1}

DEBUG = False


def list_add(l1, l2, mods=[n_cols, n_rows]):
    return [(x + y) % m for x, y, m in zip(l1, l2, mods)]


def address_overflow(cur_loc, dir):
    temp = cur_loc
    while jungle[*temp] == "E":
        temp = list_add(temp, dirs[dir])
        # if DEBUG:
        #     print(f"Step to {cur_loc}")
    if jungle[*temp] == "#":
        # if the overflow lands you at a wall, just don't move
        return cur_loc
    return temp


def move(cur_loc, dir, inst):
    if isinstance(inst, int):
        if DEBUG_SPECIAL:
            print(f"Current location: {cur_loc}, direction: {dir}, instruction: {inst}")
        dist = inst
        for i in range(dist):
            cur_loc = list_add(cur_loc, dirs[dir])
            if jungle[*cur_loc] == "#" or (
                jungle[*cur_loc] == "E" and cur_loc == address_overflow(cur_loc, dir)
            ):
                cur_loc = list_add(cur_loc, [-x for x in dirs[dir]])
                break
            elif jungle[*cur_loc] == "E":
                cur_loc = address_overflow(cur_loc, dir)

            if DEBUG:
                print(f"Step to {cur_loc}, which has value {jungle[*cur_loc]}")
    else:
        dir = (dir + dir_dict[inst]) % 4

    return (cur_loc, dir)


cur_loc = [0, 0]
dir = 0
DEBUG = False
DEBUG_SPECIAL = False

for i in instructions:
    cur_loc, dir = move(cur_loc, dir, i)

print("Part 1:", ((cur_loc[0] + 1) * 4) + ((cur_loc[1] + 1) * 1000) + dir)


# 27436

# 2 (15, 106)
# 15426
