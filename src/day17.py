from pprint import pprint
import itertools

instruction_set = open("./src/input/day17.txt", "r").read()

rocks = [
    [["x", "x", "x", "x"]],
    [[".", "x", "."], ["x", "x", "x"], [".", "x", "."]],
    [["x", "x", "x"], [".", ".", "x"], [".", ".", "x"]],
    [["x"], ["x"], ["x"], ["x"]],
    [["x", "x"], ["x", "x"]],
]

ROOM_HEIGHT = 5000
room = [[".", ".", ".", ".", ".", ".", "."] for x in range(ROOM_HEIGHT)]

inst_map = {">": (0, 1), "<": (0, -1), "v": (-1, 0)}


def move_rock(room, rock, cur_loc, inst):
    cur_rock_coords = list(itertools.product(range(len(rock)), range(len(rock[0]))))
    cur_rock_coords = [
        [x + cur_loc[0], y + cur_loc[1]]
        for x, y in cur_rock_coords
        if rock[x][y] == "x"
    ]
    mv = inst_map[inst]
    new_rock_coords = [[x + mv[0], y + mv[1]] for x, y in cur_rock_coords]
    can_move = True
    for x, y in new_rock_coords:
        # if any of the new coordinates are messed up, you can't move
        if (
            y < 0
            or y > 6
            or x < 0
            or (room[x][y] == "x" and [x, y] not in cur_rock_coords)
        ):
            can_move = False
    if not can_move:
        return False, cur_loc
    for x, y in cur_rock_coords:
        room[x][y] = "."
    for x, y in new_rock_coords:
        room[x][y] = "x"
    return True, [cur_loc[0] + mv[0], cur_loc[1] + mv[1]]


highest_rock_loc = 0
inst_counter = 0
TOWER_HEIGHT = 20
debug = False
for r in range(2022):

    rock_type = rocks[r % len(rocks)]
    start_loc = [highest_rock_loc + 3, 2]
    while True:
        i = instruction_set[inst_counter]
        inst_counter = (inst_counter + 1) % len(instruction_set)
        could_move, start_loc = move_rock(room, rock_type, start_loc, i)
        if debug and could_move:
            pprint(room[:TOWER_HEIGHT])
        could_move, start_loc = move_rock(room, rock_type, start_loc, "v")
        if debug and could_move:
            pprint(room[:TOWER_HEIGHT])
        if not could_move:
            break  # exit if it couldn't fall down

    highest_rock_loc = max(len(rock_type) + start_loc[0], highest_rock_loc)

print(highest_rock_loc)
