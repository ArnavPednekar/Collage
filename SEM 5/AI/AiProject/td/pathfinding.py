"""A* pathfinding over the tile grid.

The grid is 4-connected. Towers are treated as blocked cells, so every time
a tower is placed the enemies recompute their route. This is the core AI
algorithm of the project: heuristic search with a Manhattan distance
estimate on a uniform-cost grid.
"""

import heapq

from .config import COLS, ROWS, DIRS


def heuristic(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


def astar(start, goal, blocked, cols=COLS, rows=ROWS):
    """Return a list of (x, y) cells from start to goal, or None.

    ``blocked`` is a set of cells that cannot be entered (walls and towers).
    """
    if start == goal:
        return [start]
    if start in blocked or goal in blocked:
        return None

    frontier = [(heuristic(start, goal), 0, start)]
    counter = 0
    cost = {start: 0}
    came_from = {}
    closed = set()

    while frontier:
        _, _, current = heapq.heappop(frontier)
        if current in closed:
            continue
        if current == goal:
            path = [current]
            while current in came_from:
                current = came_from[current]
                path.append(current)
            path.reverse()
            return path

        closed.add(current)
        g = cost[current]

        for dx, dy in DIRS:
            nxt = (current[0] + dx, current[1] + dy)
            if not (0 <= nxt[0] < cols and 0 <= nxt[1] < rows):
                continue
            if nxt in blocked or nxt in closed:
                continue
            ng = g + 1
            if ng < cost.get(nxt, 1 << 30):
                cost[nxt] = ng
                came_from[nxt] = current
                counter += 1
                heapq.heappush(frontier, (ng + heuristic(nxt, goal), counter, nxt))

    return None
