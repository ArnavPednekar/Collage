"""Build the 8-slide Review 1 presentation.

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
TOTAL_SLIDES = 8

BG = "0E1018"
PANEL = "171B28"
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


def metric(slide, x, y, w, h, value, label, sub, color=ACCENT):
    add_rect(slide, x, y, w, h, fill=CARD, line=EDGE, radius=0.07)
    add_text(slide, x + 0.18, y + 0.1, w - 0.3, 0.4, value, 22, color, True,
             HEAD)
    add_text(slide, x + 0.18, y + 0.55, w - 0.3, 0.22, label, 9.5, TEXT, True,
             BODY)
    add_text(slide, x + 0.18, y + 0.76, w - 0.3, 0.28, sub, 8.5, MUTED, False,
             BODY)


# 01 — TITLE
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
         "Maze generation. A* search.\nLive re-planning.", 18.5, MUTED,
         line_spacing=1.18)
add_rect(slide, 0.8, 4.68, 5.5, 0.012, fill=EDGE, shape=MSO_SHAPE.RECTANGLE)

chip(slide, 0.8, 4.92, 2.0, "Randomized DFS", PURPLE, PURPLE_DARK)
chip(slide, 2.95, 4.92, 1.55, "A* Search", ACCENT, CYAN_DARK)
chip(slide, 4.65, 4.92, 1.95, "Live Re-pathing", GREEN, GREEN_DARK)

add_rect(slide, 0.8, 5.55, 6.35, 0.85, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 0.98, 5.63, 2.0, 0.2, "TEAM", 8.5, ACCENT, True, MONO)
add_text(slide, 0.98, 5.82, 5.95, 0.26,
         "[Member 1]   ·   [Member 2]   ·   [Member 3]", 13, TEXT, True, BODY)
add_text(slide, 0.98, 6.1, 5.95, 0.22,
         "Roll numbers: [add your roll numbers here]", 9.5, MUTED, False, MONO)

add_text(slide, 0.8, 6.8, 6.4, 0.26,
         "Review 1  ·  Rust + Bevy  ·  algorithms first", 10, MUTED, False, MONO)

# 02 — PROBLEM
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "01  ·  PROBLEM", "The map is a graph that keeps changing", 2)

add_text(slide, 0.55, 1.52, 7.1, 1.15,
         "Enemies need a route through a grid. The player blocks cells by "
         "building towers, so the graph changes during play.", 15, TEXT,
         line_spacing=1.22)
add_text(slide, 0.55, 2.72, 7.1, 1.15,
         "The project is built around three algorithms: generate a solvable "
         "maze, search it with A*, and re-plan whenever the graph changes.",
         15, MUTED, line_spacing=1.22)

pillars = [
    ("GENERATE", "Randomized DFS carves a connected maze; braiding adds loops.",
     PURPLE),
    ("SEARCH", "A* finds the lowest-cost route from every gate to the core.",
     ACCENT),
    ("VALIDATE", "A build is rejected if it makes any gate unreachable.", GREEN),
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
    ("Generation", "Randomized DFS + wall braiding."),
    ("Search", "A* with a Manhattan distance estimate."),
    ("Adaptation", "Re-path after every accepted graph change."),
    ("Safety", "Validate a build before it is committed."),
], size=12, gap=0.66, dot=PURPLE)
add_rect(slide, 8.15, 4.95, 4.45, 0.012, fill=EDGE,
         shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 8.15, 5.12, 4.45, 0.8,
         "The player changes the problem; the algorithms solve it again.", 12.5,
         TEXT, True, BODY, line_spacing=1.1)

add_rect(slide, 0.55, 6.25, 12.25, 0.72, fill="141B29", line="2A4964",
         radius=0.08)
add_text(slide, 0.85, 6.25, 11.65, 0.72,
         "FOCUS   ·   Generation, heuristic search and graph validation — the "
         "game is only the testbed.", 12, TEXT, False, BODY,
         anchor=MSO_ANCHOR.MIDDLE)

# 03 — ALGORITHM 1
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "02  ·  ALGORITHM 1  ·  GENERATION",
       "Randomized DFS builds the maze", 3)

step_card(slide, 0.55, 1.52, 6.1, 1.42, "01", ACCENT,
          "Carve with a stack",
          "A recursive backtracker visits every lattice cell once, carving to "
          "a random unvisited neighbour and backtracking at dead ends. "
          "Linear time in the number of tiles.")
step_card(slide, 0.55, 3.09, 6.1, 1.42, "02", PURPLE,
          "Braid the walls",
          "32% of the remaining interior walls are removed, turning dead ends "
          "into loops. Loops give the player choices and stop one tower from "
          "sealing the whole map.")
step_card(slide, 0.55, 4.66, 6.1, 1.42, "03", GREEN,
          "Anchor the core and gates",
          "A central chamber is carved for the core and four edge gates are "
          "opened. A* verifies every gate reaches the core before the level "
          "goes live.")

add_rect(slide, 6.94, 1.52, 5.75, 4.25, fill="090B11", radius=0.02)
add_picture(slide, "maze_gates.png", 6.86, 1.45, w=5.9)
add_rect(slide, 6.86, 1.45, 5.9, 4.36, fill=None, line="2B3348",
         line_w=1.0, shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 6.86, 5.92, 5.9, 0.24,
         "Four gate-to-core routes  ·  23 × 17 tiles  ·  gold = core", 10,
         MUTED, False, MONO)

chip(slide, 6.86, 6.27, 1.78, "23 × 17 grid", ACCENT, CYAN_DARK, h=0.34,
     size=10)
chip(slide, 8.76, 6.27, 1.78, "32% braided", PURPLE, PURPLE_DARK, h=0.34,
     size=10)
chip(slide, 10.66, 6.27, 2.1, "4 gates connected", GREEN, GREEN_DARK, h=0.34,
     size=10)
add_text(slide, 6.86, 6.75, 5.9, 0.24,
         "Complexity: O(W · H)  ·  a seed reproduces any maze.",
         10, MUTED, False, BODY)

# 04 — ALGORITHM 2
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "03  ·  ALGORITHM 2  ·  SEARCH",
       "A* finds the shortest route", 4)

add_rect(slide, 0.55, 1.5, 6.1, 1.02, fill="141B29", line="2A4964",
         radius=0.08)
add_text(slide, 0.7, 1.58, 5.8, 0.42, "f(n) = g(n) + h(n)", 25, TEXT, True,
         MONO, align=PP_ALIGN.CENTER)
add_text(slide, 0.7, 2.06, 5.8, 0.22,
         "g: cost from the gate      ·      h: Manhattan estimate to the core",
         10, MUTED, False, MONO, align=PP_ALIGN.CENTER)

feature_card(slide, 0.55, 2.68, 6.1, 1.0, "Priority frontier",
             "A min-heap expands the lowest f-score first; a closed set avoids "
             "repeats.")
feature_card(slide, 0.55, 3.86, 6.1, 1.0, "Admissible heuristic",
             "Manhattan distance never overestimates on this grid, so A* "
             "returns an optimal route rather than just a valid one.",
             accent=PURPLE)
feature_card(slide, 0.55, 5.04, 6.1, 1.0, "Live graph",
             "A tower is a blocked cell. Builds are validated from all gates, "
             "and accepted builds re-path every enemy.",
             accent=GREEN)

chip(slide, 0.55, 6.26, 2.25, "O(E log V) per search", ACCENT, CYAN_DARK,
     h=0.34, size=10)
chip(slide, 2.95, 6.26, 2.45, "4-connected paths", GREEN, GREEN_DARK,
     h=0.34, size=10)

add_rect(slide, 7.44, 1.52, 4.82, 3.42, fill="090B11", radius=0.02)
add_picture(slide, "astar_gates.png", 7.44, 1.45, w=4.82)
add_rect(slide, 7.44, 1.45, 4.82, 3.56, fill=None, line="2B3348", line_w=1.0,
         shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 7.44, 5.15, 4.82, 0.24,
         "Explored nodes in blue  ·  route in cyan", 10,
         MUTED, False, MONO)

legend = [
    ("Wall", "2A3146"),
    ("Explored", "25445C"),
    ("Route", ACCENT),
]
lx = 7.44
for label, color in legend:
    add_rect(slide, lx, 5.62, 0.12, 0.12, fill=color, shape=MSO_SHAPE.RECTANGLE)
    add_text(slide, lx + 0.2, 5.56, 1.1, 0.24, label, 10, MUTED, False, MONO)
    lx += 1.55

add_rect(slide, 7.44, 6.02, 4.82, 0.72, fill="1F1A2E", line="4A3A68",
         radius=0.08)
add_rect(slide, 7.44, 6.12, 0.05, 0.52, fill=PURPLE, shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 7.64, 6.08, 4.42, 0.6,
         "Optimality matters: the route length is the cost the player is "
         "trying to increase.", 11, TEXT, False, BODY,
         anchor=MSO_ANCHOR.MIDDLE)

# 05 — MAP GENERATION
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "04  ·  MAP GENERATION  ·  PIPELINE",
       "From empty grid to playable maze", 5)

steps = [
    ("01", "Start with walls", "The grid begins fully blocked; only the odd "
     "lattice cells are candidates for carving.", ACCENT),
    ("02", "DFS carve", "A randomized backtracker visits every cell once, "
     "carving corridors and backtracking at dead ends.", PURPLE),
    ("03", "Braid 32%", "Remove leftover interior walls to create loops, so "
     "one tower cannot seal the only route.", GREEN),
    ("04", "Core + four gates", "Carve a 3×3 core chamber, open edge gates, "
     "then A* checks every gate-to-core route.", GOLD),
]
for i, (num, title, body, color) in enumerate(steps):
    step_card(slide, 0.55, 1.5 + i * 1.1, 6.1, 0.98, num, color, title, body)

add_rect(slide, 6.94, 1.52, 5.75, 4.25, fill="090B11", radius=0.02)
add_picture(slide, "maze_pipeline.png", 6.86, 1.45, w=5.9)
add_rect(slide, 6.86, 1.45, 5.9, 4.36, fill=None, line="2B3348",
         line_w=1.0, shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 6.86, 5.92, 5.9, 0.24,
         "Pipeline: wall grid → DFS carve → braid → core + gates", 10,
         MUTED, False, MONO)

chip(slide, 6.86, 6.27, 1.78, "23 × 17 grid", ACCENT, CYAN_DARK, h=0.34,
     size=10)
chip(slide, 8.76, 6.27, 1.78, "32% braid", PURPLE, PURPLE_DARK, h=0.34,
     size=10)
chip(slide, 10.66, 6.27, 2.1, "O(W · H)", GREEN, GREEN_DARK, h=0.34,
     size=10)
add_text(slide, 6.86, 6.75, 5.9, 0.24,
         "A seed reproduces the same maze; generation is linear in tiles.",
         10, MUTED, False, BODY)

# 06 — WHY RUST
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "05  ·  TECHNOLOGY  ·  WHY RUST",
       "Why the project moved to Rust", 6)

reasons = [
    ("No GC pauses", "A* runs for every enemy and after every accepted build; "
     "Rust keeps frame times predictable.", ACCENT),
    ("Compile-time safety", "Ownership and Result make graph edits and "
     "validation failures explicit.", PURPLE),
    ("Simulation vs Bevy", "The game rules are a pure Rust module; Bevy only "
     "renders the state.", GREEN),
    ("Cargo tooling", "Build, test, clippy and fmt are one command each; 64 "
     "tests run headlessly.", GOLD),
]
for i, (title, body, color) in enumerate(reasons):
    feature_card(slide, 0.55, 1.5 + i * 1.12, 6.2, 1.0, title, body,
                 accent=color)

add_rect(slide, 7.0, 1.5, 5.78, 2.35, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 7.2, 1.63, 4.0, 0.22, "PYTHON → RUST", 10, PURPLE, True,
         MONO)
bullets(slide, 7.2, 1.95, 5.35, [
    ("Prototype", "Python was fast to write but slow with many enemies."),
    ("Rewrite", "Rust gives predictable performance and one binary."),
    ("Cost", "Steeper learning curve and longer compile times."),
], size=10.5, gap=0.62, dot=PURPLE)

add_rect(slide, 7.0, 4.05, 5.78, 2.0, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 7.2, 4.18, 4.0, 0.22, "WHY IT MATTERS", 10, GREEN, True,
         MONO)
bullets(slide, 7.2, 4.5, 5.35, [
    ("Algorithms", "A* and validation stay fast under load."),
    ("Tests", "headless simulation; no window needed."),
    ("Ship", "single binary, no runtime install."),
], size=10.5, gap=0.45, dot=GREEN)

add_rect(slide, 0.55, 6.35, 12.25, 0.62, fill="141B29", line="2A4964",
         radius=0.08)
add_text(slide, 0.85, 6.35, 11.65, 0.62,
         "Rust keeps the algorithms fast and testable; Bevy only draws the "
         "result.", 11.5, TEXT, False, BODY, anchor=MSO_ANCHOR.MIDDLE)

# 07 — VALIDATION
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "06  ·  VALIDATION", "What the tests guarantee", 7)

metric(slide, 0.55, 1.5, 3.05, 1.15, "300 seeds", "generation check",
       "every gate reaches the core", ACCENT)
metric(slide, 3.7, 1.5, 3.05, 1.15, "4 gates", "connected per maze",
       "A* verifies each one", GREEN)
metric(slide, 6.85, 1.5, 3.05, 1.15, "64 tests", "automated",
       "maze, A*, state and waves", PURPLE)
metric(slide, 10.0, 1.5, 3.05, 1.15, "O(E log V)", "per A* search",
       "binary min-heap frontier", GOLD)

add_rect(slide, 0.55, 2.9, 6.2, 3.35, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 0.75, 3.04, 4.0, 0.22, "GUARANTEES", 10, GREEN, True, MONO)
checks = [
    "All four gates are connected on 300 seeds.",
    "Walls and towers are blocked cells.",
    "A build that seals any gate is rejected.",
    "Selling reopens a cell and re-paths enemies.",
    "A seed reproduces the same maze.",
]
for i, line in enumerate(checks):
    y = 3.4 + i * 0.48
    add_rect(slide, 0.78, y + 0.07, 0.1, 0.1, fill=GREEN, shape=MSO_SHAPE.OVAL)
    add_text(slide, 1.0, y, 5.45, 0.3, line, 11, MUTED, False, BODY)

add_rect(slide, 7.0, 2.9, 5.78, 3.35, fill=PANEL, line=EDGE, radius=0.06)
add_text(slide, 7.2, 3.04, 4.0, 0.22, "COMPLEXITY", 10, PURPLE, True, MONO)
complexity = [
    ("Maze generation", "O(W · H)", "Each cell is carved once; braiding scans "
     "the grid."),
    ("A* search", "O(E log V)", "Binary heap frontier over grid edges."),
    ("Re-path", "O(K · E log V)", "K enemies search after a graph change."),
    ("Validation", "O(G · E log V)", "G gates checked before a build."),
]
for i, (name, big, body) in enumerate(complexity):
    y = 3.35 + i * 0.7
    add_text(slide, 7.2, y, 2.0, 0.24, name, 11, TEXT, True, BODY)
    add_text(slide, 9.2, y, 1.35, 0.24, big, 11, PURPLE, True, MONO,
             align=PP_ALIGN.RIGHT)
    add_text(slide, 7.2, y + 0.24, 5.25, 0.28, body, 9.5, MUTED, False, BODY)

add_rect(slide, 0.55, 6.42, 12.25, 0.6, fill="141B29", line="2A4964",
         radius=0.08)
add_text(slide, 0.85, 6.42, 11.65, 0.6,
         "300 seeds × 4 gates = 1,200 gate-to-core searches verified by the "
         "test suite.", 11.5, TEXT, False, BODY, anchor=MSO_ANCHOR.MIDDLE)

# 08 — CONCLUSION
slide = prs.slides.add_slide(BLANK)
background(slide)
header(slide, "07  ·  CONCLUSION", "Three algorithms, one adaptive system", 8)

add_rect(slide, 0.55, 1.5, 6.2, 1.15, fill="1F1A2E", line="4A3A68",
         radius=0.08)
add_rect(slide, 0.55, 1.6, 0.05, 0.95, fill=PURPLE, shape=MSO_SHAPE.RECTANGLE)
add_text(slide, 0.8, 1.58, 5.75, 0.98,
         "Generation gives variety. A* gives correct routes. Validation makes "
         "live graph changes safe.", 14.5, TEXT, True, BODY,
         anchor=MSO_ANCHOR.MIDDLE, line_spacing=1.1)

conclusions = [
    ("Generate", "randomized DFS + braiding creates a fresh, connected maze."),
    ("Search", "A* + Manhattan returns optimal gate-to-core routes."),
    ("Adapt", "validate and re-path after every accepted graph change."),
    ("Lesson", "constraints make heuristic search safe in a live system."),
]
bullets(slide, 0.55, 3.0, 6.2, conclusions, size=12.5, gap=0.66, dot=GREEN)

add_text(slide, 7.1, 1.5, 3.0, 0.22, "FUTURE WORK", 10, PURPLE, True, MONO)
future = [
    ("Weighted A*", "Trade optimality for speed with large swarms."),
    ("Flow fields", "One search serves every enemy at once."),
    ("Genetic tuning", "Evolve maze parameters against a simulated player."),
    ("Learned placement", "Q-learning for automatic tower placement."),
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

# Speaker notes
notes = [
    "Title slide. Introduce the team and frame the project as three algorithms: "
    "randomized DFS, A* search and live re-planning. Replace the member "
    "placeholders before submitting.",
    "The game is only the testbed. The real problem is a graph that changes "
    "while it is being solved: the player blocks cells, and the AI must keep "
    "the maze solvable.",
    "Maze generation is a randomized depth-first search. It carves every cell "
    "once, then braiding removes 32 percent of the remaining walls. Four edge "
    "gates and a central core are added, and A* verifies connectivity.",
    "A* uses f = g + h. The heuristic is Manhattan distance, which is "
    "admissible on a 4-connected grid. The frontier is a binary heap, so the "
    "lowest f-score expands first and the result is optimal. Because the graph "
    "is live, builds are validated from all gates and accepted builds re-path "
    "every enemy.",
    "This is the full generation pipeline. Start with a wall grid, carve with "
    "randomized DFS, braid 32 percent of the walls into loops, then add the "
    "core and four gates. The design choices matter: one route is too easy to "
    "block, loops create choices, and four gates create multiple fronts.",
    "Why Rust: no garbage-collector pauses while A* runs for many enemies, "
    "compile-time checks around graph edits, a pure simulation separated from "
    "Bevy rendering, and cargo test/clippy/fmt. The trade-off is a steeper "
    "learning curve and longer compile times.",
    "These are the guarantees the tests actually check. The 300-seed test runs "
    "four gate-to-core searches per seed, so 1,200 searches are covered. The "
    "complexity table shows where the cost comes from.",
    "To conclude: generation gives replayability, A* gives correct routing, "
    "and validation makes live changes safe. Future work is about scaling the "
    "search and learning better maze parameters. Thank you — questions?",
]
for slide, note in zip(prs.slides, notes):
    slide.notes_slide.notes_text_frame.text = note

prs.core_properties.title = "AI Tower Defense — Review 1"
prs.core_properties.author = "AI Project"
prs.save(OUT)
print("saved", OUT, "with", len(prs.slides), "slides")
