part1 = True

d_key = 1 if part1 else 811589153

inp = "./src/input/day20.txt"
contents = [d_key * int(x) for x in open(inp, "r").read().split("\n")]

contents = list(enumerate(contents))
file_length = len(contents)

for n in range(1 if part1 else 10):
    for i in range(file_length):
        i_1, (i_0, val) = next((a, x) for (a, x) in enumerate(contents) if x[0] == i)
        # i_1 is current location of value
        contents.pop(i_1)  # remove item
        # i_2 is new location of value
        i_2 = (i_1 + val) % (file_length - 1)
        contents.insert(i_2, (i_0, val))

loc_0 = next(i for i, x in enumerate(contents) if x[1] == 0)
print(sum([contents[(x + loc_0) % file_length][1] for x in [1000, 2000, 3000]]))
