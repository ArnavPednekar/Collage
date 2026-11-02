"""Generate the diagrams used by the presentation deck."""

import heapq
import os
import random
import sys

os.environ.setdefault("SDL_VIDEODRIVER", "dummy")
os.environ.setdefault("SDL_AUDIODRIVER", "dummy")

import pygame

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, ROOT)

from td.config import WALL, FLOOR, C_BG  # noqa: E402
from td.maze import generate_level  # noqa: E402
from td.pathfinding import astar  # noqa: E402

BG = (14, 16, 24)
WALL_COL = (50, 56, 80)
WALL_EDGE = (26, 30, 44)
FLOOR_COL = (18, 21, 30)
FLOOR_ALT = (21, 24, 34)
CYAN = (80, 200, 255)
GREEN = (95, 220, 145)
RED = (240, 95, 105)
GOLD = (250, 205, 90)


def _floor_shade(x, y):
    return FLOOR_COL if (x + y) % 2 == 0 else FLOOR_ALT


def render(grid, spawn, goal, path, cell, visited=None, towers=(), old_path=None,
           tower_color=(80, 180, 255)):
    cols, rows = len(grid[0]), len(grid)
    surf = pygame.Surface((cols * cell, rows * cell))
    surf.fill(BG)

    order = len(visited) if visited else 0
    for y in range(rows):
        for x in range(cols):
            rect = pygame.Rect(x * cell, y * cell, cell - 1, cell - 1)
            if grid[y][x] == WALL:
                pygame.draw.rect(surf, WALL_COL, rect)
                pygame.draw.rect(surf, WALL_EDGE, rect, 1)
            else:
                color = _floor_shade(x, y)
                if visited and (x, y) in visited:
                    t = visited[(x, y)] / max(1, order - 1)
                    color = (
                        int(22 + 34 * t),
                        int(32 + 58 * t),
                        int(46 + 78 * t),
                    )
                pygame.draw.rect(surf, color, rect)

    if old_path and len(old_path) > 1:
        old_points = [((x + 0.5) * cell, (y + 0.5) * cell) for x, y in old_path]
        pygame.draw.lines(surf, (96, 104, 124), False, old_points,
                          max(2, int(cell * 0.11)))

    if path and len(path) > 1:
        glow = pygame.Surface(surf.get_size(), pygame.SRCALPHA)
        points = [((x + 0.5) * cell, (y + 0.5) * cell) for x, y in path]
        pygame.draw.lines(glow, (*CYAN, 45), False, points, int(cell * 0.55))
        pygame.draw.lines(glow, (*CYAN, 70), False, points, int(cell * 0.28))
        surf.blit(glow, (0, 0))
        pygame.draw.lines(surf, CYAN, False, points, max(2, int(cell * 0.09)))
        for px, py in points:
            pygame.draw.circle(surf, (210, 245, 255), (int(px), int(py)), max(1, cell // 14))

    for tx, ty in towers:
        cx, cy = (tx + 0.5) * cell, (ty + 0.5) * cell
        pygame.draw.circle(surf, (12, 14, 20), (int(cx), int(cy)), int(cell * 0.36))
        pygame.draw.circle(surf, tower_color, (int(cx), int(cy)), int(cell * 0.27))

    sx, sy = (spawn[0] + 0.5) * cell, (spawn[1] + 0.5) * cell
    pygame.draw.circle(surf, (26, 62, 47), (int(sx), int(sy)), int(cell * 0.42))
    pygame.draw.circle(surf, GREEN, (int(sx), int(sy)), int(cell * 0.42), 2)
    pygame.draw.polygon(
        surf,
        GREEN,
        [
            (sx - cell * 0.18, sy - cell * 0.18),
            (sx + cell * 0.18, sy),
            (sx - cell * 0.18, sy + cell * 0.18),
        ],
    )

    gx, gy = (goal[0] + 0.5) * cell, (goal[1] + 0.5) * cell
    pygame.draw.circle(surf, (66, 28, 42), (int(gx), int(gy)), int(cell * 0.42))
    pygame.draw.circle(surf, RED, (int(gx), int(gy)), int(cell * 0.42), 2)
    pygame.draw.circle(surf, RED, (int(gx), int(gy)), int(cell * 0.15))

    return surf


def search_with_trace(start, goal, blocked, cols, rows):
    def h(a, b):
        return abs(a[0] - b[0]) + abs(a[1] - b[1])

    frontier = [(h(start, goal), 0, start)]
    counter = 0
    cost = {start: 0}
    came = {}
    closed = {}
    tick = 0

    while frontier:
        _, _, cur = heapq.heappop(frontier)
        if cur in closed:
            continue
        closed[cur] = tick
        tick += 1
        if cur == goal:
            path = [cur]
            while cur in came:
                cur = came[cur]
                path.append(cur)
            path.reverse()
            return path, closed
        for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            nxt = (cur[0] + dx, cur[1] + dy)
            if not (0 <= nxt[0] < cols and 0 <= nxt[1] < rows):
                continue
            if nxt in blocked or nxt in closed:
                continue
            ng = cost[cur] + 1
            if ng < cost.get(nxt, 1 << 30):
                cost[nxt] = ng
                came[nxt] = cur
                counter += 1
                heapq.heappush(frontier, (ng + h(nxt, goal), counter, nxt))
    return None, closed


def main():
    pygame.init()
    out = os.path.join(ROOT, "assets")
    os.makedirs(out, exist_ok=True)

    def walls_of(grid):
        return {
            (x, y)
            for y in range(len(grid))
            for x in range(len(grid[0]))
            if grid[y][x] == WALL
        }

    # --- large maze + A* route (slide 3) -------------------------------
    rng = random.Random(228)
    grid, spawn, goal = generate_level(rng, braid_chance=0.18)
    path = astar(spawn, goal, walls_of(grid), len(grid[0]), len(grid))
    img = render(grid, spawn, goal, path, 30)
    pygame.image.save(img, os.path.join(out, "maze_output.png"))

    # --- A* trace (slide 4) -------------------------------------------
    rng = random.Random(356)
    grid, spawn, goal = generate_level(
        rng, braid_chance=0.12, cols=15, rows=9
    )
    path, visited = search_with_trace(
        spawn, goal, walls_of(grid), len(grid[0]), len(grid)
    )
    img = render(grid, spawn, goal, path, 40, visited=visited)
    pygame.image.save(img, os.path.join(out, "astar_demo.png"))

    # --- before / after repathing (slide 7) ---------------------------
    best = None
    for seed in range(300):
        grid, spawn, goal = generate_level(random.Random(seed))
        walls = walls_of(grid)
        path = astar(spawn, goal, walls, len(grid[0]), len(grid))
        for cell in path[1:-1]:
            new_path = astar(spawn, goal, walls | {cell}, len(grid[0]), len(grid))
            if new_path is None:
                continue
            detour = len(new_path) - len(path)
            if best is None or detour > best[0]:
                best = (detour, seed, cell, path, new_path, grid, spawn, goal)

    _, seed, cell, path, new_path, grid, spawn, goal = best
    img = render(grid, spawn, goal, new_path, 30, towers=(cell,),
                 old_path=path, tower_color=(255, 160, 70))
    pygame.image.save(img, os.path.join(out, "repath_demo.png"))
    print("repath seed", seed, "blocked", cell, "detour", best[0])

    # --- portrait hero (title slide) ----------------------------------
    rng = random.Random(82)
    grid, spawn, goal = generate_level(
        rng, braid_chance=0.18, cols=18, rows=22
    )
    path = astar(spawn, goal, walls_of(grid), len(grid[0]), len(grid))
    img = render(grid, spawn, goal, path, 36)
    pygame.image.save(img, os.path.join(out, "hero_maze.png"))

    print("assets written to", out)
    pygame.quit()


if __name__ == "__main__":
    main()
