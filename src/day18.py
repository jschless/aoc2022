coords = [
    list(map(int, x.split(",")))
    for x in open("./src/input/day18.txt", "r").read().split("\n")
]

n_touching = 0
for i in range(len(coords)):
    for j in range(i + 1, len(coords)):
        if sum([abs(x - y) for x, y in zip(coords[i], coords[j])]) == 1:
            n_touching += 1

print("Part 1:", len(coords) * 6 - n_touching * 2)

seen = set()
todo = [(-1, -1, -1)]
sides = lambda x, y, z: {
    (x + 1, y, z),
    (x - 1, y, z),
    (x, y + 1, z),
    (x, y - 1, z),
    (x, y, z + 1),
    (x, y, z - 1),
}
c_set = set([tuple(x) for x in coords])

while todo:
    here = todo.pop()
    todo += [s for s in (sides(*here) - c_set - seen) if all(-1 <= c <= 25 for c in s)]
    seen.add(here)

acc = 0
for c in coords:
    for s in sides(*c):
        if s in seen:
            acc += 1
print("Part 2:", acc)
# part 2
# potential_void = []
# # iterate over all coordinates in the space
# for x in range(x_max):
#     for y in range(y_max):
#         for z in range(z_max):
#             n_count = 0
#             for x1, y1, z1 in neighbors:
#                 if (x + x1, y + y1, z + z1) in c_set:
#                     n_count += 1
#             if n_count == 6 and (x, y, z) not in c_set:
#                 potential_void.append([x, y, z])

# # figure out how many of the voids are touching because we need to remove two per
# n_void_touching = 0
# for i in range(len(potential_void)):
#     for j in range(i + 1, len(potential_void)):
#         if sum([abs(x - y) for x, y in zip(potential_void[i], potential_void[j])]) == 1:
#             n_void_touching += 1


# def dfs(visited, graph, node):  # function for dfs
#     if node not in visited:
#         visited.add(node)
#         for neighbour in graph[node]:
#             dfs(visited, graph, neighbour)


# from pprint import pprint

# print(n_void_touching)

# # print(potential_void)
# print(
#     "Part 2:",
#     (len(coords) - len(potential_void)) * 6 - n_touching * 2,
# )
