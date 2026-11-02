from collections import deque


def bfs(graph, start):
    visited = set()
    queue = deque([start])
    visited.add(start)
    order = []

    while queue:
        node = queue.popleft()
        order.append(node)
        for neighbour in graph.get(node, []):
            if neighbour not in visited:
                visited.add(neighbour)
                queue.append(neighbour)
    return order


def dfs(graph, start):
    visited = set()
    order = []

    def dfs_recursive(node):
        visited.add(node)
        order.append(node)
        for neighbour in graph.get(node, []):
            if neighbour not in visited:
                dfs_recursive(neighbour)

    dfs_recursive(start)
    return order


if __name__ == "__main__":
    # Example graph from Experiment 3
    # Level 0: A
    # Level 1: B, C
    # Level 2: D, E, F
    graph = {
        'A': ['B', 'C'],
        'B': ['D', 'E'],
        'C': ['F'],
        'D': [],
        'E': [],
        'F': []
    }

    start = 'A'

    bfs_order = bfs(graph, start)
    print("BFS traversal starting from", start, ":")
    print(" -> ".join(bfs_order))

    dfs_order = dfs(graph, start)
    print("\nDFS traversal starting from", start, ":")
    print(" -> ".join(dfs_order))
