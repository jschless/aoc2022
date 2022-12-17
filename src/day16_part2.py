s = open("./src/input/day16.txt", "r").read()

valve_to_int = {}
valves = []  # (value, opened, neighbors)
must_visits = []
start_node = 0
for i, line in enumerate(s.split("\n")):
    flow_rate = int(line[line.find("flow rate=") + 10 : line.find(";")])
    valve_to_int[line[6:8]] = i
    temp_n = line[line.find("to valve") + 9 :].strip()
    if len(temp_n) == 2:
        neighbors = [temp_n]
    else:
        neighbors = temp_n.split(", ")
    valves.append([flow_rate, False, neighbors])
    if flow_rate > 0:
        must_visits.append(i)
    elif line[6:8] == "AA":
        start_node = i
        must_visits.append(i)


def floyd_warshall():
    dist = {}
    for i, v in enumerate(valves):
        for n in v[2]:
            n_num = valve_to_int[n]
            dist[(i, n_num)] = 1
        dist[(i, i)] = 0
    BIG = 100000
    for k in range(len(valves)):
        for i in range(len(valves)):
            for j in range(len(valves)):
                if dist.get((i, j), BIG) > dist.get((i, k), BIG) + dist.get(
                    (k, j), BIG
                ):
                    dist[(i, j)] = dist[(i, k)] + dist[(k, j)]
    return dist


dist = floyd_warshall()
memo = {}

debug = False


def compute_path(cur_node, path, time, cur_flow):
    if len(path) == 0:
        if debug:
            print(
                f"at the end of the path, we have flow {cur_flow} and {time} seconds remaining"
            )
        return cur_flow * time
    else:
        i = path[0]
        steps = dist[(cur_node, i)] + 1
        new_flow = valves[i][0]
        if debug:
            print(
                f"Moving to node {i} which takes {steps} seconds and add {new_flow} flow"
            )
        return steps * cur_flow + compute_path(
            i, path[1:], time - steps, cur_flow + new_flow
        )


def maximize(cur_node, remaining_nodes, cur_flow, time):
    if time <= 0:
        return 0, []
    elif len(remaining_nodes) == 0:
        # we've explored everything, just need to return our flow * time remaining
        return cur_flow * time, [cur_node]
    elif tuple(sorted(remaining_nodes)) in memo:
        temp = memo[(tuple(sorted(remaining_nodes)), time)]
        return (
            compute_path(cur_node, temp, time, cur_flow),
            [cur_node, *temp],
        )
    else:
        results = []
        for i in remaining_nodes:  # try every node
            new_flow = valves[i][0]
            steps = dist[(cur_node, i)] + 1
            if steps > time:  # we won't make it to this node
                val = time * cur_flow, []
            # time it will take to get there plus time to turn valve
            else:
                res = maximize(
                    i,
                    [x for x in remaining_nodes if x != i],
                    cur_flow + new_flow,
                    time - steps,
                )
                val = (steps * cur_flow) + res[0], res[1]
            results.append(val)
        best = max(results, key=lambda x: x[0])
        memo[(tuple(sorted(remaining_nodes)), time)] = best[1]
        return best[0], [cur_node, *best[1]]


print(
    maximize(
        start_node,
        [x for x in must_visits if x != start_node],
        valves[start_node][0],
        26,
    )
)
for k, v in memo.items():
    if k[1] > 20:
        print(k, v)
print(start_node)
print(compute_path(start_node, [42, 26, 1, 22, 17, 28], 26, 0))
non_overlapping = []
for key1, tuple1 in memo.items():
    if len(tuple1) > 2:
        for key2, tuple2 in memo.items():
            if len(tuple2) > 2 and len(set(tuple1).intersection(set(tuple2))) <= 2:
                non_overlapping.append((tuple1, tuple2))

print(
    max(
        [
            compute_path(start_node, x[0], 26, 0)
            + compute_path(
                start_node, [a for a in list(x[1]) if a not in set(x[0])], 26, 0
            )
            for x in non_overlapping
        ]
    )
)
# 1580 2213
