lines = open("./src/input/day25.txt", "r").read().split("\n")

mapper = {
    "=": -2,
    "-": -1,
    "0": 0,
    "1": 1,
    "2": 2,
    2: "2",
    1: "1",
    0: "0",
    3: "=",
    4: "-",
    5: "0",
}


def get_snafu(s):
    output = ""
    rest = 0
    while s != 0 or rest:
        remainder = s % 5 + rest
        rest = 0 if remainder <= 2 else 1
        output = mapper[remainder] + output
        # print(f"s: {s} rest: {rest} remainder: {remainder} output: {output}")
        s //= 5
    return output


parse = lambda line: sum(mapper.get(s) * (5**i) for i, s in enumerate(line[::-1]))
print("Part 1:", get_snafu(sum([parse(l.strip()) for l in lines])))
