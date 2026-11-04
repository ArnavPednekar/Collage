# CONTEXT SAVE — 2026-11-03

Working dir: /home/sunrays/Documents/College/SEM 5/AI/AiProject (git repo).

## Objective overall
- Phase 1 (DONE): Python/pygame tower defense with procedural maze + A*.
- Phase 2 (DONE): Rust + Bevy port with central core, six upgradeable towers, four gates, multi-gate waves, raised terrain and top/bottom HUD. 64 tests, clippy and fmt clean.
- Review-1 deck (DONE): simplified to 8 algorithm-focused slides and updated to the current Rust design. Remaining: team names/roll numbers on slide 1.

## Phase 2 progress (what we established this session)
1. Toolchain: cargo/rustc 1.95.0 OK. Bevy 0.20 needs rustc 1.97.1 → must use **bevy 0.19.1** (verified via `cargo add`).
2. Network: crates.io index + downloads WORK. `cargo fetch` was launched in background in /tmp/nettest (bevy 0.19.1 with gltf). Fetch was still running when session ended; bevy-* sources already visible under ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/.
3. Runtime: No Xvfb, but DISPLAY=:0 exists (X11 available — user has a desktop). alsa + udev dev libs present (bevy system deps OK).
4. Asset pack = "new assets for the game/": **Kenney Tower Defense Kit (2.1), CC0**, 160 GLB objects + shared `colormap.png` texture atlas (+ `variation-a.png` alt). Formats: GLB (use this), FBX, OBJ.
5. GLB facts (parsed):
   - Unit scale: all models 1x1 tile footprint, origin at base center, +Y up. Perfect for cell=1.0 world unit.
   - Floor: tile.glb (y 0..0.2), tile-dirt (0..0.1), tile-bump (0..0.3).
   - Wall props: tile-tree.glb (h .96), tile-rock.glb (h .61), tile-crystal.glb (h .75) — use as maze "walls".
   - Towers: tower-square-build-a..f (1x1, h 1.0), tower-round-build-*, tower-square-bottom-a (h .5).
   - Weapons (place at tower top y=1.0): weapon-turret (h .3), weapon-cannon (h .44), weapon-catapult (h .44, extends -0.52..0.32 in Z), weapon-ballista (h .44).
   - Ammo: weapon-ammo-bullet/cannonball/boulder/arrow.
   - Enemies: enemy-ufo-a/b/c/d (4 variants, h .5-.68) → map to Grunt/Runner/Tank/Boss (+ scale per type).
   - Markers: spawn-square.glb (flat 0.34 footprint), selection-a.glb (flat ring, h .05) for hover/goal.
   - Each GLB = 1 node + 1 mesh + material "colormap" (tinting OK via StandardMaterial base_color).

## Chosen architecture (Bevy 0.19)
- crate dir: `rust/` inside project. Assets: copy/rename pack dir to `rust/assets/kenney/` (or reference pack path).
- Hybrid like Python: pure-Rust sim structs (Maze gen, A*, Enemy/Tower/Projectile, Game state with same methods as Python `Game`: new_game/update/step/try_place/sell_tower/start_wave/build_wave/placement_error) held in a Bevy Resource; ECS used only to mirror/render. Maze + A* as pure fns with unit tests.
- 3D scene: grid 23x17, cell 1.0; floor tiles + tree/rock/crystal walls; camera perspective 3/4 view (~55°); picking via `Camera::viewport_to_world` (verified it exists in bevy_camera-0.19.1) + ray/y=0 plane intersection (manual, avoids bevy_picking setup).
- HUD: Bevy UI right panel, dark theme colors from td/config.py (C_PANEL (22,25,37), C_TEXT (222,228,242), C_ACCENT (80,200,255), C_GOLD (250,205,90), C_GOOD (95,220,145), C_BAD (240,95,105), C_MUTED (128,138,162)).
- OPEN API QUESTION (verify when sources ready): UI buttons — `Interaction` component vs bevy_picking `Pointer<Press>` events in 0.19 (bevy_ui src wasn't extracted yet when last checked). Fallback: keyboard-only UI + 1/2/3, Space, R, F, P.

## Current game constants / rules
- Grid 23x17, cell 1.0; `BRAID_CHANCE=0.32`, `TOTAL_WAVES=15`.
- `START_GOLD=320`, `START_CORE_HP=1000`, `CORE_LEAK_SCALE=50`, `MAX_TOWER_LEVEL=3`, sell refund 60%.
- Central core at `(COLS/2, ROWS/2)` = `(11,8)`; four edge gates ordered **left, top, right, bottom**. Wave gates: 1→0, 2→1, 3→2, 4→3, 5→0+2, 6→1+3, 7→0+1+2, 8→1+2+3, 9+→all.
- Six towers: Pulse 60g, Rail 110g, Scatter 85g, Mortar 140g + splash, Cryo 120g + slow, Arc 160g + chain. Upgrades: 0.8x / 1.2x cost, +40% damage, +10% range, +8% fire rate.
- Enemies: Grunt 40hp/55pxs, Runner 22/105, Tank 160/34, Boss 380/28; rewards 8/8/20/60, leaks 1/1/2/5.
- Wave scaling: hp `1+0.13*(w-1)`, speed `1+0.015*(w-1)`, interval `max(.30, .9-.035w)`, clear bonus `45+14w`.
- A* remains 4-connected Manhattan with heap; every gate must remain connected when a tower is placed.


## Latest deck update (2026-11-03)
- `tools/make_ppt.py` now builds **8 slides**: title, problem, randomized DFS, A*, map-generation pipeline, why Rust, validation, conclusion.
- Removed the old game-rules, tower/enemy-stat, interface, codebase-walkthrough and 15-wave balance slides; the deck is now algorithm-first.
- Slide 5 now explains the full generation pipeline: wall grid → DFS carve → braid → core + four gates.
- Slide 6 now explains why the project moved to Rust: no GC pauses, compile-time safety, pure simulation vs Bevy rendering, cargo tooling.
- New figures: `assets/maze_pipeline.png` (four-stage generation), `assets/maze_gates.png` (four gate-to-core A* routes), `assets/astar_gates.png` (explored nodes + route). `assets/repath_gates.png` exists but is no longer used in the deck.
- Presentation script: `PRESENTATION_SCRIPT.md` (slide-by-slide talk track, timing, Q&A prep).
- QA rendered via Aspose; watermark is render-only. Team placeholders remain on slide 1.
- Regenerate with `.venv/bin/python tools/make_ppt.py`; output is `AI_Tower_Defense.pptx`.

## Session history summary
- Built Python game (main.py, td/{config,maze,pathfinding,entities,game}.py, requirements.txt), fixed A* walls bug, assets/ (gameplay, maze_output, astar_demo, hero_maze, repath_demo), tools/{make_assets,make_ppt}.py.
- 12-slide AI_Tower_Defense.pptx final: all slides visually QA'd via Aspose render (watermark = render-only artifact). Headers: 04·AI·MAZE GENERATION/PATHFINDING/ADAPTATION. Slide 9 badge 4 moved to (1.16,5.3). Slide 4 rules card h=2.7, rules start 2.0 gap .32. Slide 7 footnote at y 6.24 ("path length 31 → 59", repath seed 267, block (14,3), detour 28).
- Headless smoke test PASSED: 3 waves, placements, draw() OK, 0 off-floor enemies.
- Team placeholders still to fill: [Member 1..3] + roll numbers on slide 1.
- Benchmarks (todos must keep): 300/300 solvable; maze 0.16ms; A* avg 0.153ms worst 0.349ms; route mean 29.2 (20-40); 1191/1200 builds accepted.
- /tmp tooling: /tmp/render_ppt.py + /tmp/opencode/pptenv + LD_LIBRARY_PATH=/tmp/opencode/gdi/usr/lib (libgdiplus) for PPT QA renders; /tmp/smoke.py python smoke test. NOTE: /tmp/opencode/fetch.log vanished but cargo fetch may still run (pgrep it).
- /tmp/nettest = throwaway cargo project with bevy 0.19.1 dep (just for fetch/connectivity test).

## Latest major redesign (2026-10-25)
Implemented and verified:
- **Central core + gates**: raised 3x3 core chamber at map centre; four edge gates with idle/active beams; all four A* routes previewed; core HP replaces lives and enemy leaks subtract `leak * 50`.
- **Six towers + upgrades**: `TowerKind::{Basic,Sniper,Rapid,Cannon,Frost,Tesla}`, special stats (splash/slow/chain), level 1–3 with upgrade/sell UI; all 12 Kenney build bodies, 4 weapons, level rings.
- **Multi-gate waves**: `wave::spawn_indices`, `Game::active_spawns`, random gate selection per spawned enemy; HUD shows wave/enemies.
- **Raised terrain**: 0.58 grass plateaus, 1.03 wall cliffs, dirt sides, grass tops, dense trees/rocks/crystals, snow patch, pond, wood structures, island apron, shadows.
- **HUD redesign**: full-width top bar (brand/gold/core bar/wave/enemies/speed/path/new maze) and bottom bar (6 tower cards, selected-tower upgrade/sell panel, send wave, messages, hotkey help). Fullscreen borderless 1536x864 logical.
- **Camera**: lower isometric angle, distance 24..44, distance fog, Gaussian shadow filtering; whole board fits above bottom bar.
- **Controls**: 1–6 select tower, LMB build/select, RMB sell, U upgrade, X sell, Space wave, R new maze, F speed, P path, scroll/± zoom.
- Verified with temporary demo/autoplay patches (removed): 6 tower kinds placed, upgrades, wave 1 spawning/clearing, core panel, path preview.
- `cargo build` clean, `cargo test` **64 passed**, `cargo clippy --all-targets` clean, `cargo fmt` clean.
- Final artifacts: `/tmp/opencode/final_redesign.png` (clean start), `/tmp/opencode/demo1.png`–`demo4.png` (towers/waves), `/tmp/opencode/redesign4.png`.
- Run: `cd rust && DISPLAY=:0 WAYLAND_DISPLAY=wayland-1 cargo run`; capture with `WAYLAND_DISPLAY=wayland-1 grim /tmp/shot.png`.
- Kill game: `pgrep -af target/debug/ai_tower_defense`, then `kill <pid>` (do not use `pkill -f`, it can kill the shell).

### Remaining
1. Deck still needs team names + roll numbers on slide 1.
2. Optional: reusable-components report (sim vs view vs UI).
3. Optional: balance pass after playing waves 1–15; tune core HP/gold/tower stats.
4. Optional: health-bar readability / snow-patch polish.
