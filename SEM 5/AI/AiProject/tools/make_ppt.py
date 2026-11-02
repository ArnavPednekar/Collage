"""Build the 12-slide Review 1 presentation.

Run with:  .venv/bin/python tools/make_ppt.py
"""

import os

from pptx import Presentation
from pptx.dml.color import RGBColor
from pptx.enum.shapes import MSO_SHAPE
from pptx.enum.text import MSO_ANCHOR, PP_ALIGN
from pptx.util import Inches, Pt

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ASSETS = os.path.join(ROOT, "assets")
OUT = os.path.join(ROOT, "AI_Tower_Defense.pptx")
TOTAL_SLIDES = 12

# palette -----------------------------------------------------------------
BG = "0E1018"
PANEL = "171B28"
PANEL_2 = "1B2030"
CARD = "1A1E2B"
EDGE = "2A3146"
GRID = "151925"
TEXT = "DEE4F2"
MUTED = "9AA4B8"
ACCENT = "50C8FF"
CYAN_DARK = "122430"
PURPLE = "BE78FF"
PURPLE_DARK = "221B33"
GREEN = "5FDC91"
GREEN_DARK = "13281F"
GOLD = "FACD5A"
ORANGE = "FFA046"
RED = "F05F69"
PINK = "F08CDC"
GRAY = "6A7488"

BODY = "Calibri"
HEAD = "Consolas"
MONO = "Consolas"

prs = Presentation()
prs.slide_width = Inches(13.333)
prs.slide_height = Inches(7.5)
BLANK = prs.slide_layouts[6]


def rgb(hex_str):
    return RGBColor.from_string(hex_str)


def add_rect(slide, x, y, w, h, fill=None, line=None, line_w=1.0,
             shape=MSO_SHAPE.ROUNDED_RECTANGLE, radius=0.08):
    shp = slide.shapes.add_shape(
        shape, Inches(x), Inches(y), Inches(w), Inches(h)
    )
    if fill is None:
        shp.fill.background()
    else:
        shp.fill.solid()
        shp.fill.fore_color.rgb = rgb(fill)
    if line is None:
        shp.line.fill.background()
    else:
        shp.line.color.rgb = rgb(line)
        shp.line.width = Pt(line_w)
    shp.shadow.inherit = False
    if shape == MSO_SHAPE.ROUNDED_RECTANGLE:
        try:
            shp.adjustments[0] = radius
        except Exception:
            pass
    return shp


def add_text(slide, x, y, w, h, text, size=14, color=TEXT, bold=False,
             font=BODY, align=PP_ALIGN.LEFT, anchor=MSO_ANCHOR.TOP,
             line_spacing=1.0, italic=False):
    box = slide.shapes.add_textbox(Inches(x), Inches(y), Inches(w), Inches(h))
    tf = box.text_frame
    tf.word_wrap = True
    tf.vertical_anchor = anchor
    tf.margin_left = 0
    tf.margin_right = 0
    tf.margin_top = 0
    tf.margin_bottom = 0
    for i, line in enumerate(text.split("\n")):
        p = tf.paragraphs[0] if i == 0 else tf.add_paragraph()
        p.alignment = align
        p.line_spacing = line_spacing
        run = p.add_run()
        run.text = line
        f = run.font
        f.name = font
        f.size = Pt(size)
        f.bold = bold
        f.italic = italic
        f.color.rgb = rgb(color)
    return box


def add_picture(slide, name, x, y, w=None, h=None):
    path = os.path.join(ASSETS, name)
    kwargs = {"left": Inches(x), "top": Inches(y)}
    if w is not None:
        kwargs["width"] = Inches(w)
    if h is not None:
        kwargs["height"] = Inches(h)
    return slide.shapes.add_picture(path, **kwargs)


def background(slide, grid=False):
    add_rect(slide, 0, 0, 13.333, 7.5, fill=BG, shape=MSO_SHAPE.RECTANGLE)
    if grid:
        step = 0.55
        i = 1
        while i * step < 13.33:
            add_rect(slide, i * step, 0, 0.008, 7.5, fill=GRID,
                     shape=MSO_SHAPE.RECTANGLE)
            i += 1
        j = 1
        while j * step < 7.5:
            add_rect(slide, 0, j * step, 13.333, 0.008, fill=GRID,
                     shape=MSO_SHAPE.RECTANGLE)
            j += 1


def header(slide, kicker, title, number):
    add_rect(slide, 0.55, 0.47, 0.055, 0.16, fill=ACCENT,
             shape=MSO_SHAPE.RECTANGLE)
    add_text(slide, 0.72, 0.44, 10.5, 0.24, kicker, 10, ACCENT, True, MONO)
    add_text(slide, 0.55, 0.70, 12.0, 0.52, title, 28, TEXT, True, HEAD)
    add_rect(slide, 0.55, 1.28, 12.23, 0.012, fill=EDGE,
             shape=MSO_SHAPE.RECTANGLE)
    add_text(slide, 11.6, 0.44, 1.18, 0.22, f"{number:02d} / {TOTAL_SLIDES:02d}",
             9, MUTED, False, MONO, align=PP_ALIGN.RIGHT)


def chip(slide, x, y, w, label, color, fill, dot=True, h=0.36, size=10.5):
    add_rect(slide, x, y, w, h, fill=fill, line=color, radius=0.5)
    text_x = x + 0.12
    if dot:
        add_rect(slide, x + 0.13, y + h / 2 - 0.045, 0.09, 0.09,
                 fill=color, shape=MSO_SHAPE.OVAL)
        text_x = x + 0.3
    add_text(slide, text_x, y, w - (text_x - x) - 0.08, h, label, size,
             TEXT, True, MONO, align=PP_ALIGN.LEFT, anchor=MSO_ANCHOR.MIDDLE)


def bullets(slide, x, y, w, items, size=13, gap=0.56, dot=ACCENT,
            body_color=MUTED, lead_color=TEXT):
    for i, (lead, body) in enumerate(items):
        yy = y + i * gap
        add_rect(slide, x, yy + 0.085, 0.085, 0.085, fill=dot,
                 shape=MSO_SHAPE.OVAL)
        box = slide.shapes.add_textbox(
            Inches(x + 0.22), Inches(yy - 0.02), Inches(w - 0.22),
            Inches(gap)
        )
        tf = box.text_frame
        tf.word_wrap = True
        tf.margin_left = 0
        tf.margin_right = 0
        tf.margin_top = 0
        tf.margin_bottom = 0
        p = tf.paragraphs[0]
        p.line_spacing = 1.05
        r1 = p.add_run()
        r1.text = lead + "  "
        r1.font.name = BODY
        r1.font.size = Pt(size)
        r1.font.bold = True
        r1.font.color.rgb = rgb(lead_color)
        r2 = p.add_run()
        r2.text = body
        r2.font.name = BODY
        r2.font.size = Pt(size)
        r2.font.color.rgb = rgb(body_color)


def step_card(slide, x, y, w, h, num, num_color, title, body):
    add_rect(slide, x, y, w, h, fill=CARD, line=EDGE, radius=0.07)
    add_text(slide, x + 0.2, y + 0.14, 0.5, 0.3, num, 15, num_color, True, MONO)
    add_text(slide, x + 0.75, y + 0.13, w - 0.95, 0.3, title, 15, TEXT, True,
             BODY)
    add_text(slide, x + 0.75, y + 0.45, w - 0.95, h - 0.52, body, 11.5,
             MUTED, line_spacing=1.05)


def feature_card(slide, x, y, w, h, title, body, accent=ACCENT):
    add_rect(slide, x, y, w, h, fill=CARD, line=EDGE, radius=0.07)
    add_rect(slide, x, y + 0.14, 0.05, h - 0.28, fill=accent,
             shape=MSO_SHAPE.RECTANGLE)
    add_text(slide, x + 0.22, y + 0.12, w - 0.4, 0.28, title, 13.5, TEXT,
             True, BODY)
    add_text(slide, x + 0.22, y + 0.4, w - 0.4, h - 0.46, body, 10.5, MUTED,
             line_spacing=1.05)


def badge(slide, x, y, n, color=ACCENT, size=0.32):
    add_rect(slide, x - 0.03, y - 0.03, size + 0.06, size + 0.06, fill=BG,
             shape=MSO_SHAPE.OVAL)
    add_rect(slide, x, y, size, size, fill=color, shape=MSO_SHAPE.OVAL)
    add_text(slide, x, y, size, size, str(n), 10.5, BG, True, MONO,
             align=PP_ALIGN.CENTER, anchor=MSO_ANCHOR.MIDDLE)


def metric(slide, x, y, w, h, value, label, sub, color=ACCENT):
    add_rect(slide, x, y, w, h, fill=CARD, line=EDGE, radius=0.07)
    add_text(slide, x + 0.18, y + 0.1, w - 0.3, 0.4, value, 22, color, True,
             HEAD)
    add_text(slide, x + 0.18, y + 0.55, w - 0.3, 0.22, label, 9.5, TEXT, True,
             BODY)
    add_text(slide, x + 0.18, y + 0.76, w - 0.3, 0.28, sub, 8.5, MUTED, False,
             BODY)


# =========================================================================
# 01 — TITLE / MEMBERS
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide, grid=True)

add_picture(slide, "hero_maze.png", 7.62, 0.48, h=6.55)
add_rect(slide, 7.56, 0.42, 5.45, 6.67, line="242B40", line_w=1.0,
         shape=MSO_SHAPE.RECTANGLE)

add_rect(slide, 0.8, 0.98, 4.35, 0.36, fill=CYAN_DARK, line="2A6478",
         radius=0.5)
add_text(slide, 0.8, 0.98, 4.35, 0.36,
         "SEMESTER 5  ·  ARTIFICIAL INTELLIGENCE", 10, ACCENT, True, MONO,
         align=PP_ALIGN.CENTER, anchor=MSO_ANCHOR.MIDDLE)

add_text(slide, 0.8, 1.52, 6.4, 0.9, "AI TOWER", 58, TEXT, True, HEAD)
add_text(slide, 0.8, 2.34, 6.4, 0.9, "DEFENSE", 58, ACCENT, True, HEAD)
add_text(slide, 0.8, 3.52, 6.3, 0.95,
         "The AI builds the maze.\nThe swarm finds the path.", 18.5, MUTED,
         line_spacing=1.18)
add_rect(slide, 0.8, 4.68, 5.5, 0.012, fill=EDGE, shape=MSO_SHAPE.RECTANGLE)

chip(slide, 0.8, 4.92, 2.0, "Randomized DFS", PURPLE, PURPLE_DARK)
chip(slide, 2.95, 4.92, 1.55, "A* Search", ACCENT, CYAN_DARK)
chip(slide, 4.65, 4.92, 1.95, "Live Repathing", GREEN, GREEN_DARK)

add_rect(slide, 0.8, 5.55, 6.35, 0.85, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 0.98, 5.63, 2.0, 0.2, "TEAM", 8.5, ACCENT, True, MONO)
add_text(slide, 0.98, 5.82, 5.95, 0.26,
         "[Member 1]   ·   [Member 2]   ·   [Member 3]", 13, TEXT, True, BODY)
add_text(slide, 0.98, 6.1, 5.95, 0.22,
         "Roll numbers: [add your roll numbers here]", 9.5, MUTED, False, MONO)

add_text(slide, 0.8, 6.8, 6.4, 0.26,
         "Review 1  ·  Python + Pygame  ·  15 waves  ·  3 towers  ·  4 enemies",
         10, MUTED, False, MONO)

# =========================================================================
# 02 — INTRODUCTION
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "01  ·  INTRODUCTION", "A tower defense game with an AI-built map", 2)

add_text(slide, 0.55, 1.52, 7.1, 1.15,
         "Tower defense is a strategy game: enemies march along a path while "
         "the player builds towers to stop them. Most games ship hand-made "
         "maps, so the layout is predictable.", 15, TEXT, line_spacing=1.22)
add_text(slide, 0.55, 2.72, 7.1, 1.15,
         "Here the map itself is generated by an algorithm, and every enemy "
         "navigates it with A* search. Towers change the graph, so the AI has "
         "to re-plan while the game is running.", 15, MUTED, line_spacing=1.22)

pillars = [
    ("GENERATE", "Randomized DFS carves a fresh maze and braids it into loops.",
     PURPLE),
    ("SEARCH", "A* finds the shortest route for every enemy on the live grid.",
     ACCENT),
    ("REACT", "Towers mutate the graph; enemies re-route mid-wave.", GREEN),
]
px = 0.55
for title, body, color in pillars:
    add_rect(slide, px, 4.15, 2.2, 1.35, fill=CARD, line=EDGE, radius=0.07)
    add_rect(slide, px, 4.15, 2.2, 0.05, fill=color,
             shape=MSO_SHAPE.RECTANGLE)
    add_text(slide, px + 0.18, 4.3, 1.85, 0.25, title, 10.5, color, True, MONO)
    add_text(slide, px + 0.18, 4.6, 1.85, 0.8, body, 10.5, MUTED,
             line_spacing=1.05)
    px += 2.38

add_rect(slide, 7.95, 1.52, 4.85, 4.55, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 8.15, 1.68, 4.4, 0.24, "WHY IT IS AN AI PROJECT", 10, ACCENT,
         True, MONO)
bullets(slide, 8.15, 2.02, 4.45, [
    ("Generation", "Randomized depth-first search + wall braiding."),
    ("Heuristic search", "A* with a Manhattan distance estimate."),
    ("Re-planning", "The graph changes whenever a tower is built."),
    ("Decision rule", "Towers target the enemy closest to leaking."),
], size=12, gap=0.66, dot=PURPLE)
add_rect(slide, 8.15, 4.95, 4.45, 0.012, fill=EDGE,
         shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 8.15, 5.12, 4.45, 0.8,
         "The player changes the problem; the AI solves it again.", 12.5,
         TEXT, True, BODY, line_spacing=1.1)

add_rect(slide, 0.55, 6.25, 12.25, 0.72, fill="141B29", line="2A4964",
         radius=0.08)
add_text(slide, 0.85, 6.25, 11.65, 0.72,
         "PROJECT GOAL   ·   Demonstrate generation and heuristic search in a "
         "tower defense game a human can actually play.", 12, TEXT, False, BODY,
         anchor=MSO_ANCHOR.MIDDLE)

# =========================================================================
# 03 — GAME OVERVIEW
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "02  ·  GAME OVERVIEW", "Build the maze. Break the swarm.", 3)

add_rect(slide, 0.63, 1.60, 7.15, 3.99, fill="07090F", radius=0.02)
add_picture(slide, "gameplay.png", 0.55, 1.52, w=7.15)
add_rect(slide, 0.55, 1.52, 7.15, 3.99, fill=None, line="2B3348",
         line_w=1.0, shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 0.55, 5.58, 7.15, 0.25,
         "Procedural maze  ·  11 towers  ·  wave 8 of 15", 10.5, MUTED, False,
         MONO)

add_text(slide, 8.05, 1.52, 4.7, 0.24, "WHAT YOU DO", 10, ACCENT, True, MONO)
bullets(slide, 8.05, 1.82, 4.75, [
    ("Place", "Pulse, Rail and Scatter towers on any open tile."),
    ("Block", "routes to force enemies into longer, deadlier paths."),
    ("Survive", "15 escalating waves without losing your 20 lives."),
], size=12.5, gap=0.54, dot=ACCENT)

add_rect(slide, 8.05, 3.48, 4.7, 0.012, fill=EDGE,
         shape=MSO_SHAPE.RECTANGLE)

add_text(slide, 8.05, 3.62, 4.7, 0.24, "WHAT THE AI DOES", 10, PURPLE, True,
         MONO)
bullets(slide, 8.05, 3.92, 4.75, [
    ("Generates", "a brand-new maze and wave set every run."),
    ("Routes", "every enemy with A* over the live grid."),
    ("Re-plans", "instantly whenever a tower changes the graph."),
], size=12.5, gap=0.54, dot=PURPLE)

add_rect(slide, 8.05, 5.68, 4.7, 0.78, fill="201A2E", line="4A3A68",
         radius=0.08)
add_rect(slide, 8.05, 5.8, 0.05, 0.54, fill=PURPLE,
         shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 8.25, 5.78, 4.4, 0.6,
         "Every tower you place is a change to the graph the AI must solve.",
         11, TEXT, False, BODY, anchor=MSO_ANCHOR.MIDDLE)

flow = [
    ("01", "Generate maze", ACCENT),
    ("02", "Build towers", GREEN),
    ("03", "A* routes swarm", PURPLE),
    ("04", "Survive 15 waves", GOLD),
]
fx = 0.55
for i, (num, label, color) in enumerate(flow):
    add_rect(slide, fx, 6.62, 2.82, 0.62, fill=PANEL, line=EDGE, radius=0.1)
    add_text(slide, fx + 0.16, 6.7, 0.4, 0.24, num, 9.5, color, True, MONO)
    add_text(slide, fx + 0.55, 6.68, 2.1, 0.5, label, 11.5, TEXT, True, BODY,
             anchor=MSO_ANCHOR.MIDDLE)
    if i < 3:
        add_text(slide, fx + 2.86, 6.68, 0.22, 0.5, "›", 18, MUTED, True, BODY,
                 anchor=MSO_ANCHOR.MIDDLE)
    fx += 3.07

# =========================================================================
# 04 — GAMEPLAY & RULES
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "03  ·  GAMEPLAY & RULES", "Towers, enemies and 15 waves", 4)

card_positions = [(0.55, 3.85), (4.62, 3.85), (8.69, 3.85)]
for x, w in card_positions:
    add_rect(slide, x, 1.5, w, 2.7, fill=CARD, line=EDGE, radius=0.06)

# towers
tx, tw = card_positions[0]
add_text(slide, tx + 0.2, 1.64, tw - 0.4, 0.24, "TOWERS  ·  3 TYPES", 10,
         ACCENT, True, MONO)
towers = [
    ("Pulse", "60g", "12 dmg · 1.25/s · 3.0 tiles", ACCENT),
    ("Rail", "110g", "48 dmg · 0.55/s · 6.5 tiles", PURPLE),
    ("Scatter", "85g", "6 dmg · 4.00/s · 2.2 tiles", ORANGE),
]
for i, (name, cost, stats, color) in enumerate(towers):
    y = 1.98 + i * 0.66
    add_rect(slide, tx + 0.2, y + 0.1, 0.12, 0.12, fill=color,
             shape=MSO_SHAPE.OVAL)
    add_text(slide, tx + 0.42, y, 1.7, 0.26, name, 12.5, TEXT, True, BODY)
    add_text(slide, tx + tw - 0.9, y, 0.7, 0.26, cost, 11, GOLD, True, MONO,
             align=PP_ALIGN.RIGHT)
    add_text(slide, tx + 0.42, y + 0.26, tw - 0.65, 0.25, stats, 9.5, MUTED,
             False, MONO)

# enemies
ex, ew = card_positions[1]
add_text(slide, ex + 0.2, 1.64, ew - 0.4, 0.24, "ENEMIES  ·  4 TYPES", 10,
         PURPLE, True, MONO)
enemies = [
    ("Grunt", "wave 1", "40 hp · 55 px/s", RED),
    ("Runner", "wave 3", "22 hp · 105 px/s", PINK),
    ("Tank", "wave 5", "160 hp · 34 px/s", ORANGE),
    ("Boss", "every 5", "380 hp · 28 px/s", PURPLE),
]
for i, (name, wave, stats, color) in enumerate(enemies):
    y = 1.98 + i * 0.51
    add_rect(slide, ex + 0.2, y + 0.09, 0.1, 0.1, fill=color,
             shape=MSO_SHAPE.OVAL)
    add_text(slide, ex + 0.4, y, 1.3, 0.24, name, 11.5, TEXT, True, BODY)
    add_text(slide, ex + ew - 1.05, y, 0.85, 0.24, wave, 9, color, True, MONO,
             align=PP_ALIGN.RIGHT)
    add_text(slide, ex + 0.4, y + 0.23, ew - 0.6, 0.22, stats, 9, MUTED, False,
             MONO)

# rules
rx, rw = card_positions[2]
add_text(slide, rx + 0.2, 1.64, rw - 0.4, 0.24, "RULES", 10, GREEN, True, MONO)
rules = [
    "20 lives — lose them all and the base falls.",
    "15 waves, each stronger than the last.",
    "Gold from kills and wave clears.",
    "Sell a tower for 60% of its cost.",
    "A build that seals the goal is rejected.",
    "Speed toggle: 1×, 2×, 3×.",
]
for i, line in enumerate(rules):
    add_rect(slide, rx + 0.2, 2.04 + i * 0.32, 0.07, 0.07, fill=GREEN,
             shape=MSO_SHAPE.OVAL)
    add_text(slide, rx + 0.38, 2.0 + i * 0.32, rw - 0.55, 0.26, line, 9.5,
             MUTED, False, BODY)

# wave scaling panel
add_rect(slide, 0.55, 4.35, 6.2, 2.35, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 0.75, 4.49, 4.0, 0.22, "WAVE SCALING", 10, GOLD, True, MONO)
add_text(slide, 0.75, 4.72, 4.8, 0.2,
         "Enemy HP multiplier, wave 1 → 15", 9, MUTED, False, BODY)
base_y = 6.12
chart_x = 0.9
chart_w = 5.55
bar_w = 0.23
gap = (chart_w - 15 * bar_w) / 14
for i in range(15):
    wave = i + 1
    mult = 1.0 + 0.13 * (wave - 1)
    height = 0.22 + 0.9 * (mult - 1.0) / 1.82
    color = GOLD if wave == 15 else ACCENT
    add_rect(slide, chart_x + i * (bar_w + gap), base_y - height, bar_w,
             height, fill=color, shape=MSO_SHAPE.RECTANGLE)
add_rect(slide, 0.85, base_y + 0.01, chart_w + 0.1, 0.01, fill=EDGE,
         shape=MSO_SHAPE.RECTANGLE)
for wave in (1, 5, 10, 15):
    bx = chart_x + (wave - 1) * (bar_w + gap) + bar_w / 2
    add_text(slide, bx - 0.3, base_y + 0.06, 0.6, 0.18, str(wave), 8, MUTED,
             False, MONO, align=PP_ALIGN.CENTER)
add_text(slide, 4.5, 4.72, 2.0, 0.2, "×2.8 at wave 15", 9, GOLD, True, MONO,
         align=PP_ALIGN.RIGHT)

# controls panel
add_rect(slide, 6.95, 4.35, 5.83, 2.35, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 7.15, 4.49, 4.0, 0.22, "CONTROLS", 10, GREEN, True, MONO)
controls = [
    ("1 / 2 / 3", "select tower type"),
    ("Click", "build on a valid tile"),
    ("Right-click", "sell the tower"),
    ("Space", "start the next wave"),
    ("F  /  P", "speed  /  toggle A* path"),
    ("R", "generate a new maze"),
]
for i, (key, action) in enumerate(controls):
    y = 4.85 + i * 0.29
    add_text(slide, 7.15, y, 1.35, 0.24, key, 10, ACCENT, True, MONO)
    add_text(slide, 8.55, y, 4.0, 0.24, action, 10, MUTED, False, BODY)

# =========================================================================
# 05 — AI ALGORITHM 1: MAZE GENERATION
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "04  ·  AI  ·  MAZE GENERATION", "Algorithm 1 — The maze builds itself", 5)

step_card(slide, 0.55, 1.52, 6.1, 1.42, "01", ACCENT,
          "Randomized DFS carve",
          "A stack-based recursive backtracker visits every cell once, carving "
          "corridors and backtracking at dead ends. Linear time in the number "
          "of tiles.")
step_card(slide, 0.55, 3.09, 6.1, 1.42, "02", PURPLE,
          "Braid the walls",
          "32% of the remaining interior walls are removed, turning dead ends "
          "into loops. Multiple routes are what make blocking a real decision.")
step_card(slide, 0.55, 4.66, 6.1, 1.42, "03", GREEN,
          "Anchor spawn and goal",
          "Spawn sits on the left edge, the goal on the right. A* verifies the "
          "route before the level goes live, and a seed reproduces any maze.")

add_rect(slide, 6.94, 1.52, 5.75, 4.25, fill="090B11", radius=0.02)
add_picture(slide, "maze_output.png", 6.86, 1.45, w=5.9)
add_rect(slide, 6.86, 1.45, 5.9, 4.36, fill=None, line="2B3348",
         line_w=1.0, shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 6.86, 5.92, 5.9, 0.24,
         "Generated output  ·  23 × 17 tiles  ·  A* route in cyan", 10, MUTED,
         False, MONO)

chip(slide, 6.86, 6.27, 1.78, "23 × 17 grid", ACCENT, CYAN_DARK, h=0.34,
     size=10)
chip(slide, 8.76, 6.27, 1.78, "32% braided", PURPLE, PURPLE_DARK, h=0.34,
     size=10)
chip(slide, 10.66, 6.27, 2.1, "path guaranteed", GREEN, GREEN_DARK, h=0.34,
     size=10)
add_text(slide, 6.86, 6.75, 5.9, 0.24,
         "A new seed is drawn every run — press R to regenerate.",
         10, MUTED, False, BODY)

# =========================================================================
# 06 — AI ALGORITHM 2: A*
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "04  ·  AI  ·  PATHFINDING", "Algorithm 2 — A* finds the way", 6)

add_rect(slide, 0.55, 1.5, 6.1, 1.02, fill="141B29", line="2A4964",
         radius=0.08)
add_text(slide, 0.7, 1.58, 5.8, 0.42, "f(n) = g(n) + h(n)", 25, TEXT, True,
         MONO, align=PP_ALIGN.CENTER)
add_text(slide, 0.7, 2.06, 5.8, 0.22,
         "g: cost from spawn      ·      h: Manhattan estimate to goal",
         10, MUTED, False, MONO, align=PP_ALIGN.CENTER)

feature_card(slide, 0.55, 2.68, 6.1, 1.0, "Frontier as a min-heap",
             "The lowest f-score expands first; a closed set prevents "
             "re-expansion. 4-connected grid, uniform step cost.")
feature_card(slide, 0.55, 3.86, 6.1, 1.0, "Towers edit the graph",
             "Every build adds a blocked cell. Enemies re-run A* from their "
             "current cell; builds that would seal the goal are rejected.",
             accent=PURPLE)
feature_card(slide, 0.55, 5.04, 6.1, 1.0, "Smart targeting",
             "Towers prefer the enemy with the shortest remaining path — the "
             "one about to leak, not merely the closest.", accent=GREEN)

chip(slide, 0.55, 6.26, 2.25, "O(E log V) per search", ACCENT, CYAN_DARK,
     h=0.34, size=10)
chip(slide, 2.95, 6.26, 2.45, "0.15 ms average", GREEN, GREEN_DARK,
     h=0.34, size=10)

add_rect(slide, 6.98, 1.52, 5.75, 3.5, fill="090B11", radius=0.02)
add_picture(slide, "astar_demo.png", 6.9, 1.45, w=5.9)
add_rect(slide, 6.9, 1.45, 5.9, 3.56, fill=None, line="2B3348", line_w=1.0,
         shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 6.9, 5.15, 5.9, 0.24,
         "Explored nodes in blue  ·  optimal route in cyan", 10, MUTED, False,
         MONO)

legend = [
    ("Wall", "384060"),
    ("Explored", "3A6A8F"),
    ("Route", ACCENT),
]
lx = 6.9
for label, color in legend:
    add_rect(slide, lx, 5.62, 0.12, 0.12, fill=color, shape=MSO_SHAPE.RECTANGLE)
    add_text(slide, lx + 0.2, 5.56, 1.3, 0.24, label, 10, MUTED, False, MONO)
    lx += 1.9

add_rect(slide, 6.9, 6.02, 5.9, 0.72, fill="1F1A2E", line="4A3A68",
         radius=0.08)
add_rect(slide, 6.9, 6.12, 0.05, 0.52, fill=PURPLE, shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 7.1, 6.08, 5.55, 0.6,
         "Live repathing is the payoff: the maze is not static — the AI reacts "
         "to you.", 11, TEXT, False, BODY, anchor=MSO_ANCHOR.MIDDLE)

# =========================================================================
# 07 — AI ALGORITHM 3: DYNAMIC REPATHING & TARGETING
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "04  ·  AI  ·  ADAPTATION", "Algorithm 3 — The map fights back", 7)

add_text(slide, 0.55, 1.5, 6.2, 0.45,
         "The maze is not static: every tower changes the graph the swarm "
         "must solve.", 13, MUTED, line_spacing=1.1)

feature_card(slide, 0.55, 2.1, 6.2, 0.95, "Graph mutation",
             "A tower becomes a blocked cell. Each enemy re-runs A* from its "
             "current cell, so the swarm adapts in the middle of a wave.",
             accent=PURPLE)
feature_card(slide, 0.55, 3.15, 6.2, 0.95, "Build validation",
             "Before charging the player, the game searches with the new cell "
             "included. If the goal becomes unreachable, the build is rejected.",
             accent=ACCENT)
feature_card(slide, 0.55, 4.2, 6.2, 0.95, "Target selection",
             "Towers fire at the enemy with the smallest remaining path "
             "distance — the next one to leak, not merely the closest.",
             accent=GREEN)

chip(slide, 0.55, 5.42, 1.95, "re-path on build", PURPLE,
     PURPLE_DARK, h=0.34, size=9)
chip(slide, 2.62, 5.42, 2.0, "goal stays reachable", ACCENT, CYAN_DARK,
     h=0.34, size=9)
chip(slide, 4.74, 5.42, 1.95, "no frame flooding", GREEN, GREEN_DARK,
     h=0.34, size=9)

add_rect(slide, 7.02, 1.52, 5.75, 4.35, fill="090B11", radius=0.02)
add_picture(slide, "repath_demo.png", 6.94, 1.45, w=5.9)
add_rect(slide, 6.94, 1.45, 5.9, 4.36, fill=None, line="2B3348",
         line_w=1.0, shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 6.94, 5.92, 5.9, 0.24,
         "Gray: old route   ·   Cyan: new route after one tower (orange)",
         9.5, MUTED, False, MONO)
add_text(slide, 6.94, 6.24, 5.9, 0.24,
         "This example: path length 31 → 59 tiles.", 10, GOLD, True, MONO)

# =========================================================================
# 08 — TECHNOLOGY USED
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "05  ·  TECHNOLOGY", "Python, Pygame and a hand-written A*", 8)

# modules panel
add_rect(slide, 0.55, 1.5, 6.6, 3.0, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 0.75, 1.63, 4.0, 0.22, "PROJECT STRUCTURE", 10, ACCENT, True,
         MONO)
modules = [
    ("main.py", "entry point"),
    ("td/game.py", "loop, waves, HUD, input"),
    ("td/maze.py", "randomized DFS + braiding"),
    ("td/pathfinding.py", "A* search over the grid"),
    ("td/entities.py", "enemies, towers, projectiles"),
]
for i, (name, role) in enumerate(modules):
    y = 2.02 + i * 0.47
    add_text(slide, 0.75, y, 0.3, 0.22, f"{i + 1:02d}", 9, MUTED, True, MONO)
    add_text(slide, 1.12, y, 1.65, 0.22, name, 10.5, ACCENT, True, MONO)
    add_text(slide, 2.85, y, 4.1, 0.22, role, 10.5, MUTED, False, BODY)
    if i < len(modules) - 1:
        add_rect(slide, 0.75, y + 0.4, 6.2, 0.008, fill=EDGE,
                 shape=MSO_SHAPE.RECTANGLE)

add_text(slide, 0.55, 4.72, 3.0, 0.2, "DATA STRUCTURES", 9, MUTED, True, MONO)
for i, label in enumerate(("2D grid", "blocked set", "heapq", "dict + list")):
    chip(slide, 0.55 + i * 1.6, 4.98, 1.5, label, ACCENT, CYAN_DARK, dot=False,
         h=0.34, size=9)
add_text(slide, 0.55, 5.55, 6.4, 0.5,
         "Every algorithm is written from scratch — no ML or pathfinding "
         "libraries.", 11, MUTED, False, BODY, line_spacing=1.1)
chip(slide, 0.55, 5.95, 2.9, "O(W·H) maze generation", PURPLE, PURPLE_DARK,
     h=0.34, size=9.5)
chip(slide, 3.6, 5.95, 2.9, "O(E log V) per A* search", GREEN, GREEN_DARK,
     h=0.34, size=9.5)

# stack panel
add_rect(slide, 7.4, 1.5, 5.38, 3.0, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 7.6, 1.63, 4.0, 0.22, "STACK", 10, GREEN, True, MONO)
stack = [
    ("Python 3.14", "language"),
    ("pygame-ce 2.5", "rendering, input, audio"),
    ("heapq", "A* priority queue"),
    ("random", "seeded generation"),
    ("python-pptx", "this presentation"),
]
for i, (name, role) in enumerate(stack):
    y = 2.02 + i * 0.47
    add_text(slide, 7.6, y, 2.0, 0.22, name, 10.5, ACCENT, True, MONO)
    add_text(slide, 9.75, y, 2.8, 0.22, role, 10.5, MUTED, False, BODY)

# run panel
add_rect(slide, 7.4, 4.72, 5.38, 1.98, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 7.6, 4.86, 3.0, 0.22, "RUN IT", 10, GOLD, True, MONO)
add_rect(slide, 7.58, 5.12, 5.02, 0.86, fill="0C1018", line="252C3E",
         radius=0.04)
code = [
    "python -m venv .venv",
    ".venv/bin/pip install -r requirements.txt",
    ".venv/bin/python main.py",
]
for i, line in enumerate(code):
    add_text(slide, 7.72, 5.18 + i * 0.25, 4.75, 0.22, "$  " + line, 9,
             "C8D2E6", False, MONO)
add_text(slide, 7.6, 6.1, 5.0, 0.22,
         "Requires Python 3.10+ and pygame-ce.", 9.5, MUTED, False, BODY)
add_text(slide, 7.6, 6.36, 5.0, 0.22,
         "Right-click sells; Space starts each wave.", 9.5, MUTED, False, BODY)

# =========================================================================
# 09 — INTERFACE WALKTHROUGH
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "06  ·  INTERFACE", "A rough look at the interface", 9)

add_rect(slide, 0.63, 1.58, 7.7, 4.29, fill="07090F", radius=0.02)
add_picture(slide, "gameplay.png", 0.55, 1.5, w=7.7)
add_rect(slide, 0.55, 1.5, 7.7, 4.29, fill=None, line="2B3348",
         line_w=1.0, shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 0.55, 5.9, 7.7, 0.25,
         "Live gameplay  ·  11 towers  ·  wave 8  ·  5 enemies on the route",
         10, MUTED, False, MONO)

badge(slide, 6.12, 1.56, 1, ACCENT, size=0.26)
badge(slide, 6.12, 2.56, 2, PURPLE, size=0.26)
badge(slide, 3.9, 3.92, 3, GREEN, size=0.26)
badge(slide, 1.16, 5.3, 4, GOLD, size=0.26)

add_text(slide, 8.55, 1.52, 4.2, 0.24, "WHAT YOU ARE LOOKING AT", 10, ACCENT,
         True, MONO)
ui_items = [
    ("1", "HUD", "Gold, lives, wave counter and the tower shop, updated live.",
     ACCENT),
    ("2", "Tower shop", "Pick Pulse, Rail or Scatter; cost and stats are shown.",
     PURPLE),
    ("3", "Grid", "Hover for a range preview; click to build, right-click to sell.",
     GREEN),
    ("4", "Spawn and goal", "Green enters, red leaks. Cyan is the current A* route.",
     GOLD),
]
for i, (num, name, body, color) in enumerate(ui_items):
    y = 1.85 + i * 0.95
    add_text(slide, 8.55, y, 0.3, 0.25, num, 11, color, True, MONO)
    add_text(slide, 8.85, y, 3.8, 0.24, name, 12, TEXT, True, BODY)
    add_text(slide, 8.85, y + 0.25, 3.85, 0.5, body, 10, MUTED,
             line_spacing=1.05)

add_rect(slide, 8.55, 5.5, 4.25, 1.4, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 8.72, 5.62, 3.0, 0.2, "KEYBOARD", 9, GREEN, True, MONO)
add_text(slide, 8.72, 5.88, 3.95, 0.95,
         "1 / 2 / 3   towers\nSpace   next wave\nF   speed      P   path\nR   new maze",
         9.5, MUTED, False, MONO, line_spacing=1.25)

# =========================================================================
# 10 — CHALLENGES
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "07  ·  CHALLENGES", "What went wrong — and how we fixed it", 10)

challenges = [
    ("01", "Enemies walked through walls",
     "A* blocked towers but not wall tiles, so paths cut across the maze.",
     "Build the blocked set from walls + towers when the level loads.", RED),
    ("02", "One tower could seal the goal",
     "A build that looked valid made the level unsolvable.",
     "Run A* with the new cell included and reject the build if it fails.", ORANGE),
    ("03", "Perfect mazes were too easy",
     "A single route meant one tower could stop everything.",
     "Braid ~32% of interior walls so multiple routes stay open.", PURPLE),
    ("04", "Too much braiding felt empty",
     "The maze turned into an open room with a few pillars.",
     "Tune the braid chance and compare seeds visually before locking it.", ACCENT),
    ("05", "Waves were unbalanced",
     "Early waves were trivial and late waves were impossible.",
     "Scale HP by 1 + 0.13(w−1) and tune rewards and wave bonuses.", GREEN),
    ("06", "Re-pathing every frame is wasteful",
     "Recomputing A* continuously would burn CPU with 40+ enemies.",
     "Re-path only when towers change; substep the simulation at 3× speed.",
     GOLD),
]
for i, (num, title, problem, fix, color) in enumerate(challenges):
    col = i % 3
    row = i // 3
    x = 0.55 + col * 4.07
    y = 1.5 + row * 2.0
    add_rect(slide, x, y, 3.85, 1.85, fill=CARD, line=EDGE, radius=0.06)
    add_text(slide, x + 0.18, y + 0.12, 0.4, 0.22, num, 10, color, True, MONO)
    add_text(slide, x + 0.55, y + 0.1, 3.15, 0.28, title, 11.5, TEXT, True,
             BODY)
    add_text(slide, x + 0.18, y + 0.48, 3.5, 0.5, problem, 9.5, MUTED, False,
             BODY, line_spacing=1.02)
    add_text(slide, x + 0.18, y + 1.02, 3.5, 0.7,
             "FIX   " + fix, 9.5, color, False, BODY, line_spacing=1.02)

add_rect(slide, 0.55, 5.72, 12.25, 0.72, fill="141B29", line="2A4964",
         radius=0.08)
add_text(slide, 0.85, 5.72, 11.65, 0.72,
         "LESSON   ·   Validate AI assumptions with the algorithm itself — the "
         "bug was invisible until the path was drawn.", 12, TEXT, False, BODY,
         anchor=MSO_ANCHOR.MIDDLE)

# =========================================================================
# 11 — RESULTS & EVALUATION
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "08  ·  RESULTS", "Measured on 300 generated levels", 11)

metric(slide, 0.55, 1.5, 3.05, 1.15, "300 / 300", "mazes solvable",
       "A* found a spawn → goal route every time", ACCENT)
metric(slide, 3.7, 1.5, 3.05, 1.15, "0.15 ms", "average A* search",
       "0.35 ms worst case on a 23 × 17 grid", GREEN)
metric(slide, 0.55, 2.8, 3.05, 1.15, "29 tiles", "average route length",
       "Range across seeds: 20 – 40 tiles", PURPLE)
metric(slide, 3.7, 2.8, 3.05, 1.15, "99.25%", "of random builds accepted",
       "9 rejected: they would seal the goal", GOLD)

add_rect(slide, 0.55, 4.15, 6.2, 2.35, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 0.75, 4.3, 4.0, 0.22, "VALIDATION CHECKLIST", 10, GREEN, True,
         MONO)
checks = [
    "Every generated seed has a spawn → goal path.",
    "A* re-routes enemies after every accepted build.",
    "Builds that would seal the goal are rejected.",
    "All 15 waves run without simulation errors.",
]
for i, line in enumerate(checks):
    y = 4.68 + i * 0.41
    add_rect(slide, 0.78, y + 0.07, 0.1, 0.1, fill=GREEN, shape=MSO_SHAPE.OVAL)
    add_text(slide, 1.0, y, 5.5, 0.25, line, 11, MUTED, False, BODY)

# difficulty panel
add_rect(slide, 7.0, 1.5, 5.78, 2.65, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 7.2, 1.64, 4.0, 0.22, "DIFFICULTY RAMP", 10, GOLD, True, MONO)
add_text(slide, 7.2, 1.88, 4.9, 0.2,
         "Enemy HP multiplier across 15 waves", 9, MUTED, False, BODY)
d_base = 3.62
d_x = 7.35
d_w = 5.05
d_bar = 0.21
d_gap = (d_w - 15 * d_bar) / 14
for i in range(15):
    wave = i + 1
    mult = 1.0 + 0.13 * (wave - 1)
    height = 0.22 + 0.9 * (mult - 1.0) / 1.82
    color = GOLD if wave == 15 else ACCENT
    add_rect(slide, d_x + i * (d_bar + d_gap), d_base - height, d_bar, height,
             fill=color, shape=MSO_SHAPE.RECTANGLE)
add_rect(slide, 7.3, d_base + 0.01, d_w + 0.1, 0.01, fill=EDGE,
         shape=MSO_SHAPE.RECTANGLE)
for wave in (1, 5, 10, 15):
    bx = d_x + (wave - 1) * (d_bar + d_gap) + d_bar / 2
    add_text(slide, bx - 0.3, d_base + 0.06, 0.6, 0.18, str(wave), 8, MUTED,
             False, MONO, align=PP_ALIGN.CENTER)
add_text(slide, 11.55, 2.14, 0.95, 0.2, "×2.8", 9.5, GOLD, True, MONO,
         align=PP_ALIGN.CENTER)

add_rect(slide, 7.0, 4.3, 5.78, 2.2, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 7.2, 4.44, 4.0, 0.22, "WHAT IT SHOWS", 10, PURPLE, True, MONO)
bullets(slide, 7.2, 4.76, 5.35, [
    ("Generation is cheap", "0.16 ms per maze on average."),
    ("A* is fast enough", "to re-run inside the game loop."),
    ("Validation makes", "dynamic tower placement safe."),
], size=10.5, gap=0.5, dot=PURPLE)

add_text(slide, 0.55, 6.72, 12.0, 0.24,
         "Measured with Python 3.14 on a 23 × 17 grid, 300 seeds, default "
         "difficulty.", 9.5, MUTED, False, MONO)

# =========================================================================
# 12 — CONCLUSION & FUTURE WORK
# =========================================================================
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "09  ·  CONCLUSION", "What we learned & what comes next", 12)

add_rect(slide, 0.55, 1.5, 6.2, 1.15, fill="1F1A2E", line="4A3A68",
         radius=0.08)
add_rect(slide, 0.55, 1.6, 0.05, 0.95, fill=PURPLE, shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 0.8, 1.58, 5.75, 0.98,
         "A small rule set — DFS, A* and validation — produces a replayable "
         "strategy game where the AI reacts to the player.", 14.5, TEXT, True,
         BODY, anchor=MSO_ANCHOR.MIDDLE, line_spacing=1.1)

conclusions = [
    ("Generation", "turns one map into endless levels."),
    ("Search", "A* is fast enough to run inside the loop."),
    ("Validation", "is what makes dynamic building safe."),
    ("Lesson", "heuristics + constraints beat hand-tuning."),
]
bullets(slide, 0.55, 3.0, 6.2, conclusions, size=12.5, gap=0.66, dot=GREEN)

add_text(slide, 7.1, 1.5, 3.0, 0.22, "FUTURE WORK", 10, PURPLE, True, MONO)
future = [
    ("GA levels", "Evolve mazes against a simulated player."),
    ("Adaptive waves", "Adjust difficulty to the player's skill."),
    ("Learned agent", "Q-learning for automatic tower placement."),
    ("Polish", "Animations, sound effects and a web port."),
]
for i, (title, body) in enumerate(future):
    col = i % 2
    row = i // 2
    x = 7.1 + col * 2.75
    y = 1.8 + row * 1.55
    add_rect(slide, x, y, 2.6, 1.4, fill=CARD, line=EDGE, radius=0.06)
    add_text(slide, x + 0.16, y + 0.14, 2.3, 0.24, title, 11.5, TEXT, True,
             BODY)
    add_text(slide, x + 0.16, y + 0.44, 2.3, 0.85, body, 9.5, MUTED,
             line_spacing=1.05)

add_rect(slide, 0.55, 5.65, 12.25, 1.0, fill="141B29", line="2A4964",
         radius=0.08)
add_text(slide, 0.85, 5.72, 11.65, 0.42, "Thank you — questions?", 24, ACCENT,
         True, HEAD, align=PP_ALIGN.CENTER)
add_text(slide, 0.85, 6.16, 11.65, 0.25,
         "Semester 5  ·  Artificial Intelligence  ·  Review 1", 10, MUTED,
         False, MONO, align=PP_ALIGN.CENTER)

# =========================================================================
# speaker notes
# =========================================================================
notes = [
    "Title slide. Introduce the team and the project: a tower defense game "
    "where the map is generated by an algorithm and the enemies navigate it "
    "with A*. Replace the member placeholders before submitting.",
    "The game is a standard tower defense on the surface, but the map is not "
    "hand-designed. We use generation and heuristic search, and the graph "
    "changes while the player is playing. The goal is to show those algorithms "
    "in something playable.",
    "This is the core loop. The player places towers; the AI generates the "
    "maze, routes every enemy with A*, and re-plans whenever a tower changes "
    "the graph. Twenty lives, fifteen waves.",
    "The rules are deliberately simple so the AI is the focus. Three tower "
    "types, four enemy types, fifteen waves. Difficulty scales with HP and "
    "wave composition; the player can pause planning between waves.",
    "Maze generation uses randomized DFS, a recursive backtracker. It carves a "
    "perfect maze in linear time, then we braid about 32 percent of interior "
    "walls so there are loops. Loops are essential: with a single path, one "
    "tower could block everything. Spawn and goal are anchored on opposite "
    "edges and A* verifies the route.",
    "A* uses f = g + h. g is the cost from spawn and h is the Manhattan "
    "distance to the goal. The frontier is a min-heap, so the lowest f expands "
    "first. Towers are blocked cells. Before a build is accepted we run A* "
    "with the new cell; if the goal becomes unreachable, the build is rejected.",
    "This is the dynamic part. A tower mutates the graph, and each enemy "
    "re-runs A* from its current cell. In the example, blocking one cell "
    "changes the route from 31 tiles to 59. Towers also use a targeting "
    "heuristic: they fire at the enemy with the smallest remaining path.",
    "The implementation is Python with pygame-ce. Everything is written from "
    "scratch: DFS, A*, the priority queue usage and the game loop. The project "
    "is split into small modules, and the only third-party dependency is "
    "pygame-ce.",
    "This is the interface. The HUD shows gold, lives and wave. The tower shop "
    "shows costs and stats. The grid previews range and validates builds. The "
    "cyan line is the current A* route between spawn and goal.",
    "These are the real problems we hit. The biggest was that A* initially "
    "ignored walls — enemies walked straight through the maze. The fix was to "
    "build the blocked set from walls plus towers. Path validation came next, "
    "so a build can never make the level unsolvable.",
    "These numbers come from the actual implementation: 300 seeds, all "
    "solvable; A* averages 0.15 milliseconds; average route is 29 tiles; and "
    "of 1,200 random placements, 9 were rejected because they would have "
    "sealed the goal.",
    "To conclude: generation gives replayability, A* gives correct routing, "
    "and validation makes dynamic changes safe. Future work includes evolving "
    "levels with a genetic algorithm, adaptive difficulty, and a learned tower "
    "placement agent. Thank you — questions?",
]
for slide, note in zip(prs.slides, notes):
    slide.notes_slide.notes_text_frame.text = note

prs.core_properties.title = "AI Tower Defense — Review 1"
prs.core_properties.author = "AI Project"
prs.save(OUT)
print("saved", OUT, "with", len(prs.slides), "slides")
