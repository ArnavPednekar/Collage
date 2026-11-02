"""Procedural maze generation.

Algorithm:
  1. Randomized depth-first search (recursive backtracker) carves a perfect
     maze on the odd-coordinate cell lattice.
  2. "Braiding" removes a fraction of the remaining interior walls, which
     turns dead ends into loops. Loops are important for tower defense:
     the player can partially block a route and enemies will re-route.
  3. Spawn and goal are picked on opposite sides.

The generator guarantees a walkable path between spawn and goal.
"""

import random

from .config import COLS, ROWS, WALL, FLOOR

BRAID_CHANCE = 0.32


def generate_level(rng=None, braid_chance=BRAID_CHANCE, cols=COLS, rows=ROWS):
    rng = rng or random.Random()
    grid = [[WALL] * cols for _ in range(rows)]

    # --- 1. recursive backtracker on odd cells -------------------------
    start = (1, 1)
    grid[start[1]][start[0]] = FLOOR
    stack = [start]

    while stack:
        x, y = stack[-1]
        neighbours = []
        for dx, dy in ((2, 0), (-2, 0), (0, 2), (0, -2)):
            nx, ny = x + dx, y + dy
            if 1 <= nx < cols - 1 and 1 <= ny < rows - 1 and grid[ny][nx] == WALL:
                neighbours.append((nx, ny))

        if neighbours:
            nx, ny = rng.choice(neighbours)
            grid[(y + ny) // 2][(x + nx) // 2] = FLOOR
            grid[ny][nx] = FLOOR
            stack.append((nx, ny))
        else:
            stack.pop()

    # --- 2. braid: open some walls to create loops ---------------------
    for y in range(1, rows - 1):
        for x in range(1, cols - 1):
            if grid[y][x] == WALL and rng.random() < braid_chance:
                grid[y][x] = FLOOR

    # --- 3. spawn on the left, goal on the right -----------------------
    odd_rows = [y for y in range(1, rows - 1, 2)]
    sy = rng.choice(odd_rows)
    gy = rng.choice(odd_rows)
    spawn = (1, sy)
    goal = (cols - 2, gy)

    grid[spawn[1]][spawn[0]] = FLOOR
    grid[goal[1]][goal[0]] = FLOOR

    return grid, spawn, goal


def is_floor(grid, cell):
    x, y = cell
    return 0 <= x < COLS and 0 <= y < ROWS and grid[y][x] == FLOOR
