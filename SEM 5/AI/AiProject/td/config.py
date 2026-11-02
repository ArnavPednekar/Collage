COLS = 23
ROWS = 17
TILE = 40

GRID_W = COLS * TILE
GRID_H = ROWS * TILE
HUD_W = 300
WIDTH = GRID_W + HUD_W
HEIGHT = GRID_H
FPS = 60

WALL = 0
FLOOR = 1

START_GOLD = 260
START_LIVES = 20
TOTAL_WAVES = 15

C_BG = (14, 16, 24)
C_PANEL = (22, 25, 37)
C_PANEL_LIGHT = (31, 35, 51)
C_PANEL_EDGE = (48, 54, 76)
C_WALL = (48, 54, 76)
C_WALL_EDGE = (68, 75, 102)
C_FLOOR = (21, 24, 34)
C_FLOOR_ALT = (24, 27, 38)
C_GRID = (32, 36, 50)
C_TEXT = (222, 228, 242)
C_MUTED = (128, 138, 162)
C_ACCENT = (80, 200, 255)
C_PATH = (62, 150, 205)
C_GOOD = (95, 220, 145)
C_BAD = (240, 95, 105)
C_GOLD = (250, 205, 90)

DIRS = ((1, 0), (-1, 0), (0, 1), (0, -1))

TOWER_TYPES = {
    "basic": {
        "name": "Pulse",
        "cost": 60,
        "damage": 12,
        "rate": 1.25,
        "range": 3.0,
        "color": (80, 180, 255),
        "proj_speed": 460,
    },
    "sniper": {
        "name": "Rail",
        "cost": 110,
        "damage": 48,
        "rate": 0.55,
        "range": 6.5,
        "color": (190, 120, 255),
        "proj_speed": 1100,
    },
    "rapid": {
        "name": "Scatter",
        "cost": 85,
        "damage": 6,
        "rate": 4.0,
        "range": 2.2,
        "color": (255, 160, 70),
        "proj_speed": 620,
    },
}

TOWER_ORDER = ("basic", "sniper", "rapid")

ENEMY_TYPES = {
    "grunt": {
        "name": "Grunt",
        "hp": 40,
        "speed": 55,
        "reward": 8,
        "leak": 1,
        "radius": 10,
        "color": (235, 90, 90),
    },
    "runner": {
        "name": "Runner",
        "hp": 22,
        "speed": 105,
        "reward": 8,
        "leak": 1,
        "radius": 8,
        "color": (240, 140, 220),
    },
    "tank": {
        "name": "Tank",
        "hp": 160,
        "speed": 34,
        "reward": 20,
        "leak": 2,
        "radius": 14,
        "color": (200, 70, 60),
    },
    "boss": {
        "name": "Boss",
        "hp": 380,
        "speed": 28,
        "reward": 60,
        "leak": 5,
        "radius": 18,
        "color": (255, 60, 120),
    },
}
