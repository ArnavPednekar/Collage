import heapq


def a_star(graph, heuristic, start, goal):
    open_list = []
    counter = 0

    heapq.heappush(open_list, (heuristic[start], counter, start))

    g_cost = {start: 0}
    parent = {start: None}
    closed = set()

    while open_list:
        f_value, _, current = heapq.heappop(open_list)

        if current in closed:
            continue

        closed.add(current)
        print(f"Expanded {current}: g={g_cost[current]}, "
              f"h={heuristic[current]}, f={f_value}")

        if current == goal:
            path = []
            node = current
            while node is not None:
                path.append(node)
                node = parent[node]
            return path[::-1], g_cost[goal]

        for neighbour, cost in graph.get(current, {}).items():
            tentative_g = g_cost[current] + cost

            if tentative_g < g_cost.get(neighbour, float("inf")):
                g_cost[neighbour] = tentative_g
                parent[neighbour] = current
                counter += 1
                f_neighbour = tentative_g + heuristic[neighbour]
                heapq.heappush(
                    open_list, (f_neighbour, counter, neighbour)
                )
                print(f"  Updated {neighbour}: g={tentative_g}, "
                      f"h={heuristic[neighbour]}, f={f_neighbour}")

    return None, None


if __name__ == "__main__":
    graph = {
        'A': {'B': 1, 'C': 4},
        'B': {'C': 2, 'D': 5},
        'C': {'D': 1},
        'D': {'E': 3},
        'E': {}
    }

    heuristic = {
        'A': 6,
        'B': 5,
        'C': 3,
        'D': 2,
        'E': 0
    }

    start = 'A'
    goal = 'E'

    print("A* Search Algorithm")
    print(f"Start node: {start}, Goal node: {goal}")

    path, total_cost = a_star(graph, heuristic, start, goal)

    if path:
        print("\nOptimal Path:", " -> ".join(path))
        print("Total Cost:", total_cost)
    else:
        print("\nNo path found.")
