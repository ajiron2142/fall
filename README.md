# Two of Us — game lab

Three mobile-first, local gameplay prototypes. Plain JavaScript, CSS, and HTML; no build step, services, or dependencies.

From this directory, run `python3 -m http.server 8000`, then open the site in a browser. Static hosting can serve these files directly.

## Host on GitHub Pages

In the GitHub repository, open **Settings → Pages → Build and deployment**, and select **GitHub Actions** as the source. Push the game files and `.github/workflows/pages.yml` to `main`. The workflow publishes only the three game assets. Later pushes to `main` deploy updates automatically; you can also run it manually from the Actions tab. The deployment reports the site's URL.

GitHub Pages hosts these solo prototypes; it does not provide a multiplayer matchmaking server. Free Pages hosting is available for public repositories; private repository eligibility depends on the GitHub plan.

- **Signal Room:** a generated four-symbol code, split between operator and clue reader. Peek switches information views in solo mode.
- **Two Sides:** a generated island maze with a guaranteed route. Solo mode lets you peek at the navigator’s map.
- **In Sync:** five timing hits with changing targets and a simulated partner.

These explore mechanics, not actual two-player networking. Evaluate communication puzzles with another person sharing a screen; solo peeking cannot establish whether multiplayer is fun.

Signal Room and Two Sides need only occasional turn messages. In Sync would need latency-aware timing if converted to a real two-player game.
