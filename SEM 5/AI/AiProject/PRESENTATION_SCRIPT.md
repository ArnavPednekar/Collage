# AI Tower Defense — Review 1 Presentation Script

Target length: **6–7 minutes** plus questions.

Before presenting:
- Fill in `[Member 1..3]` and roll numbers on slide 1.
- Optional live demo: run `cd rust && cargo run` and keep the game on screen.
- The deck is 8 slides. Slides 5 and 6 cover the full map-generation pipeline and why the project uses Rust.

---

## Slide 1 — Title (0:00–0:30)

**Say:**

Good morning/afternoon. We are [names], and this is our AI project, **AI Tower Defense**.

The project is a tower defense game, but the game is really a testbed for three algorithms: **randomized DFS** for map generation, **A\*** for pathfinding, and **live re-planning** when the player changes the map.

**Point to:** the three chips on the title slide.

---

## Slide 2 — The problem (0:30–1:15)

**Say:**

The core problem is that the map is a graph that keeps changing.

Enemies need a route through a grid. The player blocks cells by placing towers, so every accepted tower changes the graph. The AI has to generate a solvable maze, find routes through it, and keep those routes valid while the player is building.

We can summarise the project with three verbs:

- **Generate:** randomized DFS builds a connected maze and braiding adds loops.
- **Search:** A* finds the lowest-cost route from every gate to the core.
- **Validate:** a build is rejected if it makes any gate unreachable.

The player changes the problem; the algorithms solve it again.

**Point to:** the three cards and the “why it is an AI project” panel.

---

## Slide 3 — Algorithm 1: Randomized DFS (1:15–2:15)

**Say:**

The first algorithm is map generation.

We start with a grid of walls. Then a **randomized depth-first search**, also called a recursive backtracker, carves the maze.

The algorithm keeps a stack of cells. From the current cell, it picks a random unvisited neighbour two cells away, carves the wall between them, and moves there. When it reaches a dead end, it backtracks by popping the stack. Every cell is visited once, so the carve is linear in the number of tiles: **O(W · H)**.

That produces a **perfect maze** — exactly one route between any two cells. A perfect maze is easy to generate, but it is also too easy to block: one tower could seal the only path.

So we **braid** the maze: we remove **32%** of the remaining interior walls. That turns dead ends into loops and gives the player multiple routes to think about.

Finally we carve a **3×3 core chamber** in the centre and open **four edge gates** — left, top, right and bottom. We then use A* to verify that every gate can reach the core.

**Point to:** the four-colour routes in the image. Each colour is one gate-to-core route.

---

## Slide 4 — Algorithm 2: A* (2:15–3:15)

**Say:**

The second algorithm is **A\***.

A* uses the formula:

**f(n) = g(n) + h(n)**

- **g(n)** is the actual cost from the gate to the current cell.
- **h(n)** is the heuristic estimate from the current cell to the core.
- **f(n)** is the estimated total cost.

We use **Manhattan distance** for the heuristic. The grid is 4-connected with uniform step cost, so Manhattan distance never overestimates the real cost. That makes the heuristic **admissible**, which means A* returns an **optimal** route.

The frontier is a binary min-heap: the cell with the lowest f-score is expanded first. A closed set prevents us from expanding the same cell again. The complexity is **O(E log V)** per search.

The important part is that the graph is **live**. Walls and towers are blocked cells. When the player builds or sells a tower, the graph changes, so builds are validated from all gates and accepted builds re-path every enemy.

**Point to:** the explored nodes in blue and the optimal route in cyan.

---

## Slide 5 — Map generation pipeline (3:15–4:15)

**Say:**

This slide shows the full generation pipeline.

1. **Start with walls.** The grid is fully blocked. Only the odd lattice cells are candidates.
2. **DFS carve.** The randomized backtracker carves corridors and backtracks at dead ends.
3. **Braid 32%.** We remove leftover interior walls to create loops, so one tower cannot seal the only route.
4. **Core and four gates.** We carve the 3×3 core chamber, open the edge gates, and check every gate-to-core route with A*.

The design choices are deliberate. A perfect maze has only one route, so it is too easy to block. Braiding creates choices. Four gates create multiple fronts. And because generation is seeded, the same seed reproduces the same maze, which is important for testing and debugging.

Generation is **O(W · H)** — linear in the number of tiles.

**Point to:** the four-stage image: wall grid → DFS → braid → core and gates.

---

## Slide 6 — Why Rust (4:15–5:15)

**Say:**

The original prototype was written in Python. Python was fast to write, but it becomes slow when we have many enemies and run A* repeatedly inside the game loop.

So we moved the project to **Rust**.

There are four main reasons:

- **No garbage-collector pauses.** A* runs for every enemy and after every accepted build. Rust keeps frame times predictable.
- **Compile-time safety.** Ownership, enums and `Result` make graph edits and validation failures explicit.
- **Simulation vs Bevy.** The game rules are a pure Rust module. Bevy only renders the state. That keeps the algorithms testable without opening a window.
- **Cargo tooling.** Build, test, clippy and fmt are one command each. We currently have **64 automated tests**.

The trade-off is a steeper learning curve and longer compile times. But for an algorithm-heavy project that re-paths many enemies, the performance and safety are worth it.

**Point to:** the four reason cards and the “Python → Rust” panel.

---

## Slide 7 — Validation (5:15–6:00)

**Say:**

This slide shows what the tests actually guarantee.

- The generation test runs **300 seeds** and checks **all four gates** on every seed. That is **1,200 gate-to-core searches**.
- Walls and towers are treated as blocked cells.
- A build that would seal any gate is rejected.
- Selling a tower reopens the cell and re-paths enemies.
- The same seed reproduces the same maze.

The complexity table summarises the cost:

- Maze generation is **O(W · H)**.
- A* is **O(E log V)** per search.
- Re-pathing is **O(K · E log V)** for K enemies.
- Validation is **O(G · E log V)** for G gates.

So the algorithms are fast enough to run inside the game loop, and the tests cover the important safety properties.

---

## Slide 8 — Conclusion (6:00–6:30)

**Say:**

To conclude:

- **Generation** gives variety: randomized DFS plus braiding creates a fresh, connected maze.
- **Search** gives correct routes: A* with a Manhattan heuristic returns optimal gate-to-core paths.
- **Adaptation** keeps it safe: validation and re-pathing handle every accepted graph change.

The main lesson is that **constraints make heuristic search safe in a live system**. The player can change the graph, but the algorithms keep the maze solvable.

Future work includes weighted A* or flow fields for larger swarms, genetic tuning of maze parameters, and learned tower placement.

Thank you — we are happy to take questions.

---

## Quick Q&A Preparation

**Why 32% braiding?**
It is a balance. Too little braiding leaves a perfect maze with one route, which is easy to block. Too much braiding turns the maze into an open room. Around 32% keeps loops without losing the maze structure.

**Why A* instead of BFS or Dijkstra?**
BFS works on unweighted grids, but A* uses the Manhattan heuristic to guide the search toward the core. Dijkstra would also find the shortest path, but it expands more nodes because it has no heuristic.

**Why is Manhattan distance admissible?**
The grid is 4-connected and every step costs 1. Manhattan distance is the minimum number of steps needed, so it can never overestimate the true cost.

**How do you stop a tower from sealing the core?**
Before accepting a build, we temporarily block the proposed cell and run A* from all four gates. If any gate cannot reach the core, the build is rejected.

**Why four gates?**
Multiple gates create multiple fronts. The player cannot defend one choke point and ignore the rest. It also makes the validation step meaningful because every gate must stay connected.

**Why Rust instead of Python?**
Python was good for the prototype, but Rust gives predictable performance without GC pauses, compile-time safety, and a clean separation between the simulation and the Bevy renderer.

**What happens when an enemy is already on the map and the player builds a tower?**
The enemy re-runs A* from its current cell to the core. It does not keep the old route.

**What is the hardest part of the project?**
Keeping the maze solvable while the player changes the graph. The validation step is what makes dynamic tower placement safe.
