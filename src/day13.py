import os

inp = open(os.getcwd() + "/src/input/day13.txt", "r").read()


def compare(left, right):
    if left == []:  # right can't finish first if its empty
        return 0 if right == [] else -1
    elif right == []:
        return 1
    elif isinstance(left, int) and isinstance(right, int):
        if left < right:
            return -1
        elif left == right:
            return 0
        else:
            return 1
    elif isinstance(left, list) and isinstance(right, list):
        val = compare(left[0], right[0])
        if val == 0:
            return compare(left[1:], right[1:])
        return val
    elif isinstance(left, int):
        return compare([left], right)
    elif isinstance(right, int):
        return compare(left, [right])


import functools

pairs = [[eval(s) for s in pair.split("\n")] for pair in inp.split("\n\n")]
sum = 0
for i, p in enumerate(pairs):
    if compare(p[0], p[1]) <= 0:
        sum += i + 1

# part 2
pairs.append([[[2]], [[6]]])

flat_list = [item for sublist in pairs for item in sublist]
items = sorted(
    enumerate(flat_list), key=functools.cmp_to_key(lambda lh, rh: compare(lh[1], rh[1]))
)

acc = 1
for n, tup in enumerate(items):
    i, val = tup
    if i == len(items) - 1 or i == len(items) - 2:
        acc *= n + 1

print(acc)
