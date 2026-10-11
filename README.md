# Two of Us — game lab

Afterlight is a first-person 3D browser prototype: jump and dash across generated floating ruins, switch between two realities to make paths solid, and awaken three checkpoint anchors to reach the gateway. Desktop: WASD/mouse, Space jump, Shift dash, E change reality. Phone: left joystick, drag to look, action buttons.

The three earlier prototypes remain at `lab.html`. No build step or backend. Three.js 0.160.1 is vendored with its MIT license; its npm artifact was SHA-512 verified. Rendering uses WebGL. Afterlight is solo, not implemented multiplayer. Physics handles platform landings, not general rigid bodies; the skyline is decorative. Real phone performance still needs hands-on testing.

Hold the joystick with one finger and press Jump with another. Action buttons respond on press, including secondary touch pointers. Jump input is buffered for 200 ms and works for 160 ms after leaving an edge. The first three crossings are straight, always solid, and closer together. Each successful landing saves your position.

Tap ⛶ for fullscreen where supported. On iPhones without browser fullscreen support, use Share → Add to Home Screen and launch from the home screen.

## Browser regression checks

Install Python Playwright (`pip install playwright`, then `playwright install chromium`), and run `python3 tests/browser_smoke.py`. The test starts its own temporary local server. It checks simultaneous joystick and second-finger jumping across the first gap, desktop and mobile actions, fullscreen, fall recovery, and completing 30 generated routes through the actual physics. The deterministic route driver is injected only into the test response; production has no movement shortcuts.

From this directory, run `python3 -m http.server 8000`, then open the site in a browser. Static hosting can serve these files directly.

## Host on GitHub Pages

In the GitHub repository, open **Settings → Pages → Build and deployment**, and select **GitHub Actions** as the source. Push the game files and `.github/workflows/pages.yml` to `main`. The workflow publishes the static assets and vendored renderer. Later pushes to `main` deploy updates automatically; you can also run it manually from the Actions tab. The deployment reports the site's URL.

GitHub Pages hosts these solo prototypes; it does not provide a multiplayer matchmaking server. Free Pages hosting is available for public repositories; private repository eligibility depends on the GitHub plan.

- **Signal Room:** a generated four-symbol code, split between operator and clue reader. Peek switches information views in solo mode.
- **Two Sides:** a generated island maze with a guaranteed route. Solo mode lets you peek at the navigator’s map.
- **In Sync:** five timing hits with changing targets and a simulated partner.

These explore mechanics, not actual two-player networking. Evaluate communication puzzles with another person sharing a screen; solo peeking cannot establish whether multiplayer is fun.

Signal Room and Two Sides need only occasional turn messages. In Sync would need latency-aware timing if converted to a real two-player game.
