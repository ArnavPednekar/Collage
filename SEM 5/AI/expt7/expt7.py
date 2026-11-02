import heapq


def best_first_search(graph, heuristic, start, goal):
    open_list = []
    counter = 0

    heapq.heappush(open_list, (heuristic[start], counter, start))

    parent = {start: None}
    visited = set()

    while open_list:
        h_value, _, current = heapq.heappop(open_list)

        if current in visited:
            continue

        visited.add(current)
        print(f"Selected {current} with h={h_value}")

        if current == goal:
            path = []
            node = current
            while node is not None:
                path.append(node)
                node = parent[node]
            return path[::-1]

        for neighbour in graph.get(current, []):
            if neighbour not in visited and neighbour not in parent:
                parent[neighbour] = current
                counter += 1
                heapq.heappush(
                    open_list,
                    (heuristic[neighbour], counter, neighbour)
                )
                print(f"  Added {neighbour} with h={heuristic[neighbour]}")

    return None


if __name__ == "__main__":
    graph = {
        'A': ['B', 'C'],
        'B': ['C', 'D'],
        'C': ['D'],
        'D': ['E'],
        'E': []
    }

    heuristic = {
        'A': 7,
        'B': 6,
        'C': 2,
        'D': 1,
        'E': 0
    }

    start = 'A'
    goal = 'E'

    print("Best First Search Algorithm")
    print(f"Start node: {start}, Goal node: {goal}")

    path = best_first_search(graph, heuristic, start, goal)

    if path:
        print("\nSolution Path:", " -> ".join(path))
    else:
        print("\nNo solution found.")
