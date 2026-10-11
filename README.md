# Little Worlds

Three single-player, mobile-first browser prototypes with fixed 3D cameras and simple touch controls:

- **Fold Together:** flip paper islands, walk their connected gold paths, collect stars, and reach the gateway. Paths and starting folds vary by seed. Folding carries the character with the sheet. Undo and hints are available.
- **Pocket Gravity:** drag a tilt pad to steer a marble, operate a transfer gate, gather crystals, and brake into the goal cup. Four mirrored maze orientations. Holes rescue the marble without removing collected crystals. Arrow keys tilt; Space brakes.
- **Room Shuffle:** slide rows of rooms, walk each of two friends through matching doorways, collect two keys, and meet in the garden. Seeded spanning-tree corridors and reversible row scrambling guarantee a solution. Undo and a search-based hint are available.

No build step, accounts, runtime CDN, or backend. Three.js 0.160.1 is bundled locally with its MIT license; the npm archive was SHA-512 verified. Earlier experiments remain at `afterlight.html` and `lab.html`.

## Run

From this directory, run `python3 -m http.server 8000`, then open the site in a browser. Tap ⛶ for fullscreen on supported browsers. On iPhone, use Share → Add to Home Screen.

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
