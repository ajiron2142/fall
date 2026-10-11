"""Package the tested static site and Rust source; never include caches or credentials."""
from pathlib import Path
from zipfile import ZipFile, ZipInfo, ZIP_DEFLATED
root=Path(__file__).resolve().parents[1]
files=['index.html','arena.html','afterlight.html','lab.html','style.css','game.js','echo.css','echo.js','worlds.css','worlds.mjs','puzzles.mjs','manifest.webmanifest','arena/style.css','arena/game.mjs','arena/core.wasm','vendor/three.module.js','vendor/THREE-LICENSE.txt','arena-core/Cargo.toml','arena-core/Cargo.lock','arena-core/src/lib.rs']
instructions='''IMPACT — browser movement and weapon demo

This is a browser build, not a native desktop executable.

1. Extract this archive.
2. In the extracted directory, run: python3 -m http.server 8000
3. Open http://localhost:8000/arena.html in your browser.

Do not open arena.html through file://; browsers restrict WASM loading that way.
Everything runs locally. No account, backend, or runtime CDN is required.

WASD + mouse, click to attack, Space jump/double jump, Shift dash, Q / mouse wheel weapon, R course, E bullet time.
Touch: joystick to move, drag the sky to aim, separate jump/dash/attack buttons.
Aim assist defaults on for touch. Toggle it in the pause menu.
Choose Snappy, Momentum, or Floaty movement in Pause; each starts the same course.
The shotgun is equipped by default. Course / R restarts at platform 01.
The arena is 160 x 320 metres. Runway starts a clear 280-metre sprint lane.

The Rust core handles movement, collisions, bat impulses, travelling shotgun pellets,
object simulation, slow motion, and scoring. Three.js handles rendering;
browser JavaScript handles input, sound, and presentation. This is a small
prototype, not a complete Rust game engine or a multiplayer FPS.

Rust source is in arena-core/. Tested with Rust 1.99.0, no crate dependencies.
To rebuild, install Rust >= 1.85 and its wasm32-unknown-unknown target:
rustup target add wasm32-unknown-unknown
cargo test --locked --offline --manifest-path arena-core/Cargo.toml
cargo build --locked --offline --release --target wasm32-unknown-unknown --manifest-path arena-core/Cargo.toml
Copy arena-core/target/wasm32-unknown-unknown/release/arena_core.wasm to arena/core.wasm.

The included puzzle prototypes are available at index.html.
Three.js is MIT-licensed; its license is included in vendor/.
'''
destination=root/'downloads'/'impact-browser.zip';destination.parent.mkdir(exist_ok=True)
with ZipFile(destination,'w',compression=ZIP_DEFLATED,compresslevel=9) as archive:
 for name in files:
  data=(root/name).read_bytes()
  if name=='arena.html':
   data=data.replace(b'<a href="downloads/impact-browser.zip" download>Download demo \xe2\x86\x93</a>',b'')
  info=ZipInfo(name,date_time=(2026,1,1,0,0,0));info.compress_type=ZIP_DEFLATED;info.external_attr=0o644<<16;archive.writestr(info,data)
 info=ZipInfo('START-HERE.txt',date_time=(2026,1,1,0,0,0));info.compress_type=ZIP_DEFLATED;info.external_attr=0o644<<16;archive.writestr(info,instructions)
with ZipFile(destination) as archive:
 assert archive.testzip() is None
 assert archive.read('arena/core.wasm')[:4]==b'\x00asm'
print(f'Packaged {len(files)} assets/source files: {destination.stat().st_size:,} bytes.')
