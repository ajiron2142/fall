# Little Worlds

Three single-player, mobile-first browser prototypes with fixed 3D cameras and simple touch controls:

- **Fold Together:** flip paper islands, walk their connected gold paths, collect stars, and reach the gateway. Paths and starting folds vary by seed. Folding carries the character with the sheet. Undo and hints are available.
- **Pocket Gravity:** drag a tilt pad to steer a marble, operate a transfer gate, gather crystals, and brake into the goal cup. Four mirrored maze orientations. Holes rescue the marble without removing collected crystals. Arrow keys tilt; Space brakes.
- **Room Shuffle:** slide rows of rooms, walk each of two friends through matching doorways, collect two keys, and meet in the garden. Seeded spanning-tree corridors and reversible row scrambling guarantee a solution. Undo and a search-based hint are available.

No build step, accounts, runtime CDN, or backend. Three.js 0.160.1 is bundled locally with its MIT license; the npm archive was SHA-512 verified. Earlier experiments remain at `afterlight.html` and `lab.html`.

## Run

From this directory, run `python3 -m http.server 8000`, then open the site in a browser. Tap ⛶ for fullscreen on supported browsers. On iPhone, use Share → Add to Home Screen.

## Impact — FPS feel test

Open `arena.html` for a separate first-person arena with a bat and shotgun. The shotgun is equipped by default. Switch weapons with Q, the mouse wheel, or the on-screen pill. WASD/mouse moves and aims, click attacks, Space jumps (press again in midair for a double jump), Shift dashes, and E toggles slow motion. Touch controls default off; enable them in settings (saved on this device). Touch uses a move joystick, drag-to-look, and simultaneous action buttons; holding and dragging Swing/Fire combines aiming and attacking under one thumb. Aim assist defaults on for touch and is optional in the pause menu. The tapered maple bat has a wrapped handle and one gloved hand and a visible forearm. It rests low beside the body at a diagonal, makes a quick outward side swing, and follows through low with a brief stylized arc. A tiny idle wrist sway adds breathing motion; running adds power. Snappy runs at 24 m/s by default. Dash once to reach 30 m/s; holding a forward direction preserves earned speed across running and jumps, without repeated taps or a held sprint button. Release movement or turn sharply to brake. Landing retains horizontal momentum while grip builds over 160 milliseconds, producing a short skid before normal traction returns; low-speed drift snaps to rest. Momentum runs at 28 m/s and Floaty at 20 m/s. Jumping preserves momentum, and air steering redirects it. The second jump redirects horizontal momentum toward movement input (including in bullet time), or keeps its heading without input. It restarts a slightly softer arc, works during descent, and recharges on landing. The top-right FPS counter reports actual rendered frames per second. Bat impacts add a brief hit pause; both weapons add bass, sparks, and arena light pulses. Shotgun recoil moves the weapon, keeping the camera steady. The camera uses a fixed 88-degree vertical field of view, with no speed-based widening or zoom. Bullet time slows the player, objects, pellets, cooldowns, and weapon animation to 8%; aiming remains responsive. Pellets hit moving targets on contact, so toggling time back to normal releases a pending volley. The industrial arena uses procedural panels and fixtures, with no external asset downloads. The shotgun fires nine deterministic travelling pellets with a nine-metre limit, distance falloff, strong close-range impulses, and a short pump cycle. The arena is 160 metres wide and 320 metres long (almost ten times the previous floor area), with seven separate playgrounds: Easy/Medium/Hard jumping climbs to 12.5/17.9/23.3 metres, Easy/Medium/Hard wall-running lanes, and one combined Expert route. The jump routes keep their tall climbs, with larger landing surfaces and 18/21/24-metre spacing tuned for both 24 m/s running and 30 m/s momentum. Wall-running lanes span 40/56/72 metres. The combined route uses 50-metre gaps and sits separately beside the sprint runway. Glowing walls support automatic wall-running on close contact when moving alongside them while airborne, preserve carried speed, and release when steering away; Space/Jump kicks away and restores the air jump. Wall-only routes can be completed without jumping. Only the combined route requires both skills. Choose a section in Pause; Course / R restarts that section. Runway teleports to a separate 280-metre clear sprint lane with distance markings. The pause menu offers Snappy, Momentum, and Floaty movement presets; choosing one resets the player to the selected section’s starting platform for a fair comparison and saves the choice on this device. The firing range uses six axis-aligned breakable blocks and four thin square targets (one moves horizontally), with visible damage bars and 3/6/9-metre distance markings. A bat strike breaks a block; shotgun pellet damage falls off with distance. Hits use matching box/panel bounds rather than sphere hitboxes. Blocks have gravity, firm floor friction, and low bounce. Broken objects disappear and rebuild after three seconds of world time; simple cube fragments are visual effects. The HUD counts breaks. Range / T rebuilds and returns to the firing area while keeping the selected weapon. There are no enemies, health system, or multiplayer.

`arena-core` is a dependency-free Rust library compiled to WebAssembly: movement, collisions, block contacts, attack rules, damage, rebuilding, slow motion, and scoring live there. Three.js renders; browser JavaScript handles UI, input, audio, and effects. This is a gameplay experiment with a small Rust simulation, not a complete custom Rust engine. The core was built using Rust 1.99.0; it requires Rust >= 1.85.

To rebuild, install Rust and `rustup target add wasm32-unknown-unknown`, then run `sh scripts/build-arena.sh`. The script runs Rust tests, builds the WASM module, and packages `downloads/impact-browser.zip`. The Pages workflow runs this build before publishing so the binary and source stay together. The archive includes prebuilt browser assets and the Rust source; extract and follow `START-HERE.txt`. It requires a local HTTP server, not compilation. It is not a native desktop executable.

`python3 tests/arena_browser.py` tests the actual WASM build in mobile and desktop browsers, including simultaneous movement/jump/dash, both attacks, slow motion, touch aiming, pause/reset, fullscreen, and keyboard/mouse controls.

## Validate

`node --test tests/puzzles.test.mjs` checks generated puzzle solutions, folding with an occupied sheet, state immutability, role ownership, room hints, gate collisions, braking, and completing the marble maze through actual physics in every orientation.

For browser checks, install Python Playwright (`pip install playwright`, then `playwright install chromium`) and run `python3 tests/worlds_browser.py`. It starts its own temporary local server and checks mobile canvas picking, puzzle completion, undo, simultaneous tilt/gate/brake input, fullscreen, landscape layout, and desktop keyboard input. `python3 tests/browser_smoke.py` checks the retained Afterlight prototype. Browser emulation does not establish performance on real phone hardware.

## Future co-op

`puzzles.mjs` contains DOM-free rules: JSON-serializable seeded state, `applyAction(state, action)`, and fixed-step `gravityStep(state, dt)`. Rendering and input live separately in `worlds.mjs`. No transport or matchmaking is implemented.

Solo mode permits all controls. Creating a state with mode `coop` enforces these role assignments:

| Game | Player A | Player B |
| --- | --- | --- |
| Fold Together | Walk; fold outer sheets | Fold middle sheet |
| Pocket Gravity | Tilt and brake | Operate transfer gate |
| Room Shuffle | Coral friend; outer rows | Mint friend; middle row |

For online play, a shared host can bind players to roles, validate commands with these rules, and broadcast state snapshots. Fold/room games need only discrete commands; gravity needs a host simulation with input updates and interpolation. Player-specific clue views still need to be designed: the solo prototype currently reveals the whole puzzle, including hints. Role restrictions alone do not establish that co-op is fun.

## Publish

GitHub **Settings → Pages → Source → GitHub Actions**. Push to `main`; `.github/workflows/pages.yml` publishes only the static game assets and vendored renderer. GitHub Pages hosts these solo games, not a future matchmaking service. Free Pages hosting is available for public repositories; private repository eligibility depends on the GitHub plan.
