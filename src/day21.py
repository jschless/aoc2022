class Node:
    def __init__(self, name, op, left, right):
        self.name = name
        self.op = op
        self.left = left
        self.right = right


class Root:
    def __init__(self, name, value):
        self.name = name
        self.value = value


lines = open("./src/input/day21.txt", "r").read().split("\n")
ops = {
    "+": lambda x, y: x + y,
    "*": lambda x, y: x * y,
    "-": lambda x, y: x - y,
    "/": lambda x, y: x / y,
}
node_dict = {}
for line in lines:
    toks = line.split(":")
    name, vals = toks[0].strip(), toks[1].strip()
    if vals.isdigit():
        node_dict[name] = Root(name, int(vals))
    else:
        op_toks = vals.split(" ")
        node_dict[name] = Node(name, ops[op_toks[1]], op_toks[0], op_toks[2])


def traverse_tree(node):
    if isinstance(node, Root):
        return node.value
    else:
        return node.op(
            traverse_tree(node_dict[node.left]), traverse_tree(node_dict[node.right])
        )


root = node_dict["root"]
print("Part 1:", int(traverse_tree(root)))

MAX_VAL = 100000000000000
l, r = -1 * MAX_VAL, MAX_VAL
while l < r:
    mid = int((l + r) / 2)
    node_dict["humn"] = Root("humn", mid)
    l_res = traverse_tree(node_dict[root.left])
    r_res = traverse_tree(node_dict[root.right])
    if l_res == r_res:
        print(f"Part 2: {mid}")
        break
    elif l_res > r_res:
        l = mid + 1
    else:
        r = mid
