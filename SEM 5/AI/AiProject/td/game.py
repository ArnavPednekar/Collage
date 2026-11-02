"""Game loop, rendering and player interaction."""

import math
import random

import pygame

from .config import (
    COLS,
    ROWS,
    TILE,
    GRID_W,
    GRID_H,
    HUD_W,
    WIDTH,
    HEIGHT,
    FPS,
    WALL,
    START_GOLD,
    START_LIVES,
    TOTAL_WAVES,
    C_BG,
    C_PANEL,
    C_PANEL_LIGHT,
    C_PANEL_EDGE,
    C_WALL,
    C_WALL_EDGE,
    C_FLOOR,
    C_FLOOR_ALT,
    C_GRID,
    C_TEXT,
    C_MUTED,
    C_ACCENT,
    C_PATH,
    C_GOOD,
    C_BAD,
    C_GOLD,
    TOWER_TYPES,
    TOWER_ORDER,
)
from .entities import Enemy, Projectile, Tower, cell_center
from .maze import generate_level
from .pathfinding import astar


def make_font(size, bold=False):
    for name in ("dejavusansmono", "liberationmono", "consolas", "menlo", "monospace"):
        path = pygame.font.match_font(name, bold=bold)
        if path:
            return pygame.font.Font(path, size)
    return pygame.font.Font(None, size)


class Game:
    def __init__(self):
        pygame.init()
        pygame.display.set_caption("AI Tower Defense - A* Maze Defense")
        self.screen = pygame.display.set_mode((WIDTH, HEIGHT))
        self.clock = pygame.time.Clock()
        self.running = True
        self.fonts = {}
        self.rng = random.Random()

        self.tower_buttons = [
            (kind, pygame.Rect(GRID_W + 18, 160 + i * 64, HUD_W - 36, 58))
            for i, kind in enumerate(TOWER_ORDER)
        ]
        self.start_button = pygame.Rect(GRID_W + 18, 356, HUD_W - 36, 46)
        self.new_button = pygame.Rect(GRID_W + 18, 410, HUD_W - 36, 34)

        self.new_game()

    # ------------------------------------------------------------------
    # setup
    # ------------------------------------------------------------------
    def new_game(self):
        self.grid, self.spawn, self.goal = generate_level(self.rng)
        self.blocked = {
            (x, y)
            for y in range(ROWS)
            for x in range(COLS)
            if self.grid[y][x] == WALL
        }
        self.towers = []
        self.enemies = []
        self.projectiles = []

        self.gold = START_GOLD
        self.lives = START_LIVES
        self.wave = 0
        self.wave_active = False
        self.spawn_queue = []
        self.spawn_timer = 0.0
        self.spawn_interval = 0.8
        self.hp_mult = 1.0
        self.speed_mult = 1.0

        self.selected = "basic"
        self.show_path = True
        self.speed = 1.0
        self.game_over = False
        self.victory = False

        self.message = ""
        self.message_color = C_TEXT
        self.message_timer = 0.0

        self.preview_path = astar(self.spawn, self.goal, self.blocked)

    def font(self, size, bold=False):
        key = (size, bold)
        if key not in self.fonts:
            self.fonts[key] = make_font(size, bold)
        return self.fonts[key]

    # ------------------------------------------------------------------
    # events
    # ------------------------------------------------------------------
    def handle_events(self):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.running = False
            elif event.type == pygame.KEYDOWN:
                self.handle_key(event.key)
            elif event.type == pygame.MOUSEBUTTONDOWN:
                if event.button == 1:
                    self.on_click(event.pos)
                elif event.button == 3:
                    self.on_right_click(event.pos)

    def handle_key(self, key):
        if key == pygame.K_ESCAPE:
            self.running = False
        elif key in (pygame.K_1, pygame.K_2, pygame.K_3):
            self.selected = TOWER_ORDER[key - pygame.K_1]
        elif key == pygame.K_SPACE:
            self.start_wave()
        elif key == pygame.K_r:
            self.new_game()
        elif key == pygame.K_f:
            self.speed = 1.0 if self.speed >= 3.0 else self.speed + 1.0
        elif key == pygame.K_p:
            self.show_path = not self.show_path

    def on_click(self, pos):
        if pos[0] >= GRID_W:
            for kind, rect in self.tower_buttons:
                if rect.collidepoint(pos):
                    self.selected = kind
                    return
            if self.start_button.collidepoint(pos):
                self.start_wave()
            elif self.new_button.collidepoint(pos):
                self.new_game()
            return

        cell = (pos[0] // TILE, pos[1] // TILE)
        self.try_place(cell)

    def on_right_click(self, pos):
        if pos[0] >= GRID_W:
            return
        self.sell_tower((pos[0] // TILE, pos[1] // TILE))

    # ------------------------------------------------------------------
    # actions
    # ------------------------------------------------------------------
    def flash(self, text, color=C_TEXT):
        self.message = text
        self.message_color = color
        self.message_timer = 2.0

    def placement_error(self, cell):
        x, y = cell
        if not (0 <= x < COLS and 0 <= y < ROWS):
            return "Outside the map"
        if self.grid[y][x] == WALL:
            return "Wall"
        if cell == self.spawn or cell == self.goal:
            return "Spawn / goal"
        if any(t.cell == cell for t in self.towers):
            return "Occupied"
        for e in self.enemies:
            if e.alive and int(e.pos.x // TILE) == x and int(e.pos.y // TILE) == y:
                return "Enemy here"
        if self.gold < TOWER_TYPES[self.selected]["cost"]:
            return "Not enough gold"
        if astar(self.spawn, self.goal, self.blocked | {cell}) is None:
            return "Would block the path"
        return None

    def try_place(self, cell):
        error = self.placement_error(cell)
        if error:
            self.flash(error, C_BAD)
            return

        data = TOWER_TYPES[self.selected]
        self.gold -= data["cost"]
        self.blocked.add(cell)
        self.towers.append(Tower(self.selected, cell))

        for enemy in self.enemies:
            enemy.repath(self.blocked)

        self.preview_path = astar(self.spawn, self.goal, self.blocked)
        self.flash(f"{data['name']} built", C_GOOD)

    def sell_tower(self, cell):
        for i, tower in enumerate(self.towers):
            if tower.cell != cell:
                continue
            refund = int(TOWER_TYPES[tower.kind]["cost"] * 0.6)
            self.gold += refund
            self.towers.pop(i)
            self.blocked.discard(cell)
            for enemy in self.enemies:
                enemy.repath(self.blocked)
            self.preview_path = astar(self.spawn, self.goal, self.blocked)
            self.flash(f"Sold for {refund}g", C_GOLD)
            return
        self.flash("No tower on that cell", C_MUTED)

    def build_wave(self, n):
        kinds = ["grunt"] * (3 + n)
        if n >= 3:
            kinds += ["runner"] * (n - 2)
        if n >= 5:
            kinds += ["tank"] * (1 + (n - 5) // 3)
        if n % 5 == 0:
            kinds += ["boss"] * (n // 5)
        self.rng.shuffle(kinds)
        return kinds

    def start_wave(self):
        if self.game_over or self.victory:
            return
        if self.wave_active:
            self.flash("Wave already in progress", C_MUTED)
            return
        if self.wave >= TOTAL_WAVES:
            return

        self.wave += 1
        self.spawn_queue = self.build_wave(self.wave)
        self.spawn_timer = 0.0
        self.spawn_interval = max(0.30, 0.9 - 0.035 * self.wave)
        self.hp_mult = 1.0 + 0.13 * (self.wave - 1)
        self.speed_mult = 1.0 + 0.015 * (self.wave - 1)
        self.wave_active = True
        self.flash(f"Wave {self.wave} incoming", C_ACCENT)

    # ------------------------------------------------------------------
    # simulation
    # ------------------------------------------------------------------
    def update(self, real_dt):
        if self.message_timer > 0:
            self.message_timer -= real_dt
        if self.game_over or self.victory:
            return

        real_dt = min(real_dt, 0.1)
        steps = max(1, int(math.ceil(real_dt * self.speed / 0.025)))
        step_dt = real_dt * self.speed / steps
        for _ in range(steps):
            self.step(step_dt)

    def step(self, dt):
        if self.game_over or self.victory:
            return

        if self.wave_active:
            self.spawn_timer -= dt
            while self.spawn_queue and self.spawn_timer <= 0:
                kind = self.spawn_queue.pop(0)
                path = astar(self.spawn, self.goal, self.blocked)
                if path:
                    self.enemies.append(
                        Enemy(kind, path, self.goal, self.hp_mult, self.speed_mult)
                    )
                self.spawn_timer += self.spawn_interval

        for enemy in self.enemies:
            if enemy.alive:
                enemy.update(dt)

        for tower in self.towers:
            tower.update(dt, self.enemies, self.projectiles)

        for projectile in self.projectiles:
            projectile.update(dt)
        self.projectiles = [p for p in self.projectiles if p.alive]

        life_loss = 0
        survivors = []
        for enemy in self.enemies:
            if enemy.alive:
                survivors.append(enemy)
            elif enemy.leaked:
                life_loss += enemy.leak
            else:
                self.gold += enemy.reward
        self.enemies = survivors

        if life_loss:
            self.lives = max(0, self.lives - life_loss)
            self.flash(f"Enemy leaked  -{life_loss} life", C_BAD)

        if self.lives <= 0:
            self.lives = 0
            self.game_over = True
            self.wave_active = False
            self.flash("The base has fallen", C_BAD)
            return

        if self.wave_active and not self.spawn_queue and not self.enemies:
            self.wave_active = False
            bonus = 45 + 14 * self.wave
            self.gold += bonus
            if self.wave >= TOTAL_WAVES:
                self.victory = True
                self.flash("All waves cleared!", C_GOOD)
            else:
                self.flash(f"Wave {self.wave} cleared  +{bonus}g", C_GOOD)

    # ------------------------------------------------------------------
    # rendering
    # ------------------------------------------------------------------
    def text(self, content, size, color, pos, anchor="topleft", bold=False):
        surf = self.font(size, bold).render(content, True, color)
        rect = surf.get_rect(**{anchor: pos})
        self.screen.blit(surf, rect)
        return rect

    def draw(self):
        self.screen.fill(C_BG)
        self.draw_grid()
        if self.show_path and self.preview_path:
            self.draw_path(self.preview_path)
        self.draw_markers()
        self.draw_towers()
        self.draw_enemies()
        self.draw_projectiles()
        self.draw_hover()
        self.draw_hud()
        self.draw_overlay()
        pygame.display.flip()

    def draw_grid(self):
        for y in range(ROWS):
            for x in range(COLS):
                rect = pygame.Rect(x * TILE, y * TILE, TILE, TILE)
                if self.grid[y][x] == WALL:
                    pygame.draw.rect(self.screen, C_WALL, rect)
                    pygame.draw.rect(self.screen, C_WALL_EDGE, rect, 1)
                    pygame.draw.line(
                        self.screen,
                        C_WALL_EDGE,
                        (rect.x + 3, rect.y + 2),
                        (rect.right - 4, rect.y + 2),
                    )
                else:
                    color = C_FLOOR if (x + y) % 2 == 0 else C_FLOOR_ALT
                    pygame.draw.rect(self.screen, color, rect)
                    pygame.draw.rect(self.screen, C_GRID, rect, 1)

    def draw_path(self, path):
        if len(path) < 2:
            return
        surf = pygame.Surface((GRID_W, GRID_H), pygame.SRCALPHA)
        points = [cell_center(c) for c in path]
        pygame.draw.lines(surf, (*C_PATH, 70), False, points, 10)
        pygame.draw.lines(surf, (*C_ACCENT, 90), False, points, 3)
        for x, y in points:
            pygame.draw.circle(surf, (*C_ACCENT, 110), (int(x), int(y)), 3)
        self.screen.blit(surf, (0, 0))

    def draw_markers(self):
        sx, sy = cell_center(self.spawn)
        pygame.draw.circle(self.screen, (28, 70, 52), (int(sx), int(sy)), int(TILE * 0.42))
        pygame.draw.circle(self.screen, C_GOOD, (int(sx), int(sy)), int(TILE * 0.42), 2)
        pygame.draw.polygon(
            self.screen,
            C_GOOD,
            [(sx - 8, sy - 8), (sx + 8, sy), (sx - 8, sy + 8)],
        )

        gx, gy = cell_center(self.goal)
        pygame.draw.circle(self.screen, (74, 30, 46), (int(gx), int(gy)), int(TILE * 0.42))
        pygame.draw.circle(self.screen, C_BAD, (int(gx), int(gy)), int(TILE * 0.42), 2)
        pygame.draw.circle(self.screen, C_BAD, (int(gx), int(gy)), int(TILE * 0.16))

    def draw_towers(self):
        for tower in self.towers:
            px, py = int(tower.pos.x), int(tower.pos.y)
            radius = int(TILE * 0.36)
            pygame.draw.circle(self.screen, (14, 16, 24), (px, py), radius + 3)
            pygame.draw.circle(self.screen, tower.color, (px, py), radius)
            ex = tower.pos.x + math.cos(tower.angle) * radius * 1.15
            ey = tower.pos.y + math.sin(tower.angle) * radius * 1.15
            pygame.draw.line(self.screen, (18, 22, 32), (px, py), (ex, ey), 7)
            pygame.draw.line(self.screen, tower.color, (px, py), (ex, ey), 4)
            pygame.draw.circle(self.screen, (240, 246, 255), (px, py), 3)

    def draw_enemies(self):
        for enemy in self.enemies:
            if not enemy.alive:
                continue
            px, py = int(enemy.pos.x), int(enemy.pos.y)
            pygame.draw.circle(self.screen, (12, 14, 20), (px, py), enemy.radius + 2)
            pygame.draw.circle(self.screen, enemy.color, (px, py), enemy.radius)
            if enemy.kind == "boss":
                pygame.draw.circle(self.screen, C_GOLD, (px, py), enemy.radius + 4, 2)

            bar_w = enemy.radius * 2 + 8
            bar_h = 4
            bx = int(enemy.pos.x - bar_w / 2)
            by = int(enemy.pos.y - enemy.radius - 10)
            pygame.draw.rect(self.screen, (30, 32, 44), (bx, by, bar_w, bar_h))
            ratio = max(0.0, enemy.hp / enemy.max_hp)
            pygame.draw.rect(
                self.screen, (230, 80, 90), (bx, by, int(bar_w * ratio), bar_h)
            )

    def draw_projectiles(self):
        for projectile in self.projectiles:
            px, py = int(projectile.pos.x), int(projectile.pos.y)
            pygame.draw.circle(self.screen, projectile.color, (px, py), 5)
            pygame.draw.circle(self.screen, (255, 255, 255), (px, py), 2)

    def draw_range(self, cell, radius, color):
        cx, cy = cell_center(cell)
        d = int(radius * 2)
        surf = pygame.Surface((d, d), pygame.SRCALPHA)
        pygame.draw.circle(surf, (*color, 28), (int(radius), int(radius)), int(radius))
        pygame.draw.circle(
            surf, (*color, 120), (int(radius), int(radius)), int(radius), 1
        )
        self.screen.blit(surf, (cx - radius, cy - radius))

    def draw_hover(self):
        if self.game_over or self.victory:
            return
        mx, my = pygame.mouse.get_pos()
        if mx >= GRID_W or my >= GRID_H or mx < 0 or my < 0:
            return

        cell = (mx // TILE, my // TILE)
        error = self.placement_error(cell)
        color = C_GOOD if error is None else C_BAD

        tint = pygame.Surface((TILE, TILE), pygame.SRCALPHA)
        tint.fill((*color, 50))
        self.screen.blit(tint, (cell[0] * TILE, cell[1] * TILE))
        pygame.draw.rect(
            self.screen,
            color,
            pygame.Rect(cell[0] * TILE, cell[1] * TILE, TILE, TILE),
            2,
        )

        if error is None:
            radius = TOWER_TYPES[self.selected]["range"] * TILE
            self.draw_range(cell, radius, color)

    def draw_hud(self):
        pygame.draw.rect(self.screen, C_PANEL, (GRID_W, 0, HUD_W, HEIGHT))
        pygame.draw.line(
            self.screen, C_PANEL_EDGE, (GRID_W, 0), (GRID_W, HEIGHT), 2
        )

        self.text("AI TOWER DEFENSE", 22, C_TEXT, (GRID_W + 18, 14), bold=True)
        self.text("A* pathfinding maze defense", 12, C_MUTED, (GRID_W + 18, 43))

        stats = [
            ("GOLD", str(self.gold), C_GOLD),
            ("LIVES", str(self.lives), C_GOOD),
            ("WAVE", f"{self.wave}/{TOTAL_WAVES}", C_ACCENT),
        ]
        box_w = 82
        for i, (label, value, color) in enumerate(stats):
            rect = pygame.Rect(GRID_W + 18 + i * (box_w + 8), 72, box_w, 52)
            pygame.draw.rect(self.screen, C_PANEL_LIGHT, rect, border_radius=8)
            pygame.draw.rect(self.screen, C_PANEL_EDGE, rect, 1, border_radius=8)
            self.text(label, 10, C_MUTED, (rect.centerx, rect.y + 8), "midtop")
            self.text(value, 20, color, (rect.centerx, rect.y + 23), "midtop", True)

        self.text("TOWERS", 11, C_MUTED, (GRID_W + 18, 140), bold=True)

        for i, (kind, rect) in enumerate(self.tower_buttons):
            data = TOWER_TYPES[kind]
            selected = kind == self.selected
            fill = (40, 52, 74) if selected else C_PANEL_LIGHT
            pygame.draw.rect(self.screen, fill, rect, border_radius=8)
            pygame.draw.rect(
                self.screen,
                C_ACCENT if selected else C_PANEL_EDGE,
                rect,
                2 if selected else 1,
                border_radius=8,
            )

            badge = pygame.Rect(rect.x + 8, rect.y + 10, 20, 20)
            pygame.draw.rect(self.screen, data["color"], badge, border_radius=5)
            self.text(
                str(i + 1), 12, (14, 16, 24), (badge.centerx, badge.centery), "center", True
            )

            self.text(data["name"], 16, C_TEXT, (rect.x + 38, rect.y + 8), bold=True)
            self.text(
                f"{data['cost']}g",
                15,
                C_GOLD,
                (rect.right - 10, rect.y + 9),
                "topright",
                True,
            )
            stats_text = (
                f"{data['damage']} dmg  {data['rate']:.2f}/s  {data['range']:.1f}t"
            )
            self.text(stats_text, 11, C_MUTED, (rect.x + 38, rect.y + 32))

        can_start = (
            not self.wave_active and not self.game_over and not self.victory
        )
        if can_start:
            fill, edge, label = (28, 62, 82), C_ACCENT, "START WAVE  [Space]"
        elif self.wave_active:
            fill, edge, label = C_PANEL_LIGHT, C_PANEL_EDGE, "WAVE IN PROGRESS"
        else:
            fill, edge, label = C_PANEL_LIGHT, C_PANEL_EDGE, "NO MORE WAVES"

        pygame.draw.rect(self.screen, fill, self.start_button, border_radius=8)
        pygame.draw.rect(self.screen, edge, self.start_button, 2, border_radius=8)
        self.text(
            label,
            14,
            C_TEXT if can_start else C_MUTED,
            self.start_button.center,
            "center",
            True,
        )

        pygame.draw.rect(self.screen, C_PANEL_LIGHT, self.new_button, border_radius=8)
        pygame.draw.rect(self.screen, C_PANEL_EDGE, self.new_button, 1, border_radius=8)
        self.text(
            "REGENERATE MAZE  [R]",
            12,
            C_MUTED,
            self.new_button.center,
            "center",
            True,
        )

        self.text(
            f"Speed: {int(self.speed)}x   [F]",
            13,
            C_MUTED,
            (GRID_W + 18, 458),
        )
        self.text(
            f"Path preview: {'ON' if self.show_path else 'OFF'}   [P]",
            13,
            C_MUTED,
            (GRID_W + 18, 480),
        )

        self.text("CONTROLS", 11, C_MUTED, (GRID_W + 18, 512), bold=True)
        controls = [
            "1 / 2 / 3    select tower",
            "Click        build tower",
            "Right-click  sell tower",
            "Space        start next wave",
            "F / P        speed / A* path",
            "R            new maze",
        ]
        for i, line in enumerate(controls):
            self.text(line, 12, C_MUTED, (GRID_W + 18, 532 + i * 20))

        if self.message_timer > 0 and self.message:
            self.text(
                self.message,
                13,
                self.message_color,
                (GRID_W + HUD_W // 2, HEIGHT - 16),
                "midbottom",
                True,
            )

    def draw_overlay(self):
        if not (self.game_over or self.victory):
            return

        veil = pygame.Surface((WIDTH, HEIGHT), pygame.SRCALPHA)
        veil.fill((6, 8, 14, 200))
        self.screen.blit(veil, (0, 0))

        if self.victory:
            title, color = "VICTORY", C_GOOD
            sub = f"The maze held for all {TOTAL_WAVES} waves"
        else:
            title, color = "BASE DESTROYED", C_BAD
            sub = f"You survived {self.wave} of {TOTAL_WAVES} waves"

        self.text(title, 54, color, (WIDTH // 2, HEIGHT // 2 - 60), "center", True)
        self.text(sub, 18, C_TEXT, (WIDTH // 2, HEIGHT // 2 + 6), "center")
        self.text(
            "Press R to generate a new maze",
            15,
            C_MUTED,
            (WIDTH // 2, HEIGHT // 2 + 44),
            "center",
        )

    # ------------------------------------------------------------------
    def run(self):
        while self.running:
            real_dt = self.clock.tick(FPS) / 1000.0
            self.handle_events()
            self.update(real_dt)
            self.draw()
        pygame.quit()
