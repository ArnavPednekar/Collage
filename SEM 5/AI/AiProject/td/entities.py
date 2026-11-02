"""Enemies, towers and projectiles."""

import math

import pygame

from .config import TILE, ENEMY_TYPES, TOWER_TYPES, DIRS
from .pathfinding import astar


def cell_center(cell):
    return (cell[0] + 0.5) * TILE, (cell[1] + 0.5) * TILE


class Enemy:
    def __init__(self, kind, path, goal, hp_mult=1.0, speed_mult=1.0):
        data = ENEMY_TYPES[kind]
        self.kind = kind
        self.name = data["name"]
        self.max_hp = max(1, int(round(data["hp"] * hp_mult)))
        self.hp = float(self.max_hp)
        self.speed = data["speed"] * speed_mult
        self.reward = data["reward"]
        self.leak = data["leak"]
        self.radius = data["radius"]
        self.color = data["color"]
        self.goal = goal
        self.path = path
        self.idx = 1
        self.pos = pygame.Vector2(cell_center(path[0]))
        self.travelled = 0.0
        self.alive = True
        self.leaked = False

    def update(self, dt):
        if not self.alive:
            return
        remaining = self.speed * dt

        while remaining > 0 and self.idx < len(self.path):
            tx, ty = self.path[self.idx]
            target = pygame.Vector2(cell_center((tx, ty)))
            delta = target - self.pos
            dist = delta.length()

            if dist <= remaining:
                self.pos = target
                self.travelled += dist
                remaining -= dist
                self.idx += 1
            else:
                self.pos += delta / dist * remaining
                self.travelled += remaining
                remaining = 0.0

        if self.idx >= len(self.path):
            self.leaked = True
            self.alive = False

    def repath(self, blocked):
        """Recompute the route from the enemy's current cell to the goal."""
        cell = (int(self.pos.x // TILE), int(self.pos.y // TILE))
        path = astar(cell, self.goal, blocked)
        if path is None:
            return False
        self.path = path
        self.idx = 1 if len(path) > 1 else 0
        return True

    def remaining_distance(self):
        """Path length left to the goal, used for tower target selection."""
        if self.idx >= len(self.path):
            return 0.0
        total = 0.0
        tx, ty = self.path[self.idx]
        target = pygame.Vector2(cell_center((tx, ty)))
        total += (target - self.pos).length()
        for i in range(self.idx, len(self.path) - 1):
            ax, ay = self.path[i]
            bx, by = self.path[i + 1]
            total += (abs(ax - bx) + abs(ay - by)) * TILE
        return total


class Tower:
    def __init__(self, kind, cell):
        data = TOWER_TYPES[kind]
        self.kind = kind
        self.name = data["name"]
        self.cell = cell
        self.pos = pygame.Vector2(cell_center(cell))
        self.range = data["range"] * TILE
        self.damage = data["damage"]
        self.rate = data["rate"]
        self.color = data["color"]
        self.proj_speed = data["proj_speed"]
        self.cooldown = 0.0
        self.angle = -math.pi / 2

    def update(self, dt, enemies, projectiles):
        self.cooldown -= dt
        if self.cooldown > 0:
            return

        target = None
        best = None
        for enemy in enemies:
            if not enemy.alive or enemy.leaked:
                continue
            if self.pos.distance_to(enemy.pos) <= self.range + enemy.radius * 0.5:
                dist = enemy.remaining_distance()
                if best is None or dist < best:
                    best = dist
                    target = enemy

        if target is not None:
            self.cooldown = 1.0 / self.rate
            self.angle = math.atan2(
                target.pos.y - self.pos.y, target.pos.x - self.pos.x
            )
            projectiles.append(
                Projectile(self.pos, target, self.damage, self.color, self.proj_speed)
            )


class Projectile:
    def __init__(self, pos, target, damage, color, speed):
        self.pos = pygame.Vector2(pos)
        self.target = target
        self.damage = damage
        self.color = color
        self.speed = speed
        self.radius = 3
        self.alive = True

    def update(self, dt):
        if not self.target.alive or self.target.leaked:
            self.alive = False
            return

        delta = self.target.pos - self.pos
        dist = delta.length()
        step = self.speed * dt

        if dist <= step or dist < 1.0:
            self.pos = pygame.Vector2(self.target.pos)
            self.target.hp -= self.damage
            self.alive = False
            if self.target.hp <= 0:
                self.target.hp = 0
                self.target.alive = False
        else:
            self.pos += delta / dist * step
